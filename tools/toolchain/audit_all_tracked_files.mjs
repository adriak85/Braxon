#!/usr/bin/env node
import { createHash } from "node:crypto";
import { spawn } from "node:child_process";
import { createReadStream, existsSync, lstatSync, readFileSync, readlinkSync, writeFileSync } from "node:fs";
import { relative, resolve, extname } from "node:path";
import { StringDecoder } from "node:string_decoder";

const root = resolve(process.argv[2] ?? process.cwd());
const policyPath = resolve(root, "config/nsq/canonical_surface_registry.json");
const outputPath = resolve(root, "state/braxon/full_tree_audit.json");
const blankRegistryPath = resolve(root, "config/nsq/blank_surface_registry.json");
const policy = JSON.parse(readFileSync(policyPath, "utf8"));
const blankRegistry = existsSync(blankRegistryPath)
  ? JSON.parse(readFileSync(blankRegistryPath, "utf8"))
  : { entries: [] };
const registeredBlank = new Map(
  (blankRegistry.entries ?? [])
    .filter((entry) => entry.classification !== "remove_or_materialize_required")
    .map((entry) => [entry.path, entry]),
);
const featureRegistry = JSON.parse(readFileSync(resolve(root, "config/nsq/feature_execution_registry.json"), "utf8"));

function trackedFiles() {
  return new Promise((resolveList, reject) => {
    const child = spawn("git", ["-C", root, "ls-files", "-z"], { stdio: ["ignore", "pipe", "inherit"] });
    const chunks = [];
    child.stdout.on("data", (chunk) => chunks.push(chunk));
    child.on("error", reject);
    child.on("close", (code) => {
      if (code !== 0) return reject(new Error(`git ls-files failed with ${code}`));
      resolveList(Buffer.concat(chunks).toString("utf8").split("\0").filter(Boolean));
    });
  });
}

const textExtensions = new Set([
  ".rs", ".c", ".cc", ".cpp", ".cxx", ".h", ".hpp", ".py", ".sh", ".bash", ".zsh", ".js", ".mjs", ".ts", ".tsx", ".json", ".jsonl", ".toml", ".yaml", ".yml", ".md", ".txt", ".csv", ".xml", ".html", ".css", ".scss", ".s", ".S", ".zig", ".scm", ".ss", ".lisp", ".el", ".d2", ".mmd", ".puml", ".ini", ".cfg", ".lock",
]);
const integrityPattern = /\b(?:mock(?:ed|ing)?|placeholder|not[ -]?implemented|to[ -]?do|fixme|truncated)\b|\[\.\.\.\]|\u2026\s*\[?truncated\]?/ig;

function inspectFile(path, textRequired) {
  return new Promise((resolveInspection, reject) => {
    const hash = createHash("sha256");
    const decoder = new StringDecoder("utf8");
    const stream = createReadStream(path);
    let bytes = 0;
    let nonWhitespace = false;
    let tail = "";
    const markers = new Set();
    stream.on("error", reject);
    stream.on("data", (chunk) => {
      hash.update(chunk);
      bytes += chunk.length;
      if (!nonWhitespace && /\S/.test(chunk.toString("utf8"))) nonWhitespace = true;
      if (textRequired) {
        const text = tail + decoder.write(chunk);
        for (const match of text.matchAll(integrityPattern)) markers.add(match[0].toLowerCase());
        tail = text.slice(-96);
      }
    });
    stream.on("end", () => {
      if (textRequired) {
        const text = tail + decoder.end();
        for (const match of text.matchAll(integrityPattern)) markers.add(match[0].toLowerCase());
      }
      resolveInspection({
        sha256: hash.digest("hex"),
        bytes,
        nonWhitespace: textRequired ? nonWhitespace : bytes > 0,
        integrity_markers: [...markers].sort(),
      });
    });
  });
}

const deprecated = new Map(
  (policy.deprecated_historical_surfaces ?? [])
    .filter((surface) => !surface.path.endsWith("/"))
    .map((surface) => [surface.path, surface]),
);
const historicalPrefixes = (policy.deprecated_historical_surfaces ?? [])
  .filter((surface) => surface.path.endsWith("/"))
  .map((surface) => [surface.path, surface]);
const canonicalExact = new Map();
for (const route of policy.canonical_routes ?? []) {
  for (const source of route.sources ?? []) canonicalExact.set(source, route.id);
  for (const frontDoor of route.front_doors ?? []) canonicalExact.set(frontDoor, route.id);
}
for (const feature of featureRegistry.features ?? []) {
  if (feature.canonicality === "canonical_active") canonicalExact.set(feature.source, feature.id);
}

function classify(path) {
  if (canonicalExact.has(path)) return { classification: "canonical_active", authority: canonicalExact.get(path) };
  if (path.startsWith(".codex/")) return { classification: "developer_governance_surface", authority: "repository_canonicality_policy" };
  if (path.startsWith(".hooks/")) return { classification: "security_provenance_surface", authority: "security_hook_capture_provenance" };
  if (path.startsWith(".github/") || path.startsWith(".githooks/") || path === ".gitmodules") return { classification: "continuous_integration_or_git_control_surface", authority: "repository_validation" };
  if (deprecated.has(path)) {
    const surface = deprecated.get(path);
    return { classification: surface.classification, authority: surface.successor };
  }
  const historical = historicalPrefixes.find(([prefix]) => path.startsWith(prefix));
  if (historical) return { classification: historical[1].classification, authority: historical[1].successor };
  if (path.startsWith("state/full_android_language_toolchain/src/")) return { classification: "attributed_upstream_source", authority: "source_availability_manifest" };
  if (path.startsWith("vendor/")) return { classification: "vendored_dependency_source", authority: "Cargo.lock_and_vendor_config" };
  if (path.startsWith("benchmarks/")) return { classification: "benchmark_evidence_surface", authority: "benchmark_validation" };
  if (path.startsWith("audit/") || path.startsWith("reconstruction/")) return { classification: "historical_audit_or_reconstruction_surface", authority: "canonicality_audit" };
  if (path.startsWith("assets/") || path.startsWith("models/")) return { classification: "semantic_artifact_or_source_payload_surface", authority: "semantic_corpus_or_source_manifest" };
  if (path.startsWith("apps/") || path.startsWith("nsq/") || path.startsWith("asm/") || path.startsWith("android/")) return { classification: "nsq_application_or_target_surface", authority: "NSQ_reflexor_or_target_contract" };
  if (path.startsWith("packaged/") || path.startsWith("generated/") || path.startsWith("artifacts/") || path.startsWith("reviewed_dropin_") || path.startsWith("hounds/") || path.startsWith("hooks/") || path.startsWith("prompts/") || path.startsWith(".tower/")) return { classification: "packaged_or_captured_support_surface", authority: "repository_canonicality_policy" };
  if (path.startsWith("tests/") || path.includes("/tests/")) return { classification: "executable_verification_surface", authority: "workspace_test_suite" };
  if (path.startsWith("docs/") || path.startsWith("specs/")) return { classification: "documentation_or_contract_surface", authority: "repository_canonicality_policy" };
  if (path.startsWith("config/")) return { classification: "configuration_contract_surface", authority: "NSQ_reflexor_or_toolchain_validation" };
  if (path.startsWith("state/")) return { classification: "derived_state_or_provenance_surface", authority: "state_contract_or_capture_provenance" };
  if (path.startsWith("scripts/") || path.startsWith("tools/") || path.startsWith("bin/")) return { classification: "operational_tool_surface", authority: "repository_front_door_or_validation" };
  if (path.startsWith("crates/") || path.startsWith("src/") || path.startsWith("nsq-unified/")) return { classification: "workspace_source_surface", authority: "workspace_reflexor_discovery_and_tests" };
  if (["Cargo.toml", "Cargo.lock", "LICENSE", "README.md", ".gitignore", "rust-toolchain.toml"].includes(path) || path.startsWith(".cargo/")) return { classification: "repository_control_surface", authority: "workspace_build_or_license" };
  if (!path.includes("/")) return { classification: "root_operational_or_control_surface", authority: "repository_front_door_or_bootstrap_contract" };
  return { classification: "unclassified", authority: "classification_required" };
}

const files = await trackedFiles();
const records = [];
let totalBytes = 0;
const classCounts = new Map();
const missing = [];
for (const path of files) {
  const absolute = resolve(root, path);
  let stat;
  try {
    stat = lstatSync(absolute);
  } catch {
    missing.push({ path, reason: "tracked_path_missing" });
    continue;
  }
  if (!stat.isFile()) {
    const symlink = stat.isSymbolicLink();
    const target = symlink ? readlinkSync(absolute) : null;
    const targetPresent = symlink ? existsSync(absolute) : true;
    records.push({
      path,
      kind: symlink ? "symlink" : "non_regular",
      bytes: symlink ? Buffer.byteLength(target ?? "", "utf8") : 0,
      sha256: symlink ? createHash("sha256").update(target ?? "").digest("hex") : null,
      link_target: target,
      link_target_present: targetPresent,
      classification: symlink && !targetPresent ? "unresolved_environment_link" : "repository_control_surface",
      authority: symlink ? "tracked_link_contract" : "git_tracked_non_regular_entry",
    });
    continue;
  }
  const policyRecord = classify(path);
  const textRequired = textExtensions.has(extname(path).toLowerCase()) || ["Cargo.toml", "Cargo.lock", "README.md", "LICENSE", ".gitignore"].includes(path);
  const inspection = await inspectFile(absolute, textRequired);
  if (inspection.bytes !== stat.size) throw new Error(`streamed byte count changed during audit: ${path}`);
  totalBytes += stat.size;
  classCounts.set(policyRecord.classification, (classCounts.get(policyRecord.classification) ?? 0) + 1);
  records.push({ path, kind: "regular", text_required: textRequired, ...inspection, ...policyRecord });
}
const unclassified = records.filter((record) => record.classification === "unclassified").map((record) => record.path);
const blankRequiredContent = records.filter((record) => record.kind === "regular" && record.text_required && !record.nonWhitespace).map((record) => record.path);
const unregisteredBlankRequiredContent = blankRequiredContent.filter((path) => !registeredBlank.has(path));
const integrityMarkers = records
  .filter((record) => record.kind === "regular" && record.integrity_markers?.length > 0)
  .map((record) => ({ path: record.path, markers: record.integrity_markers, classification: record.classification }));
const unresolvedEnvironmentLinks = records
  .filter((record) => record.classification === "unresolved_environment_link")
  .map((record) => ({ path: record.path, link_target: record.link_target }));
const merkle = createHash("sha256");
for (const record of records.filter((record) => record.kind === "regular").sort((a, b) => a.path.localeCompare(b.path))) {
  merkle.update(`${record.path}\0${record.sha256}\0${record.bytes}\0${record.classification}\n`);
}
const report = {
  schema: "braxon.nsq.full_tracked_tree_audit.v1",
  scope: "every recursively tracked repository path; every tracked regular file streamed in full for SHA-256 identity; no depth or extension filter",
  root,
  valid: missing.length === 0 && unclassified.length === 0 && unregisteredBlankRequiredContent.length === 0 && unresolvedEnvironmentLinks.length === 0,
  status: missing.length > 0 ? "tracked_paths_missing" : unresolvedEnvironmentLinks.length > 0 ? "unresolved_environment_links_detected" : unclassified.length > 0 ? "classification_required" : unregisteredBlankRequiredContent.length > 0 ? "unregistered_blank_required_content_detected" : "all_tracked_files_classified_with_integrity_markers_reported",
  tracked_path_total: files.length,
  regular_file_total: records.filter((record) => record.kind === "regular").length,
  total_regular_bytes: totalBytes,
  merkle_sha256: merkle.digest("hex"),
  classification_counts: Object.fromEntries([...classCounts.entries()].sort(([a], [b]) => a.localeCompare(b))),
  missing,
  unclassified,
  blank_required_content: blankRequiredContent,
  registered_blank_nonroutable_total: blankRequiredContent.length - unregisteredBlankRequiredContent.length,
  unregistered_blank_required_content: unregisteredBlankRequiredContent,
  integrity_markers: integrityMarkers,
  unresolved_environment_links: unresolvedEnvironmentLinks,
  records,
};
writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({
  valid: report.valid,
  status: report.status,
  tracked_path_total: report.tracked_path_total,
  regular_file_total: report.regular_file_total,
  total_regular_bytes: report.total_regular_bytes,
  merkle_sha256: report.merkle_sha256,
  unclassified_total: report.unclassified.length,
  blank_required_content_total: report.blank_required_content.length,
  unregistered_blank_required_content_total: report.unregistered_blank_required_content.length,
  integrity_marker_total: report.integrity_markers.length,
  unresolved_environment_link_total: report.unresolved_environment_links.length,
  output: relative(root, outputPath),
}, null, 2));
