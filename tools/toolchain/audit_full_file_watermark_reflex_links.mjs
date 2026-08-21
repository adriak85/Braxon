#!/usr/bin/env node
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { extname, relative, resolve } from "node:path";

const root = resolve(process.argv[2] ?? process.cwd());
const treePath = resolve(root, "state/braxon/full_tree_audit.json");
const contractPath = resolve(root, "config/nsq/full_file_watermark_reflex_contract.json");
const registryPath = resolve(root, "config/nsq/feature_execution_registry.json");
const outputPath = resolve(root, "state/braxon/full_file_watermark_reflex_inventory.json");
const blankRegistryPath = resolve(root, "config/nsq/blank_surface_registry.json");
const tree = JSON.parse(readFileSync(treePath, "utf8"));
const blankRegistry = JSON.parse(readFileSync(blankRegistryPath, "utf8"));
const registeredBlank = new Map(
  (blankRegistry.entries ?? [])
    .filter((entry) => entry.classification !== "remove_or_materialize_required")
    .map((entry) => [entry.path, entry]),
);
const contract = JSON.parse(readFileSync(contractPath, "utf8"));
const registry = JSON.parse(readFileSync(registryPath, "utf8"));

const codeExtensions = new Set([
  ".rs", ".c", ".cc", ".cpp", ".cxx", ".h", ".hpp", ".py", ".sh", ".bash", ".zsh", ".js", ".mjs", ".ts", ".tsx", ".s", ".S", ".asm", ".zig", ".scm", ".ss", ".lisp", ".el", ".m", ".mm", ".go", ".java", ".kt", ".swift", ".rb", ".php",
]);
const directFeature = new Map(
  (registry.features ?? [])
    .filter((feature) => feature.canonicality === "canonical_active")
    .map((feature) => [feature.source, feature.id]),
);
const systemReferencePattern = /(?:crates|src|config|state|assets|scripts|tools|vendor)\/[A-Za-z0-9._+@=:/-]+/g;
const markerPattern = /\b(?:mock(?:ed|ing)?|placeholder|not[ -]?implemented|to[ -]?do|fixme|truncated)\b|\[\.\.\.\]|\u2026\s*\[?truncated\]?/ig;

function isExecutable(record) {
  if (record.kind !== "regular") return false;
  const path = record.path;
  if (codeExtensions.has(extname(path))) return true;
  if (!path.includes("/") && /(?:^|_)(?:build|bootstrap|setup|rebuild|verify|validate|run|repair|resume|prime|stage|trace|probe|relink|strengthen)(?:_|$)/.test(path)) return true;
  return false;
}

function bindingFor(record) {
  if (directFeature.has(record.path)) {
    return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: directFeature.get(record.path), binding_kind: "direct_canonical_feature" };
  }
  switch (record.classification) {
    case "canonical_active":
    case "workspace_source_surface":
      return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: "nsq-reflexor.dynamic_workspace_discovery", binding_kind: "workspace_discovery" };
    case "operational_tool_surface":
    case "root_operational_or_control_surface":
    case "developer_governance_surface":
    case "continuous_integration_or_git_control_surface":
    case "security_provenance_surface":
      return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: "repository_front_door_or_validation_route", binding_kind: "operational_route" };
    case "nsq_application_or_target_surface":
      return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: "nsq_target_contract_route", binding_kind: "target_contract" };
    case "benchmark_evidence_surface":
      return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: "benchmark_validation", binding_kind: "benchmark_route" };
    case "historical_audit_or_reconstruction_surface":
    case "historical_archive":
    case "packaged_or_captured_support_surface":
      return { watermark_binding: "historical_watermark_provenance", reflexor_binding: "canonicality_audit", binding_kind: "historical_or_captured_provenance" };
    case "derived_state_or_provenance_surface":
      return { watermark_binding: "historical_watermark_provenance", reflexor_binding: "state_contract_or_capture_provenance", binding_kind: "derived_provenance" };
    case "attributed_upstream_source":
      return { watermark_binding: "functional_watermark_provenance_binding", reflexor_binding: "source_availability_manifest", binding_kind: "attributed_upstream_provenance" };
    case "vendored_dependency_source":
      return { watermark_binding: "functional_watermark_provenance_binding", reflexor_binding: "Cargo.lock_and_vendor_config", binding_kind: "vendored_dependency_provenance" };
    case "executable_verification_surface":
      return { watermark_binding: "feature:watermark.file_operation", reflexor_binding: "workspace_test_suite", binding_kind: "verification_route" };
    case "deprecated_historical_only":
      return { watermark_binding: "historical_watermark_provenance", reflexor_binding: "deprecated_non_routable_successor_required", binding_kind: "deprecated_provenance" };
    default:
      return { watermark_binding: null, reflexor_binding: null, binding_kind: "unbound" };
  }
}

function lineMerkle(path, text) {
  const normalized = text.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  if (lines.length > 0 && lines.at(-1) === "") lines.pop();
  const rootHash = createHash("sha256");
  const markers = new Set();
  const references = new Set();
  let nonBlankLines = 0;
  let codeLikeLines = 0;
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index];
    const lineHash = createHash("sha256")
      .update(path)
      .update("\0")
      .update(String(index + 1))
      .update("\0")
      .update(Buffer.from(line, "utf8"))
      .digest("hex");
    rootHash.update(lineHash).update("\n");
    if (/\S/.test(line)) nonBlankLines += 1;
    if (/\S/.test(line) && !/^\s*(?:\/\/|\/\*|\*|#|;|<!--)/.test(line)) codeLikeLines += 1;
    for (const match of line.matchAll(markerPattern)) markers.add(match[0].toLowerCase());
    for (const match of line.matchAll(systemReferencePattern)) references.add(match[0]);
  }
  return {
    line_total: lines.length,
    nonblank_line_total: nonBlankLines,
    code_like_line_total: codeLikeLines,
    line_merkle_sha256: rootHash.digest("hex"),
    integrity_markers: [...markers].sort(),
    static_cross_system_references: [...references].sort(),
  };
}

const records = [];
const unreadable = [];
for (const record of tree.records ?? []) {
  if (!isExecutable(record)) continue;
  const absolute = resolve(root, record.path);
  if (!existsSync(absolute)) {
    unreadable.push({ path: record.path, reason: "executable_source_not_present" });
    continue;
  }
  let raw;
  try {
    raw = readFileSync(absolute);
  } catch (error) {
    unreadable.push({ path: record.path, reason: `read_failed:${error.code ?? "unknown"}` });
    continue;
  }
  const binding = bindingFor(record);
  const line = lineMerkle(record.path, raw.toString("utf8"));
  records.push({
    path: record.path,
    classification: record.classification,
    source_sha256: record.sha256,
    source_bytes: record.bytes,
    ...line,
    ...binding,
  });
}
const unbound = records.filter((record) => !record.watermark_binding || !record.reflexor_binding).map((record) => record.path);
const blank = records.filter((record) => record.nonblank_line_total === 0).map((record) => record.path);
const unregisteredBlank = blank.filter((path) => !registeredBlank.has(path));
const liveMarkerCandidates = records
  .filter((record) => ["canonical_active", "workspace_source_surface", "operational_tool_surface", "nsq_application_or_target_surface", "root_operational_or_control_surface"].includes(record.classification) && record.integrity_markers.length > 0)
  .map((record) => ({ path: record.path, classification: record.classification, markers: record.integrity_markers }));
const allRoot = createHash("sha256");
for (const record of [...records].sort((left, right) => left.path.localeCompare(right.path))) {
  allRoot.update(`${record.path}\0${record.source_sha256}\0${record.line_merkle_sha256}\0${record.watermark_binding}\0${record.reflexor_binding}\n`);
}
const report = {
  schema: "braxon.nsq.full_file_watermark_reflex_inventory.v1",
  contract_schema: contract.schema,
  scope: contract.scope,
  valid: unreadable.length === 0 && unbound.length === 0 && unregisteredBlank.length === 0,
  status: unreadable.length > 0 ? "unreadable_executable_sources" : unbound.length > 0 ? "unbound_executable_sources" : unregisteredBlank.length > 0 ? "unregistered_blank_executable_sources" : "all_retained_executable_sources_have_line_merkle_watermark_and_reflex_bindings",
  tree_audit_merkle_sha256: tree.merkle_sha256,
  executable_file_total: records.length,
  line_total: records.reduce((total, record) => total + record.line_total, 0),
  code_like_line_total: records.reduce((total, record) => total + record.code_like_line_total, 0),
  inventory_merkle_sha256: allRoot.digest("hex"),
  unreadable,
  unbound,
  blank_executable_files: blank,
  registered_blank_nonroutable_total: blank.length - unregisteredBlank.length,
  unregistered_blank_executable_files: unregisteredBlank,
  live_integrity_marker_candidates: liveMarkerCandidates,
  records,
};
writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({
  valid: report.valid,
  status: report.status,
  executable_file_total: report.executable_file_total,
  line_total: report.line_total,
  code_like_line_total: report.code_like_line_total,
  unbound_total: report.unbound.length,
  blank_executable_file_total: report.blank_executable_files.length,
  unregistered_blank_executable_file_total: report.unregistered_blank_executable_files.length,
  live_integrity_marker_candidate_total: report.live_integrity_marker_candidates.length,
  output: relative(root, outputPath),
}, null, 2));
