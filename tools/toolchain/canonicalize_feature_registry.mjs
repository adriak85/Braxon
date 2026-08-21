#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(process.argv[2] ?? process.cwd());
const path = resolve(root, "config/nsq/feature_execution_registry.json");
const registry = JSON.parse(readFileSync(path, "utf8"));

const classifications = new Map([
  ["feature:operator.intelligence", ["canonical_active", ""]],
  ["feature:parameter.citadel", ["canonical_active", ""]],
  ["feature:benchmark.native_equivalence", ["canonical_active", ""]],
  ["feature:benchmark.native_recovery", ["canonical_active", ""]],
  ["feature:model.donor_readiness", ["canonical_active", ""]],
  ["feature:model.tensor_inference", ["canonical_active", ""]],
  ["feature:tensor.materialization", ["deprecated_historical_only", "feature:model.tensor_inference"]],
  ["feature:repository.operation", ["canonical_active", ""]],
  ["feature:language.parameter_parse", ["canonical_active", ""]],
  ["feature:toolchain.contained_verify", ["canonical_active", ""]],
  ["feature:watermark.file_operation", ["canonical_active", ""]],
  ["feature:toolchain.bionic_compatibility", ["canonical_active", ""]],
]);

if (registry.features.length !== classifications.size) {
  throw new Error(`registry classification table covers ${classifications.size} features but registry contains ${registry.features.length}`);
}

for (const feature of registry.features) {
  const classification = classifications.get(feature.id);
  if (!classification) throw new Error(`missing canonicality classification for ${feature.id}`);
  const [canonicality, deprecatedReplacedBy] = classification;
  feature.canonicality = canonicality;
  if (deprecatedReplacedBy) feature.deprecated_replaced_by = deprecatedReplacedBy;
  else delete feature.deprecated_replaced_by;

  if (feature.id === "feature:model.donor_readiness") {
    feature.required_artifacts = [
      "config/nsq/braxon_council_ten_stack.json",
      "crates/braxon-core/src/council_ten.rs",
      "crates/nsq-citadel/src/materialization.rs",
    ];
    feature.state_contract = "canonical_council_ten_seed_to_ten_body_nsq_materialization_fire_and_release";
    feature.action_contract = "validate_authoritative_council_ten_stack_then_materialize_fire_and_release_each_seed_body";
    feature.answer_contract = "seed_route_readiness_report_without_model_weight_or_resident_runtime_claim";
  }
  if (feature.id === "feature:model.tensor_inference") {
    feature.required_artifacts = [
      "assets/braxon_core/tokenizer/braxon_unified_tokenizer.json",
      "config/nsq/braxon_council_ten_stack.json",
      "crates/nsq-citadel/src/materialization.rs",
    ];
    feature.state_contract = "tokenized_prompt_to_selected_council_ten_citadel_seed_window";
    feature.action_contract = "execute_selected_band_seed_materialization_fire_and_release";
    feature.answer_contract = "bounded_seed_window_execution_answer_without_model_weight_claim";
  }
  if (feature.id === "feature:tensor.materialization") {
    feature.required_artifacts = ["crates/nsq-core/src/native_tensor.rs"];
    feature.state_contract = "deprecated_conventional_tensor_index_compatibility_only";
    feature.action_contract = "historical_external_tensor_format_materialization";
    feature.answer_contract = "deprecated_path_not_routable";
  }
}

registry.canonicality_policy = {
  canonical_active: "Eligible for Kinetic Semantic Reflexor discovery and route selection.",
  deprecated_historical_only: "Retained only for provenance; excluded from Kinetic Semantic Reflexor route selection and must identify its canonical successor.",
  removed: "Not represented as a feature; absence is verified by the canonicality audit.",
};
writeFileSync(path, `${JSON.stringify(registry, null, 2)}\n`);
console.log(`canonicalized ${registry.features.length} feature registry entries at ${path}`);
