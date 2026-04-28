#!/usr/bin/env node
import { readFile } from "node:fs/promises";

const SOURCE_MAP_PATH = "docs/superpowers/data/canon/first-volume-source-map.json";
const CANON_INDEX_PATH = "docs/superpowers/data/canon/first-volume-canon-index.json";
const STORY_BEATS_PATH = "docs/superpowers/data/first-volume-story-beats.json";
const SPRINT7_TARGETS_PATH = "docs/superpowers/data/sprint7-narrative-targets.json";
const REQUIRED_FACT_FIELDS = [
  "fact_id",
  "volume",
  "chapter_no",
  "chapter_title",
  "line_start",
  "line_end",
  "fact_type",
  "canon_summary",
  "game_design_use",
  "mechanic_hooks",
  "narrative_hooks",
  "canon_strict_rule",
  "sandbox_if_allowance",
  "forbidden_misreadings",
];
const FORBIDDEN_RAW_TEXT_KEYS = new Set([
  "source_excerpt",
  "excerpt",
  "original_text",
  "raw_text",
  "chapter_text",
  "long_quote",
]);
const FORBIDDEN_MOJIBAKE_CHARS = ["�", "鑺", "绗", "鍗", "闈", "铔"];

async function readJson(path) {
  return JSON.parse(await readFile(path, "utf8"));
}

function walkObject(value, path = "$") {
  if (typeof value === "string") {
    return FORBIDDEN_MOJIBAKE_CHARS.filter((marker) => value.includes(marker)).map(
      (marker) => `${path} 含疑似乱码标记 ${marker}`,
    );
  }
  if (!value || typeof value !== "object") {
    return [];
  }
  const problems = [];
  if (Array.isArray(value)) {
    value.forEach((item, index) => problems.push(...walkObject(item, `${path}[${index}]`)));
    return problems;
  }
  for (const [key, child] of Object.entries(value)) {
    if (FORBIDDEN_RAW_TEXT_KEYS.has(key)) {
      problems.push(`${path}.${key} 不允许保存原文摘录字段`);
    }
    problems.push(...walkObject(child, `${path}.${key}`));
  }
  return problems;
}

function ensure(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function collectBeatRefs(value, refs = []) {
  if (!value || typeof value !== "object") {
    return refs;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectBeatRefs(item, refs);
    }
    return refs;
  }
  if (Array.isArray(value.canon_index_refs)) {
    refs.push(...value.canon_index_refs);
  }
  for (const child of Object.values(value)) {
    collectBeatRefs(child, refs);
  }
  return refs;
}

async function main() {
  const sourceMap = await readJson(SOURCE_MAP_PATH);
  const canonIndex = await readJson(CANON_INDEX_PATH);
  const storyBeats = await readJson(STORY_BEATS_PATH);
  const sprint7Targets = await readJson(SPRINT7_TARGETS_PATH);

  ensure(sourceMap.entries?.length === 199, "第一卷 source map 必须覆盖 199 节");
  ensure(sourceMap.source?.raw_sha256, "source map 必须记录原始文件 hash");
  ensure(sourceMap.source?.encoding, "source map 必须记录编码");

  const rawTextProblems = [
    ...walkObject(sourceMap, SOURCE_MAP_PATH),
    ...walkObject(canonIndex, CANON_INDEX_PATH),
  ];
  ensure(rawTextProblems.length === 0, rawTextProblems.join("\n"));

  const factIds = new Set();
  for (const fact of canonIndex.facts ?? []) {
    for (const field of REQUIRED_FACT_FIELDS) {
      ensure(Object.hasOwn(fact, field), `事实卡 ${fact.fact_id ?? "<unknown>"} 缺字段 ${field}`);
    }
    ensure(!factIds.has(fact.fact_id), `重复事实卡 id：${fact.fact_id}`);
    factIds.add(fact.fact_id);
    const chapter = sourceMap.entries.find((entry) => entry.chapter_no === fact.chapter_no);
    ensure(chapter, `事实卡 ${fact.fact_id} 引用了不存在章节 ${fact.chapter_no}`);
    ensure(
      fact.line_start >= chapter.line_start && fact.line_end <= chapter.line_end,
      `事实卡 ${fact.fact_id} 行号不在章节范围内`,
    );
  }
  ensure(factIds.size >= 10, "首批事实卡至少应覆盖 10 个关键事实");

  const beatRefs = collectBeatRefs(storyBeats);
  ensure(beatRefs.length > 0, "剧情 beat 必须引用 canon_index_refs");
  for (const ref of beatRefs) {
    ensure(factIds.has(ref), `剧情 beat 引用了不存在的事实卡：${ref}`);
  }

  const targetRefs = collectBeatRefs(sprint7Targets);
  for (const ref of targetRefs) {
    ensure(factIds.has(ref), `Sprint 7 目标槽位引用了不存在的事实卡：${ref}`);
  }

  console.log(
    `canon index 验证通过：source map ${sourceMap.entries.length} 节，事实卡 ${factIds.size} 张，beat 引用 ${beatRefs.length} 条`,
  );
}

main().catch((error) => {
  console.error(error.message);
  process.exitCode = 1;
});
