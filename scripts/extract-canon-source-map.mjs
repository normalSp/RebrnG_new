#!/usr/bin/env node
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { TextDecoder } from "node:util";

const FIRST_VOLUME_TITLE = "第一卷 魔性不改";
const SECOND_VOLUME_TITLE = "第二卷 魔子出山";
const FIRST_VOLUME_CHAPTER_COUNT = 199;
const CHAPTER_HEADING_RE = /^第[一二三四五六七八九十百千零〇两]+节\s+.+$/;

function parseArgs(argv) {
  const args = new Map();
  for (let index = 0; index < argv.length; index += 1) {
    const token = argv[index];
    if (!token.startsWith("--")) {
      continue;
    }
    const key = token.slice(2);
    const value = argv[index + 1];
    if (!value || value.startsWith("--")) {
      args.set(key, true);
    } else {
      args.set(key, value);
      index += 1;
    }
  }
  return args;
}

export function decodeSource(buffer) {
  const candidates = ["utf-8", "gb18030", "gbk"];
  for (const encoding of candidates) {
    try {
      const text = new TextDecoder(encoding, { fatal: true }).decode(buffer);
      return { encoding, text };
    } catch {
      // Try the next common Chinese novel encoding.
    }
  }
  throw new Error("无法按 UTF-8 / GB18030 / GBK 解码原著源文件");
}

export function buildSourceMap(buffer, options = {}) {
  const localFile = options.localFile ?? "reverend-insanity.txt";
  const rawSha256 = createHash("sha256").update(buffer).digest("hex");
  const { encoding, text } = decodeSource(buffer);
  const lines = text.split(/\r?\n/);
  const firstVolumeLine = lines.findIndex((line) => line.trim() === FIRST_VOLUME_TITLE) + 1;
  const secondVolumeLine = lines.findIndex((line) => line.trim() === SECOND_VOLUME_TITLE) + 1;

  if (firstVolumeLine <= 0) {
    throw new Error(`未找到卷标：${FIRST_VOLUME_TITLE}`);
  }
  if (secondVolumeLine <= 0) {
    throw new Error(`未找到第二卷起点：${SECOND_VOLUME_TITLE}`);
  }

  const chapterMarkers = lines
    .map((line, index) => ({ line: index + 1, title: line.trim() }))
    .filter(
      (entry) =>
        entry.line > firstVolumeLine &&
        entry.line < secondVolumeLine &&
        CHAPTER_HEADING_RE.test(entry.title),
    );

  if (chapterMarkers.length !== FIRST_VOLUME_CHAPTER_COUNT) {
    throw new Error(
      `第一卷章节数应为 ${FIRST_VOLUME_CHAPTER_COUNT}，实际解析到 ${chapterMarkers.length}`,
    );
  }

  const sourceId = "ri-local-reverend-insanity-txt";
  const entries = chapterMarkers.map((chapter, index) => {
    const next = chapterMarkers[index + 1];
    return {
      source_id: sourceId,
      local_file: localFile,
      encoding,
      raw_sha256: rawSha256,
      volume: FIRST_VOLUME_TITLE,
      chapter_no: index + 1,
      chapter_title: chapter.title,
      line_start: chapter.line,
      line_end: (next?.line ?? secondVolumeLine) - 1,
    };
  });

  return {
    schema_version: "first-volume-source-map-v1",
    generated_note: "This file stores source line ranges only. It must not contain original chapter text.",
    source: {
      source_id: sourceId,
      local_file: localFile,
      encoding,
      raw_sha256: rawSha256,
      volume: FIRST_VOLUME_TITLE,
      chapter_count: entries.length,
      line_start: firstVolumeLine,
      line_end: secondVolumeLine - 1,
      next_volume_marker: SECOND_VOLUME_TITLE,
      next_volume_line: secondVolumeLine,
    },
    entries,
  };
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  const source = args.get("source");
  const output = args.get("out");
  if (!source || !output) {
    throw new Error(
      "用法：node scripts/extract-canon-source-map.mjs --source <reverend-insanity.txt> --out <first-volume-source-map.json>",
    );
  }

  const sourcePath = resolve(String(source));
  const outputPath = resolve(String(output));
  const buffer = await readFile(sourcePath);
  const sourceMap = buildSourceMap(buffer, { localFile: "reverend-insanity.txt" });
  await mkdir(dirname(outputPath), { recursive: true });
  await writeFile(outputPath, `${JSON.stringify(sourceMap, null, 2)}\n`, "utf8");

  console.log(
    `已生成第一卷 source map：${sourceMap.entries.length} 节，编码 ${sourceMap.source.encoding}，hash ${sourceMap.source.raw_sha256}`,
  );
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch((error) => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
