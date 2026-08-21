#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { relative, resolve } from "node:path";

const root = resolve(process.argv[2] ?? process.cwd());
const tree = JSON.parse(readFileSync(resolve(root, "state/braxon/full_tree_audit.json"), "utf8"));
const linkage = JSON.parse(readFileSync(resolve(root, "state/braxon/full_file_watermark_reflex_inventory.json"), "utf8"));
const outputPath = resolve(root, "config/nsq/blank_surface_registry.json");
const byPath = new Map((tree.records ?? []).map((record) => [record.path, record]));
const executableBlank = new Set(linkage.blank_executable_files ?? []);
const entries = (tree.blank_required_content ?? []).map((path) => {
  const record = byPath.get(path);
  if (!record) throw new Error(`blank path missing full-tree record: ${path}`);
  let classification;
  let successor;
  if (record.classification === "attributed_upstream_source") {
    classification = "upstream_empty_semantic_marker_nonexecutable";
    successor = "source_availability_manifest";
  } else if (record.classification === "derived_state_or_provenance_surface") {
    classification = "derived_empty_capture_nonroutable";
    successor = "state_capture_provenance";
  } else if (record.classification === "historical_archive" || record.classification === "historical_audit_or_reconstruction_surface") {
    classification = "historical_empty_capture_nonroutable";
    successor = "canonicality_audit";
  } else {
    classification = "remove_or_materialize_required";
    successor = "none";
  }
  return {
    path,
    source_classification: record.classification,
    blank_executable_by_extension: executableBlank.has(path),
    classification,
    canonical_successor: successor,
    active_reflexor_routable: false,
    content_claim: "empty file has no executable implementation and is retained only when its recorded provenance or upstream source identity requires it",
  };
});
const invalid = entries.filter((entry) => entry.classification === "remove_or_materialize_required");
const report = {
  schema: "braxon.nsq.blank_surface_registry.v1",
  scope: "every blank tracked text surface reported by the full recursive tree audit",
  tree_audit_merkle_sha256: tree.merkle_sha256,
  entry_total: entries.length,
  valid: invalid.length === 0,
  entries,
  invalid,
};
writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({ valid: report.valid, entry_total: report.entry_total, invalid_total: report.invalid.length, output: relative(root, outputPath) }, null, 2));
