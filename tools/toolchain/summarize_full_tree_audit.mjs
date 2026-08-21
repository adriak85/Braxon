#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";

const root = resolve(process.argv[2] ?? process.cwd());
const inputPath = resolve(root, "state/braxon/full_tree_audit.json");
const outputPath = resolve(root, "state/braxon/full_tree_audit_summary.json");
const audit = JSON.parse(readFileSync(inputPath, "utf8"));

function topPath(path) {
  const parts = path.split("/");
  return parts.length > 1 ? parts[0] : "<root>";
}
function countBy(values, key) {
  const counts = new Map();
  for (const value of values) {
    const bucket = key(value);
    counts.set(bucket, (counts.get(bucket) ?? 0) + 1);
  }
  return Object.fromEntries([...counts.entries()].sort(([a], [b]) => a.localeCompare(b)));
}
function samplesBy(values, key, limit = 12) {
  const buckets = new Map();
  for (const value of values) {
    const bucket = key(value);
    const entries = buckets.get(bucket) ?? [];
    if (entries.length < limit) entries.push(value.path);
    buckets.set(bucket, entries);
  }
  return Object.fromEntries([...buckets.entries()].sort(([a], [b]) => a.localeCompare(b)));
}

const records = audit.records ?? [];
const blanks = records.filter((record) => record.kind === "regular" && record.text_required && !record.nonWhitespace);
const markers = records.filter((record) => record.kind === "regular" && record.integrity_markers?.length > 0);
const unresolved = audit.unresolved_environment_links ?? [];
const summary = {
  schema: "braxon.nsq.full_tracked_tree_audit_summary.v1",
  audit_merkle_sha256: audit.merkle_sha256,
  complete_file_scope: audit.scope,
  status: audit.status,
  totals: {
    tracked_path_total: audit.tracked_path_total,
    regular_file_total: audit.regular_file_total,
    total_regular_bytes: audit.total_regular_bytes,
    unclassified_total: audit.unclassified?.length ?? 0,
    blank_required_content_total: blanks.length,
    marker_bearing_file_total: markers.length,
    unresolved_environment_link_total: unresolved.length,
  },
  unclassified: audit.unclassified ?? [],
  blank_required_content: {
    by_classification: countBy(blanks, (record) => record.classification),
    by_top_path: countBy(blanks, (record) => topPath(record.path)),
    samples_by_top_path: samplesBy(blanks, (record) => topPath(record.path)),
  },
  integrity_markers: {
    marker_frequency: countBy(markers.flatMap((record) => record.integrity_markers.map((marker) => ({ marker }))), (entry) => entry.marker),
    by_classification: countBy(markers, (record) => record.classification),
    by_top_path: countBy(markers, (record) => topPath(record.path)),
    samples_by_top_path: samplesBy(markers, (record) => topPath(record.path)),
  },
  unresolved_environment_links: unresolved,
};
writeFileSync(outputPath, `${JSON.stringify(summary, null, 2)}\n`);
console.log(JSON.stringify({
  status: summary.status,
  totals: summary.totals,
  output: relative(root, outputPath),
}, null, 2));
