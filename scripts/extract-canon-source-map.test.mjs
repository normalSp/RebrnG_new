import assert from "node:assert/strict";
import test from "node:test";
import { buildSourceMap, decodeSource } from "./extract-canon-source-map.mjs";

function chineseNumber(value) {
  const digits = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
  if (value < 10) {
    return digits[value];
  }
  if (value < 20) {
    return `十${value % 10 === 0 ? "" : digits[value % 10]}`;
  }
  if (value < 100) {
    return `${digits[Math.floor(value / 10)]}十${value % 10 === 0 ? "" : digits[value % 10]}`;
  }
  if (value === 100) {
    return "一百";
  }
  return `一百${value % 100 < 10 ? "零" : ""}${chineseNumber(value % 100)}`;
}

function firstVolumeFixture(chapterCount = 199) {
  const lines = ["第一卷 魔性不改"];
  for (let index = 1; index <= chapterCount; index += 1) {
    lines.push(`第${chineseNumber(index)}节 章节${index}`);
    lines.push(`第${index}节的转述测试内容。`);
  }
  lines.push("第二卷 魔子出山");
  lines.push("第一节 第二卷测试章节");
  return lines.join("\n");
}

test("decodeSource reads UTF-8 without replacement characters", () => {
  const buffer = Buffer.from(firstVolumeFixture(), "utf8");
  const decoded = decodeSource(buffer);

  assert.equal(decoded.encoding, "utf-8");
  assert.equal(decoded.text.includes("�"), false);
});

test("buildSourceMap maps exactly the first volume and stores no chapter text", () => {
  const buffer = Buffer.from(firstVolumeFixture(), "utf8");
  const sourceMap = buildSourceMap(buffer);

  assert.equal(sourceMap.entries.length, 199);
  assert.equal(sourceMap.entries[0].chapter_no, 1);
  assert.equal(sourceMap.entries.at(-1).chapter_no, 199);
  assert.equal(sourceMap.source.next_volume_marker, "第二卷 魔子出山");
  assert.equal(Object.hasOwn(sourceMap.entries[0], "text"), false);
  assert.equal(Object.hasOwn(sourceMap.entries[0], "excerpt"), false);
  assert.equal(Object.hasOwn(sourceMap.entries[0], "raw_text"), false);
});

test("buildSourceMap rejects incomplete first-volume extraction", () => {
  const buffer = Buffer.from(firstVolumeFixture(198), "utf8");

  assert.throws(() => buildSourceMap(buffer), /第一卷章节数应为 199/);
});
