#!/usr/bin/env node
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";

const root = resolve(process.argv[2] ?? process.cwd());
const registryPath = resolve(root, "config/nsq/feature_execution_registry.json");
const surfaceRegistryPath = resolve(root, "config/nsq/canonical_surface_registry.json");
const reportPath = resolve(root, "state/braxon/canonicality_audit.json");
const registry = JSON.parse(readFileSync(registryPath, "utf8"));
const surfaceRegistry = JSON.parse(readFileSync(surfaceRegistryPath, "utf8"));
const classifiedLegacySurfaces = surfaceRegistry.deprecated_historical_surfaces ?? [];
const exactLegacyClassification = new Map(
  classifiedLegacySurfaces
    .filter((surface) => !surface.path.endsWith("/"))
    .map((surface) => [surface.path, surface.classification]),
);
const prefixLegacyClassification = classifiedLegacySurfaces
  .filter((surface) => surface.path.endsWith("/"))
  .map((surface) => [surface.path, surface.classification]);
const tracked = execFileSync("git", ["-C", root, "ls-files", "-z"], {
  encoding: "buffer",
  maxBuffer: 128 * 1024 * 1024,
})
  .toString("utf8")
  .split("\0")
  .filter(Boolean);

const requiredStatuses = new Set(["canonical_active", "deprecated_historical_only"]);
const featureById = new Map(registry.features.map((feature) => [feature.id, feature]));
const idSeen = new Set();
const duplicateIds = [];
const invalidFeatures = [];
const canonicalPurposes = new Map();
const deprecatedFeatures = [];
const canonicalFeatures = [];
for (const feature of registry.features) {
  if (idSeen.has(feature.id)) duplicateIds.push(feature.id);
  idSeen.add(feature.id);
  if (!requiredStatuses.has(feature.canonicality)) {
    invalidFeatures.push(`${feature.id}:missing_or_invalid_canonicality`);
    continue;
  }
  if (feature.canonicality === "deprecated_historical_only") {
    const successor = featureById.get(feature.deprecated_replaced_by);
    if (!feature.deprecated_replaced_by || !successor || successor.canonicality !== "canonical_active") {
      invalidFeatures.push(`${feature.id}:deprecated_successor_not_canonical_active`);
    }
    deprecatedFeatures.push({
      id: feature.id,
      source: feature.source,
      classification: feature.canonicality,
      successor: feature.deprecated_replaced_by,
      purpose: feature.action_contract,
    });
    continue;
  }
  const purpose = feature.action_contract.trim();
  const existing = canonicalPurposes.get(purpose) ?? [];
  existing.push(feature.id);
  canonicalPurposes.set(purpose, existing);
  canonicalFeatures.push({
    id: feature.id,
    source: feature.source,
    classification: feature.canonicality,
    purpose,
    required_artifacts: feature.required_artifacts,
  });
}

const duplicatePurposes = [...canonicalPurposes.entries()]
  .filter(([, ids]) => ids.length > 1)
  .map(([purpose, ids]) => ({ purpose, ids }));
const legacyActiveReferences = [];
for (const feature of registry.features.filter((feature) => feature.canonicality === "canonical_active")) {
  const combined = JSON.stringify(feature);
  if (/model\.safetensors\.index\.json|authoritative_tensor_index|authoritative donor index/i.test(combined)) {
    legacyActiveReferences.push(feature.id);
  }
}

const deprecatedSourcePaths = new Set(deprecatedFeatures.map((feature) => feature.source));
const canonicalSourcePaths = new Set(canonicalFeatures.map((feature) => feature.source));
const staleCandidates = [];
const canonicalVocabularyInspectionSources = new Set([
  "tools/toolchain/audit_canonicality.mjs",
]);
const ignoredPrefixes = [
  "state/full_android_language_toolchain/src/",
  "vendor/",
  "target/",
  "assets/",
];
for (const path of tracked) {
  if (ignoredPrefixes.some((prefix) => path.startsWith(prefix))) continue;
  if (!/\.(rs|py|mjs|js|sh|json|md|toml)$/i.test(path)) continue;
  const absolute = resolve(root, path);
  if (!existsSync(absolute) || statSync(absolute).size > 1_500_000) continue;
  const text = readFileSync(absolute, "utf8");
  const staleVocabulary = /model\.safetensors\.index\.json|authoritative donor index|complete shard set|safetensors_shards/i.test(text);
  if (staleVocabulary && !deprecatedSourcePaths.has(path)) {
    const explicitClassification = exactLegacyClassification.get(path)
      ?? prefixLegacyClassification.find(([prefix]) => path.startsWith(prefix))?.[1];
    staleCandidates.push({
      path,
      classification: explicitClassification
        ?? (canonicalVocabularyInspectionSources.has(path)
          ? "canonical_audit_vocabulary_detector"
          : (canonicalSourcePaths.has(path)
            ? "canonical_active_contains_legacy_vocabulary"
            : "unclassified_legacy_candidate")),
    });
  }
}

const checks = [
  { name: "all_registered_features_are_classified", passed: invalidFeatures.length === 0, evidence: invalidFeatures },
  { name: "feature_identifiers_are_unique", passed: duplicateIds.length === 0, evidence: duplicateIds },
  { name: "canonical_feature_purposes_are_unique", passed: duplicatePurposes.length === 0, evidence: duplicatePurposes },
  { name: "active_features_do_not_require_conventional_donor_index", passed: legacyActiveReferences.length === 0, evidence: legacyActiveReferences },
  { name: "no_active_source_contains_conventional_donor_vocabulary", passed: staleCandidates.every((candidate) => candidate.classification !== "canonical_active_contains_legacy_vocabulary"), evidence: staleCandidates },
  { name: "all_legacy_vocabulary_surfaces_are_explicitly_classified", passed: staleCandidates.every((candidate) => candidate.classification !== "unclassified_legacy_candidate"), evidence: staleCandidates },
];
const valid = checks.every((check) => check.passed);
const report = {
  schema: "braxon.nsq.canonicality_audit.v1",
  authority: "NSQ kinetic semantic reflexor",
  workspace_root: root,
  generated_from: "tracked_repository_and_feature_execution_registry",
  valid,
  status: valid ? "canonicality_verified" : "canonicality_requires_reclassification",
  scope: {
    tracked_file_total: tracked.length,
    excluded_prefixes: ignoredPrefixes,
    feature_total: registry.features.length,
    canonical_feature_total: canonicalFeatures.length,
    deprecated_historical_feature_total: deprecatedFeatures.length,
    explicitly_classified_legacy_surface_total: classifiedLegacySurfaces.length,
  },
  checks,
  canonical_features: canonicalFeatures,
  deprecated_historical_features: deprecatedFeatures,
  stale_candidates: staleCandidates,
};
writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
const digest = createHash("sha256").update(JSON.stringify(report)).digest("hex");
console.log(JSON.stringify({ valid, report: relative(root, reportPath), sha256: digest, stale_candidate_total: staleCandidates.length }, null, 2));
