import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

import {
  buildMockCandidates,
  loadTargetsFromFile,
  normalizeCandidateFromTarget,
  scanRuntimeRedline,
  validateCandidate,
  validateCandidateDirectory,
  validateTarget,
  writeCandidates,
} from "./deepseek-candidates.mjs";

async function tempDir(name) {
  const dir = path.join(tmpdir(), `rebrng-${name}-${Date.now()}-${Math.random().toString(16).slice(2)}`);
  await mkdir(dir, { recursive: true });
  return dir;
}

test("mock generation creates only needs_review moonlight-gu candidates", () => {
  const candidates = buildMockCandidates();

  assert.ok(candidates.length >= 6);
  assert.ok(candidates.some((candidate) => candidate.target_content_id.includes("claim_moonlight_gu")));
  assert.ok(candidates.some((candidate) => candidate.target_content_id.includes("refine_moonlight_gu")));
  assert.ok(candidates.some((candidate) => candidate.target_content_id.includes("cultivate_moonlight")));

  for (const candidate of candidates) {
    assert.equal(candidate.review_status, "needs_review");
    assert.doesNotThrow(() => validateCandidate(candidate));
    assert.equal(Object.hasOwn(candidate, "prompt"), false);
    assert.equal(Object.hasOwn(candidate, "response"), false);
    assert.equal(Object.hasOwn(candidate, "model"), false);
    assert.equal(Object.hasOwn(candidate, "api_key"), false);
    assert.equal(Object.hasOwn(candidate, "reasoning_content"), false);
  }
});

test("candidate validation rejects approved status and unsafe archived fields", () => {
  const [candidate] = buildMockCandidates();

  assert.throws(
    () => validateCandidate({ ...candidate, review_status: "approved" }),
    /review_status must be needs_review/,
  );
  assert.throws(
    () => validateCandidate({ ...candidate, prompt: "full prompt must not be saved" }),
    /forbidden field: prompt/,
  );
  assert.throws(
    () => validateCandidate({ ...candidate, model: "deepseek-v4-pro" }),
    /forbidden field: model/,
  );
});

test("network candidate normalization pins target metadata from the requested slot", () => {
  const [target] = buildMockCandidates().map((candidate) => ({
    target_content_id: candidate.target_content_id,
    target_slot: candidate.target_slot,
    mode: candidate.mode,
    evidence: candidate.evidence,
    state_assumptions: candidate.state_assumptions,
    risk_notes: candidate.risk_notes,
    canon_index_refs: ["moonlight_gu_refinement_resistance"],
  }));
  const candidate = normalizeCandidateFromTarget(
    {
      candidate_text: "候选正文只用于离线审校。",
    },
    target,
  );

  assert.equal(candidate.candidate_id, "s0_action_claim_moonlight_gu_action_result_v001");
  assert.equal(candidate.target_content_id, target.target_content_id);
  assert.equal(candidate.target_slot, target.target_slot);
  assert.equal(candidate.mode, target.mode);
  assert.equal(candidate.evidence, target.evidence);
  assert.deepEqual(candidate.canon_index_refs, ["moonlight_gu_refinement_resistance"]);
  assert.equal(candidate.review_status, "needs_review");
  assert.doesNotThrow(() => validateCandidate(candidate));
});

test("writeCandidates writes JSON candidates that validate from a local directory", async () => {
  const dir = await tempDir("deepseek-candidates");
  try {
    const candidates = buildMockCandidates().slice(0, 2);
    const written = await writeCandidates(candidates, dir);

    assert.equal(written.length, 2);
    assert.ok(written.every((file) => file.startsWith(dir)));
    const raw = JSON.parse(await readFile(written[0], "utf8"));
    assert.equal(raw.review_status, "needs_review");

    const result = await validateCandidateDirectory(dir);
    assert.equal(result.valid, 2);
    assert.equal(result.invalid, 0);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test("writeCandidates uses target slots for stable filenames instead of model candidate ids", async () => {
  const dir = await tempDir("deepseek-stable-filenames");
  try {
    const [first, second] = buildMockCandidates().slice(0, 2);
    const written = await writeCandidates(
      [
        { ...first, candidate_id: "model_returned_duplicate_id" },
        { ...second, candidate_id: "model_returned_duplicate_id" },
      ],
      dir,
    );

    assert.equal(written.length, 2);
    assert.equal(new Set(written).size, 2);
    assert.ok(written[0].endsWith(`${first.target_content_id}_${first.target_slot}.json`.replace(/[^a-zA-Z0-9_.-]+/g, "_")));
    assert.ok(written[1].endsWith(`${second.target_content_id}_${second.target_slot}.json`.replace(/[^a-zA-Z0-9_.-]+/g, "_")));
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test("target files drive mock candidates without adding unsafe archived fields", async () => {
  const dir = await tempDir("deepseek-targets");
  try {
    const targetFile = path.join(dir, "targets.json");
    await writeFile(
      targetFile,
      JSON.stringify(
        {
          targets: [
            {
              target_content_id: "s0.action.scout.merit_notice",
              target_slot: "action_result",
              mode: "canon_strict",
              evidence: "canon_inferred",
              candidate_text: "功绩告示不是白送的机会，它也把审计视线写进账本。",
              state_assumptions: ["玩家位于功绩告示处", "本行动只写文本候选，不改功绩规则"],
              risk_notes: ["不得生成额外奖励", "不得跳过 AP 或窗口代价"],
            },
          ],
        },
        null,
        2,
      ),
      "utf8",
    );

    const targets = await loadTargetsFromFile(targetFile);
    const candidates = buildMockCandidates(targets);

    assert.equal(candidates.length, 1);
    assert.equal(candidates[0].target_content_id, "s0.action.scout.merit_notice");
    assert.equal(candidates[0].target_slot, "action_result");
    assert.equal(candidates[0].candidate_text, "功绩告示不是白送的机会，它也把审计视线写进账本。");
    assert.equal(candidates[0].review_status, "needs_review");
    assert.equal(Object.hasOwn(candidates[0], "prompt"), false);
    assert.doesNotThrow(() => validateCandidate(candidates[0]));
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test("target files can carry canon fact references into candidates", async () => {
  const dir = await tempDir("deepseek-canon-targets");
  try {
    const targetFile = path.join(dir, "targets.json");
    await writeFile(
      targetFile,
      JSON.stringify(
        {
          targets: [
            {
              target_content_id: "s0.setup.opening_rite.afterglow",
              target_slot: "setup_dialogue",
              mode: "canon_strict",
              evidence: "canon_inferred",
              candidate_text: "开窍大典的候选文本只用于离线审校。",
              state_assumptions: ["玩家已经经历开窍大典"],
              risk_notes: ["不得跳过开窍"],
              canon_index_refs: ["opening_rite_cave_walk", "hope_gu_enters_body"],
            },
          ],
        },
        null,
        2,
      ),
      "utf8",
    );

    const targets = await loadTargetsFromFile(targetFile);
    const candidates = buildMockCandidates(targets);

    assert.deepEqual(candidates[0].canon_index_refs, ["opening_rite_cave_walk", "hope_gu_enters_body"]);
    assert.doesNotThrow(() => validateCandidate(candidates[0]));
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test("target validation rejects malformed canon index references", () => {
  assert.throws(
    () =>
      validateTarget({
        target_content_id: "s0.action.bad",
        target_slot: "action_result",
        mode: "canon_strict",
        evidence: "canon_inferred",
        state_assumptions: [],
        risk_notes: [],
        canon_index_refs: ["opening_rite_cave_walk", 7],
      }),
    /canon_index_refs must be an array of strings/,
  );
});

test("directory validation rejects malformed candidate files", async () => {
  const dir = await tempDir("deepseek-invalid");
  try {
    await writeFile(
      path.join(dir, "bad.json"),
      JSON.stringify({ candidate_id: "bad", review_status: "approved" }),
      "utf8",
    );

    await assert.rejects(() => validateCandidateDirectory(dir), /target_content_id must be a non-empty string/);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test("runtime redline scan rejects DeepSeek/API markers in runtime paths", async () => {
  const dir = await tempDir("deepseek-redline");
  try {
    const runtimeDir = path.join(dir, "crates", "game-core");
    await mkdir(runtimeDir, { recursive: true });
    await writeFile(path.join(runtimeDir, "bad.rs"), 'const API: &str = "api.deepseek.com";', "utf8");

    await assert.rejects(() => scanRuntimeRedline([runtimeDir]), /runtime redline violation/);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});
