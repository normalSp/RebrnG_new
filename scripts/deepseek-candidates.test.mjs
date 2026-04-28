import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

import {
  buildMockCandidates,
  scanRuntimeRedline,
  validateCandidate,
  validateCandidateDirectory,
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
