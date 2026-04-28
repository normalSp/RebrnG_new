#!/usr/bin/env node
import { mkdir, readFile, readdir, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const DEFAULT_OUT_DIR = ".local/deepseek-candidates";
const DEFAULT_BASE_URL = "https://api.deepseek.com";
const DEFAULT_MODEL = "deepseek-v4-pro";

const ALLOWED_CANDIDATE_FIELDS = new Set([
  "candidate_id",
  "target_content_id",
  "target_slot",
  "mode",
  "evidence",
  "candidate_text",
  "state_assumptions",
  "risk_notes",
  "review_status",
  "review_notes",
]);

const ALLOWED_REVIEW_FIELDS = new Set([
  "candidate_id",
  "overall",
  "hard_conflicts",
  "soft_drifts",
  "overpowered_flags",
  "modern_tone_flags",
  "evidence_gaps",
  "runtime_leaks",
  "copyright_risks",
  "required_edits",
  "review_notes",
]);

const REQUIRED_CANDIDATE_STRINGS = [
  "candidate_id",
  "target_content_id",
  "target_slot",
  "mode",
  "evidence",
  "candidate_text",
  "review_status",
];

const CANDIDATE_ARRAY_FIELDS = ["state_assumptions", "risk_notes"];
const ALLOWED_MODES = new Set(["canon_strict", "sandbox_if"]);
const ALLOWED_EVIDENCE = new Set(["canon_explicit", "canon_inferred", "project_inferred", "sandbox_if"]);
const ALLOWED_REVIEW_OVERALL = new Set(["pass", "pass_with_notes", "needs_revision", "fail"]);
const TARGET_ARRAY_FIELDS = ["state_assumptions", "risk_notes"];

const RUNTIME_REDLINE_MARKERS = [
  "DEEPSEEK_API_KEY",
  "DEEPSEEK_BASE_URL",
  "api.deepseek.com",
  "chat.completions",
  "deepseek-v4",
  "runtime proposal",
  "runtime narrator",
  "AI proposal",
  "AI narrator",
];

const RUNTIME_SCAN_PATHS = ["crates/game-core", "apps/desktop", "packages/ui-ledger", "content/s0"];

const DEFAULT_TARGETS = [
  {
    target_content_id: "s0.action.claim_moonlight_gu",
    target_slot: "action_result",
    evidence: "canon_inferred",
    candidate_text:
      "你按学堂名册领取月光蛊。它还不是你的力量，只是被记入你名下的一只未炼化蛊虫；能不能驱使，仍要看空窍里下一步归属。",
    state_assumptions: ["空窍已开", "学堂前院可领取月光蛊", "领取不消耗 AP 或元石"],
    risk_notes: ["不得写成额外奖励", "不得暗示月光蛊已经炼化", "不得写成本命蛊"],
  },
  {
    target_content_id: "s0.action.claim_moonlight_gu",
    target_slot: "ledger_feedback",
    evidence: "canon_inferred",
    candidate_text:
      "账上多了一只月光蛊，也多了一条未完成的归属。持有不等于可用，学堂不会替你省下炼化的窗口。",
    state_assumptions: ["玩家已领取未炼化月光蛊"],
    risk_notes: ["不得跳过 refine_moonlight_gu", "不得改变元石或 AP 结算"],
  },
  {
    target_content_id: "s0.action.refine_moonlight_gu",
    target_slot: "action_result",
    evidence: "canon_inferred",
    candidate_text:
      "你把一段窗口压进空窍，顺着月光蛊的寒意一点点磨去外来的抗拒。它终于能随你的真元牵动，却也从随身之物变成空窍里的负担。",
    state_assumptions: ["玩家持有未炼化月光蛊", "炼化消耗 1 AP", "炼化不消耗元石"],
    risk_notes: ["不得给额外修为", "不得取消后续喂养压力", "不得写成本命蛊建立"],
  },
  {
    target_content_id: "s0.action.refine_moonlight_gu",
    target_slot: "blocked_no_aperture",
    evidence: "project_inferred",
    candidate_text:
      "空窍未开，炼化无处落脚。蛊虫可以拿在手里，却不能凭空成为你的驱使之物。",
    state_assumptions: ["空窍未开时尝试炼化"],
    risk_notes: ["不得绕过开窍大典", "不得把持有写成控制"],
  },
  {
    target_content_id: "s0.action.cultivate_moonlight",
    target_slot: "action_result",
    evidence: "canon_inferred",
    candidate_text:
      "你扣下一枚元石，催动空窍里已归属的月光蛊。月华没有替你省下代价，只在一次次牵引里留下更清楚的修行痕迹。",
    state_assumptions: ["空窍已开", "月光蛊已炼化", "修行消耗 1 AP 与 1 元石"],
    risk_notes: ["不得生成新蛊虫", "不得生成杀招", "不得额外奖励资源"],
  },
  {
    target_content_id: "s0.action.cultivate_moonlight",
    target_slot: "blocked_unrefined",
    evidence: "project_inferred",
    candidate_text:
      "月光蛊尚未真正归你驱使。没有炼化归属，修行只会变成空窍里的牵扯，不能稳定推进月光痕迹。",
    state_assumptions: ["玩家没有已炼化月光蛊"],
    risk_notes: ["不得把领取等同炼化", "不得跳过炼化行动"],
  },
  {
    target_content_id: "s0.action.cultivate_moonlight",
    target_slot: "blocked_no_primeval_stone",
    evidence: "project_inferred",
    candidate_text:
      "元石见底，月光蛊再听话也不能替你凭空补足消耗。学堂给的是门槛，不是宽免。",
    state_assumptions: ["月光蛊已炼化", "玩家元石不足"],
    risk_notes: ["不得取消元石压力", "不得把蛊虫写成免费修行来源"],
  },
  {
    target_content_id: "s0.action.inspect_gu",
    target_slot: "moonlight_gu_summary",
    evidence: "canon_inferred",
    candidate_text:
      "月光蛊，一转，月道用途。账上要同时看它的位置、炼化归属、损伤和喂养压力；少看一项，后面就会在代价里补回来。",
    state_assumptions: ["玩家正在检查月光蛊状态"],
    risk_notes: ["不得把检查写成修复", "不得把核心蛊等同本命蛊"],
  },
  {
    target_content_id: "s0.action.inspect_gu",
    target_slot: "feeding_stable",
    evidence: "project_inferred",
    candidate_text:
      "喂养压力暂时还稳。稳不代表没有负担，只是这笔账还没有立刻追到你的窗口上。",
    state_assumptions: ["月光蛊喂养状态稳定"],
    risk_notes: ["不得展开复杂周期公式", "不得生成额外消耗"],
  },
  {
    target_content_id: "s0.action.inspect_gu",
    target_slot: "feeding_warning",
    evidence: "project_inferred",
    candidate_text:
      "喂养压力已经开始露头。它不会立刻替你判死，却会让后续资源、恢复和路线选择变得更窄。",
    state_assumptions: ["月光蛊喂养状态出现压力"],
    risk_notes: ["不得写成已开启周期扣费", "不得直接摧毁蛊虫"],
  },
];

function candidateIdFor(target) {
  return `${target.target_content_id}.${target.target_slot}`.replace(/[^a-zA-Z0-9]+/g, "_").replace(/^_|_$/g, "") + "_v001";
}

function candidateFileName(candidate) {
  return `${candidate.target_content_id}_${candidate.target_slot}`.replace(/[^a-zA-Z0-9_.-]+/g, "_").replace(/^_|_$/g, "") + ".json";
}

export function buildMockCandidates(targets = DEFAULT_TARGETS) {
  return targets.map((target) => ({
    candidate_id: candidateIdFor(target),
    target_content_id: target.target_content_id,
    target_slot: target.target_slot,
    mode: target.mode ?? "canon_strict",
    evidence: target.evidence,
    candidate_text:
      target.candidate_text ??
      `离线 mock 候选：${target.target_content_id} / ${target.target_slot}。此文本只用于校验流程，不能直接入库。`,
    state_assumptions: target.state_assumptions,
    risk_notes: target.risk_notes,
    review_status: "needs_review",
    review_notes: "",
  }));
}

export function validateTarget(target) {
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    throw new Error("target must be an object");
  }
  for (const field of ["target_content_id", "target_slot", "mode", "evidence"]) {
    if (typeof target[field] !== "string" || target[field].trim() === "") {
      throw new Error(`${field} must be a non-empty string`);
    }
  }
  if (!ALLOWED_MODES.has(target.mode)) {
    throw new Error("target mode must be canon_strict or sandbox_if");
  }
  if (!ALLOWED_EVIDENCE.has(target.evidence)) {
    throw new Error("target evidence is not allowed");
  }
  if (target.candidate_text !== undefined && (typeof target.candidate_text !== "string" || target.candidate_text.trim() === "")) {
    throw new Error("candidate_text must be a non-empty string when provided");
  }
  for (const field of TARGET_ARRAY_FIELDS) {
    if (!Array.isArray(target[field]) || target[field].some((value) => typeof value !== "string")) {
      throw new Error(`${field} must be an array of strings`);
    }
  }
  return true;
}

export function normalizeCandidateFromTarget(candidate, target) {
  if (!candidate || typeof candidate !== "object" || Array.isArray(candidate)) {
    throw new Error("candidate must be an object");
  }
  validateTarget(target);
  return {
    ...candidate,
    candidate_id:
      typeof candidate.candidate_id === "string" && candidate.candidate_id.trim() !== ""
        ? candidate.candidate_id
        : candidateIdFor(target),
    target_content_id: target.target_content_id,
    target_slot: target.target_slot,
    mode: target.mode,
    evidence: target.evidence,
    state_assumptions: Array.isArray(candidate.state_assumptions) ? candidate.state_assumptions : target.state_assumptions,
    risk_notes: Array.isArray(candidate.risk_notes) ? candidate.risk_notes : target.risk_notes,
    review_status: "needs_review",
    review_notes: typeof candidate.review_notes === "string" ? candidate.review_notes : "",
  };
}

export async function loadTargetsFromFile(file) {
  const raw = JSON.parse(await readFile(file, "utf8"));
  const targets = Array.isArray(raw) ? raw : raw?.targets;
  if (!Array.isArray(targets)) {
    throw new Error("targets file must contain an array or a targets array");
  }
  for (const target of targets) {
    validateTarget(target);
  }
  return targets;
}

export function validateCandidate(candidate) {
  if (!candidate || typeof candidate !== "object" || Array.isArray(candidate)) {
    throw new Error("candidate must be an object");
  }

  for (const field of Object.keys(candidate)) {
    if (!ALLOWED_CANDIDATE_FIELDS.has(field)) {
      throw new Error(`forbidden field: ${field}`);
    }
  }

  for (const field of REQUIRED_CANDIDATE_STRINGS) {
    if (typeof candidate[field] !== "string" || candidate[field].trim() === "") {
      throw new Error(`${field} must be a non-empty string`);
    }
  }

  if (!ALLOWED_MODES.has(candidate.mode)) {
    throw new Error("mode must be canon_strict or sandbox_if");
  }
  if (!ALLOWED_EVIDENCE.has(candidate.evidence)) {
    throw new Error("evidence is not allowed");
  }
  if (candidate.review_status !== "needs_review") {
    throw new Error("review_status must be needs_review");
  }
  if (typeof candidate.review_notes !== "string") {
    throw new Error("review_notes must be a string");
  }

  for (const field of CANDIDATE_ARRAY_FIELDS) {
    if (!Array.isArray(candidate[field]) || candidate[field].some((value) => typeof value !== "string")) {
      throw new Error(`${field} must be an array of strings`);
    }
  }

  return true;
}

export function validateReview(review) {
  if (!review || typeof review !== "object" || Array.isArray(review)) {
    throw new Error("review must be an object");
  }
  for (const field of Object.keys(review)) {
    if (!ALLOWED_REVIEW_FIELDS.has(field)) {
      throw new Error(`forbidden review field: ${field}`);
    }
  }
  if (typeof review.candidate_id !== "string" || review.candidate_id.trim() === "") {
    throw new Error("candidate_id must be a non-empty string");
  }
  if (!ALLOWED_REVIEW_OVERALL.has(review.overall)) {
    throw new Error("overall is not allowed");
  }
  for (const field of [
    "hard_conflicts",
    "soft_drifts",
    "overpowered_flags",
    "modern_tone_flags",
    "evidence_gaps",
    "runtime_leaks",
    "copyright_risks",
    "required_edits",
  ]) {
    if (!Array.isArray(review[field]) || review[field].some((value) => typeof value !== "string")) {
      throw new Error(`${field} must be an array of strings`);
    }
  }
  if (typeof review.review_notes !== "string") {
    throw new Error("review_notes must be a string");
  }
  return true;
}

export async function writeCandidates(candidates, outDir = DEFAULT_OUT_DIR) {
  await mkdir(outDir, { recursive: true });
  const written = [];
  for (const candidate of candidates) {
    validateCandidate(candidate);
    const file = path.join(outDir, candidateFileName(candidate));
    await writeFile(file, `${JSON.stringify(candidate, null, 2)}\n`, "utf8");
    written.push(file);
  }
  return written;
}

export async function validateCandidateDirectory(dir = DEFAULT_OUT_DIR) {
  const files = await listJsonFiles(dir);
  let valid = 0;
  let invalid = 0;
  for (const file of files) {
    const json = JSON.parse(await readFile(file, "utf8"));
    if (file.endsWith(".review.json")) {
      validateReview(json);
    } else {
      validateCandidate(json);
    }
    valid += 1;
  }
  return { valid, invalid };
}

async function listJsonFiles(dir) {
  const found = [];
  const entries = await readdir(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      found.push(...(await listJsonFiles(fullPath)));
    } else if (entry.isFile() && entry.name.endsWith(".json")) {
      found.push(fullPath);
    }
  }
  return found.sort();
}

export async function scanRuntimeRedline(paths = RUNTIME_SCAN_PATHS) {
  const hits = [];
  for (const scanPath of paths) {
    if (!(await exists(scanPath))) {
      continue;
    }
    await scanPathForMarkers(scanPath, hits);
  }
  if (hits.length) {
    throw new Error(`runtime redline violation:\n${hits.join("\n")}`);
  }
  return { scanned: paths.length, hits: 0 };
}

async function scanPathForMarkers(scanPath, hits) {
  const info = await stat(scanPath);
  if (info.isDirectory()) {
    const entries = await readdir(scanPath, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.name === "target" || entry.name === "node_modules" || entry.name === "dist") {
        continue;
      }
      await scanPathForMarkers(path.join(scanPath, entry.name), hits);
    }
    return;
  }
  if (!info.isFile() || !isTextFile(scanPath)) {
    return;
  }
  const text = await readFile(scanPath, "utf8");
  for (const marker of RUNTIME_REDLINE_MARKERS) {
    if (text.includes(marker)) {
      hits.push(`${scanPath}: ${marker}`);
    }
  }
}

function isTextFile(file) {
  return [".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".yaml", ".yml", ".toml", ".md"].includes(path.extname(file));
}

async function exists(file) {
  try {
    await stat(file);
    return true;
  } catch {
    return false;
  }
}

async function generateCommand(options) {
  const outDir = options.out ?? DEFAULT_OUT_DIR;
  const targets = options.targets ? await loadTargetsFromFile(options.targets) : DEFAULT_TARGETS;
  const allowNetwork = Boolean(options["allow-network"]);
  const mock = Boolean(options.mock) || !allowNetwork || !process.env.DEEPSEEK_API_KEY;

  if (mock) {
    const candidates = buildMockCandidates(targets);
    const written = await writeCandidates(candidates, outDir);
    return { mode: "mock", written };
  }

  const candidates = [];
  for (const target of targets) {
    candidates.push(await requestDeepSeekCandidate(target));
  }
  const written = await writeCandidates(candidates, outDir);
  return { mode: "network", written };
}

async function requestDeepSeekCandidate(target) {
  const baseUrl = process.env.DEEPSEEK_BASE_URL || DEFAULT_BASE_URL;
  const model = process.env.DEEPSEEK_MODEL || DEFAULT_MODEL;
  const response = await fetch(`${baseUrl.replace(/\/$/, "")}/chat/completions`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${process.env.DEEPSEEK_API_KEY}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model,
      response_format: { type: "json_object" },
      messages: [
        {
          role: "system",
          content:
            "你是 RebrnG 离线候选文本生成助手。只输出一个 JSON 对象，字段必须限定为 candidate_id、target_content_id、target_slot、mode、evidence、candidate_text、state_assumptions、risk_notes、review_status、review_notes。review_status 必须是 needs_review。不要输出 prompt、response、model、api_key、reasoning_content 或 thinking_chain。",
        },
        {
          role: "user",
          content: JSON.stringify(target),
        },
      ],
    }),
  });
  if (!response.ok) {
    throw new Error(`DeepSeek request failed with HTTP ${response.status}`);
  }
  const body = await response.json();
  const content = body?.choices?.[0]?.message?.content;
  if (typeof content !== "string") {
    throw new Error("DeepSeek response missing JSON content");
  }
  const parsed = JSON.parse(content);
  const candidate = normalizeCandidateFromTarget(
    parsed?.candidate && typeof parsed.candidate === "object" ? parsed.candidate : parsed,
    target,
  );
  validateCandidate(candidate);
  return candidate;
}

async function reviewCommand(options) {
  const dir = options.dir ?? DEFAULT_OUT_DIR;
  const files = (await listJsonFiles(dir)).filter((file) => !file.endsWith(".review.json"));
  const written = [];
  for (const file of files) {
    const candidate = JSON.parse(await readFile(file, "utf8"));
    validateCandidate(candidate);
    const review = {
      candidate_id: candidate.candidate_id,
      overall: "pass_with_notes",
      hard_conflicts: [],
      soft_drifts: [],
      overpowered_flags: [],
      modern_tone_flags: [],
      evidence_gaps: [],
      runtime_leaks: [],
      copyright_risks: [],
      required_edits: ["人工审校后才允许手写入 YAML。"],
      review_notes: "mock review only; no runtime AI or automatic content import.",
    };
    validateReview(review);
    const reviewFile = file.replace(/\.json$/, ".review.json");
    await writeFile(reviewFile, `${JSON.stringify(review, null, 2)}\n`, "utf8");
    written.push(reviewFile);
  }
  return { written };
}

function parseArgs(argv) {
  const [command, ...rest] = argv;
  const options = {};
  for (let index = 0; index < rest.length; index += 1) {
    const arg = rest[index];
    if (!arg.startsWith("--")) {
      continue;
    }
    const key = arg.slice(2);
    const next = rest[index + 1];
    if (!next || next.startsWith("--")) {
      options[key] = true;
    } else {
      options[key] = next;
      index += 1;
    }
  }
  return { command, options };
}

async function main(argv = process.argv.slice(2)) {
  const { command, options } = parseArgs(argv);
  if (command === "generate") {
    const result = await generateCommand(options);
    console.log(JSON.stringify(result, null, 2));
    return;
  }
  if (command === "review") {
    const result = await reviewCommand(options);
    console.log(JSON.stringify(result, null, 2));
    return;
  }
  if (command === "validate") {
    const result = await validateCandidateDirectory(options.dir ?? DEFAULT_OUT_DIR);
    console.log(JSON.stringify(result, null, 2));
    return;
  }
  if (command === "redline") {
    const result = await scanRuntimeRedline();
    console.log(JSON.stringify(result, null, 2));
    return;
  }
  throw new Error(
    "usage: deepseek-candidates.mjs generate|review|validate|redline [--mock] [--allow-network] [--targets FILE] [--out DIR] [--dir DIR]",
  );
}

const currentFile = fileURLToPath(import.meta.url);
if (process.argv[1] && path.resolve(process.argv[1]) === currentFile) {
  main().catch((error) => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
