use crate::{
    execute_canonical_parameter_citadel_cycle, ParameterCitadelInvariants, TokenizerBridge,
    TokenizerBridgeReceipt,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const LANGUAGE_OPERATION_SCHEMA: &str = "braxon.nsq.language_operation.v1";
const LANGUAGE_MATRIX_RELATIVE_PATH: &str = "config/nsq/language_functional_ingestion_matrix.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalToolResolution {
    pub declared_requirement: String,
    pub concrete_executable_name: Option<String>,
    pub resolved_path: Option<String>,
    pub ready: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LexicalParseSummary {
    pub character_count: usize,
    pub alphabetic_count: usize,
    pub numeric_count: usize,
    pub whitespace_count: usize,
    pub punctuation_count: usize,
    pub symbol_count: usize,
    pub identifier_run_count: usize,
    pub lexical_contract: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParameterGuidedParseReceipt {
    pub role: String,
    pub input_signal: i64,
    pub lexical_context: i64,
    pub generation: u64,
    pub invariants: ParameterCitadelInvariants,
    pub model_weight_execution_claimed: bool,
    pub no_resident_runtime: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LanguageOperationReport {
    pub schema: String,
    pub language_id: String,
    pub family: String,
    pub target_environment: String,
    pub nsq_capability: String,
    pub kinetic_reflexor_route: String,
    pub tokenizer_receipt: TokenizerBridgeReceipt,
    pub lexical_parse: LexicalParseSummary,
    pub parameter_guided_parse: ParameterGuidedParseReceipt,
    pub local_tools: Vec<LocalToolResolution>,
    pub target_execution_environment_matches: bool,
    pub semantic_parse_ready: bool,
    pub native_materialization_ready: bool,
    pub no_hidden_download_allowed: bool,
    pub no_resident_runtime: bool,
    pub exact_materialization_guidance: String,
}

#[derive(Debug, Deserialize)]
struct LanguageMatrix {
    schema: String,
    language_total: usize,
    #[serde(default)]
    languages: Vec<LanguageRecord>,
}

#[derive(Debug, Deserialize)]
struct LanguageRecord {
    id: String,
    family: String,
    target_environment: String,
    semantic_contract: SemanticContract,
    target_materialization: TargetMaterialization,
}

#[derive(Debug, Deserialize)]
struct SemanticContract {
    kinetic_reflexor_route: String,
    nsq_capability: String,
    resident_runtime: bool,
    semantic_operation_state: String,
}

#[derive(Debug, Deserialize)]
struct TargetMaterialization {
    #[serde(default)]
    required_local_tools: Vec<String>,
    hidden_download_allowed: bool,
}

pub fn execute_language_operation(
    start: impl AsRef<Path>,
    language_id: &str,
    input: &str,
) -> Result<LanguageOperationReport, String> {
    if language_id.trim().is_empty() {
        return Err("language operation requires a nonempty language ID".to_string());
    }
    if input.is_empty() {
        return Err("language operation requires nonempty source or artifact input".to_string());
    }
    let root = resolve_root(start)?;
    let matrix: LanguageMatrix = read_json(&root.join(LANGUAGE_MATRIX_RELATIVE_PATH))?;
    if matrix.schema != "braxon.language_functional_ingestion_matrix.v1" {
        return Err("unsupported language functional-ingestion matrix schema".to_string());
    }
    if matrix.language_total != matrix.languages.len() {
        return Err(
            "language functional-ingestion matrix count does not match records".to_string(),
        );
    }
    let record = matrix
        .languages
        .into_iter()
        .find(|record| record.id == language_id)
        .ok_or_else(|| {
            format!("language '{language_id}' is not declared in the functional-ingestion matrix")
        })?;
    if record.target_environment != "aarch64-linux-android"
        || record.semantic_contract.nsq_capability != format!("language:{}", record.id)
        || record
            .semantic_contract
            .kinetic_reflexor_route
            .trim()
            .is_empty()
        || record.semantic_contract.semantic_operation_state != "operable_on_demand"
        || record.semantic_contract.resident_runtime
        || record.target_materialization.hidden_download_allowed
    {
        return Err(format!(
            "language '{}' has an invalid or non-on-demand NSQ materialization contract",
            record.id
        ));
    }

    let tokenizer = TokenizerBridge::from_root(&root, "braxon_native")?;
    let tokenizer_receipt = tokenizer.encode_translate_round_trip(input);
    let lexical_parse = lexical_parse(input)?;
    let input_signal = i64::try_from(lexical_parse.character_count)
        .map_err(|_| "language input exceeds bounded parameter signal range".to_string())?;
    let lexical_context = lexical_context(&lexical_parse)?;
    let parameter_operation =
        execute_canonical_parameter_citadel_cycle(input_signal, lexical_context)?;
    let parameter_guided_parse = ParameterGuidedParseReceipt {
        role: parameter_operation.role,
        input_signal,
        lexical_context,
        generation: parameter_operation.generation,
        invariants: parameter_operation.invariants,
        model_weight_execution_claimed: false,
        no_resident_runtime: true,
    };

    let local_tools = record
        .target_materialization
        .required_local_tools
        .iter()
        .map(|requirement| resolve_local_tool(requirement))
        .collect::<Vec<_>>();
    let target_execution_environment_matches =
        cfg!(all(target_arch = "aarch64", target_os = "android"));
    let semantic_parse_ready = tokenizer_receipt.all_required_mappings_resolved()
        && parameter_guided_parse.invariants.all_pass()
        && !parameter_guided_parse.model_weight_execution_claimed
        && parameter_guided_parse.no_resident_runtime;
    let tools_ready = !local_tools.is_empty() && local_tools.iter().all(|tool| tool.ready);
    let native_materialization_ready = semantic_parse_ready
        && tools_ready
        && target_execution_environment_matches
        && !record.target_materialization.hidden_download_allowed;
    let missing = local_tools
        .iter()
        .filter(|tool| !tool.ready)
        .map(|tool| tool.declared_requirement.clone())
        .collect::<Vec<_>>();
    let exact_materialization_guidance = if native_materialization_ready {
        format!(
            "'{}' has a resolved local tool set on the declared AArch64 Android target; run its target-specific compile, link, or interpreter probe before promotion.",
            record.id
        )
    } else if !target_execution_environment_matches {
        format!(
            "Semantic parsing completed, but native materialization is not promoted from this host. Re-run on aarch64-linux-android after resolving: {}.",
            if missing.is_empty() {
                "target-local compiler/runtime probe".to_string()
            } else {
                missing.join(", ")
            }
        )
    } else {
        format!(
            "Install or materialize only the declared local tools for '{}': {}. Hidden downloads are forbidden.",
            record.id,
            missing.join(", ")
        )
    };
    Ok(LanguageOperationReport {
        schema: LANGUAGE_OPERATION_SCHEMA.to_string(),
        language_id: record.id,
        family: record.family,
        target_environment: record.target_environment,
        nsq_capability: record.semantic_contract.nsq_capability,
        kinetic_reflexor_route: record.semantic_contract.kinetic_reflexor_route,
        tokenizer_receipt,
        lexical_parse,
        parameter_guided_parse,
        local_tools,
        target_execution_environment_matches,
        semantic_parse_ready,
        native_materialization_ready,
        no_hidden_download_allowed: !record.target_materialization.hidden_download_allowed,
        no_resident_runtime: !record.semantic_contract.resident_runtime,
        exact_materialization_guidance,
    })
}

fn resolve_root(start: impl AsRef<Path>) -> Result<PathBuf, String> {
    let canonical = start
        .as_ref()
        .canonicalize()
        .map_err(|error| format!("unable to resolve language operation start: {error}"))?;
    canonical
        .ancestors()
        .find(|candidate| candidate.join(LANGUAGE_MATRIX_RELATIVE_PATH).is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            "unable to locate repository functional language-ingestion matrix".to_string()
        })
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("failed to read '{}': {error}", path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|error| format!("failed to parse '{}': {error}", path.display()))
}

fn lexical_parse(input: &str) -> Result<LexicalParseSummary, String> {
    let mut summary = LexicalParseSummary {
        character_count: 0,
        alphabetic_count: 0,
        numeric_count: 0,
        whitespace_count: 0,
        punctuation_count: 0,
        symbol_count: 0,
        identifier_run_count: 0,
        lexical_contract: "bounded_character_lexical_semantic_parse".to_string(),
    };
    let mut in_identifier = false;
    for character in input.chars() {
        summary.character_count = summary
            .character_count
            .checked_add(1)
            .ok_or("language input character count overflow")?;
        let identifier_character = character.is_alphanumeric() || character == '_';
        if identifier_character && !in_identifier {
            summary.identifier_run_count = summary
                .identifier_run_count
                .checked_add(1)
                .ok_or("language input identifier count overflow")?;
        }
        in_identifier = identifier_character;
        if character.is_alphabetic() {
            summary.alphabetic_count += 1;
        } else if character.is_numeric() {
            summary.numeric_count += 1;
        } else if character.is_whitespace() {
            summary.whitespace_count += 1;
        } else if character.is_ascii_punctuation() {
            summary.punctuation_count += 1;
        } else {
            summary.symbol_count += 1;
        }
    }
    Ok(summary)
}

fn lexical_context(summary: &LexicalParseSummary) -> Result<i64, String> {
    let weighted = summary
        .alphabetic_count
        .checked_mul(3)
        .and_then(|value| value.checked_add(summary.numeric_count.checked_mul(5)?))
        .and_then(|value| value.checked_add(summary.whitespace_count.checked_mul(2)?))
        .and_then(|value| value.checked_add(summary.punctuation_count.checked_mul(7)?))
        .and_then(|value| value.checked_add(summary.symbol_count.checked_mul(11)?))
        .and_then(|value| value.checked_add(summary.identifier_run_count.checked_mul(13)?))
        .ok_or("language lexical context overflow")?;
    i64::try_from(weighted)
        .map_err(|_| "language lexical context exceeds bounded parameter range".to_string())
}

fn resolve_local_tool(requirement: &str) -> LocalToolResolution {
    let concrete_executable_name = concrete_executable_name(requirement);
    let resolved_path = concrete_executable_name
        .as_deref()
        .and_then(find_executable_in_path);
    let ready = resolved_path.is_some();
    let reason = match (&concrete_executable_name, &resolved_path) {
        (Some(executable), Some(path)) => {
            format!("declared requirement resolves to '{executable}' at {path}")
        }
        (Some(executable), None) => format!(
            "declared requirement resolves to executable '{executable}', but it is not on PATH"
        ),
        (None, None) => "declared requirement has no concrete executable binding; attach a local materializer/probe before native promotion".to_string(),
        (None, Some(_)) => unreachable!("a path cannot resolve without a concrete executable"),
    };
    LocalToolResolution {
        declared_requirement: requirement.to_string(),
        concrete_executable_name,
        resolved_path,
        ready,
        reason,
    }
}

fn concrete_executable_name(requirement: &str) -> Option<String> {
    let normalized = requirement.trim().to_ascii_lowercase();
    let mapping = [
        ("clang++", "clang++"),
        ("clang", "clang"),
        ("lld", "ld.lld"),
        ("llvm-ar", "llvm-ar"),
        ("llvm-ranlib", "llvm-ranlib"),
        ("llvm-objdump", "llvm-objdump"),
        ("llvm-readelf", "llvm-readelf"),
        ("llvm-nm", "llvm-nm"),
        ("llvm-strip", "llvm-strip"),
        ("rustc", "rustc"),
        ("rust", "rustc"),
        ("cargo", "cargo"),
        ("rustfmt", "rustfmt"),
        ("clippy", "clippy-driver"),
        ("python", "python3"),
        ("guile", "guile"),
        ("zig", "zig"),
        ("cmake", "cmake"),
        ("ninja", "ninja"),
        ("make", "make"),
        ("perl", "perl"),
        ("ruby", "ruby"),
        ("lua", "lua"),
        ("java", "java"),
        ("kotlinc", "kotlinc"),
        ("go", "go"),
        ("swift", "swiftc"),
        ("ghc", "ghc"),
        ("ocaml", "ocamlc"),
        ("erlang", "erl"),
        ("elixir", "elixir"),
        ("sbcl", "sbcl"),
        ("gnat", "gnatmake"),
        ("gfortran", "gfortran"),
        ("wasm", "wasm-ld"),
        ("adb", "adb"),
    ];
    mapping.iter().find_map(|(needle, executable)| {
        normalized
            .contains(needle)
            .then(|| (*executable).to_string())
    })
}

fn find_executable_in_path(executable: &str) -> Option<String> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|directory| directory.join(executable))
            .find(|candidate| candidate.is_file())
            .map(|candidate| candidate.display().to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root")
    }

    #[test]
    fn declared_language_enters_tokenizer_and_parameter_parse_without_model_weights() {
        let report = execute_language_operation(repository_root(), "rust", "fn main() { 42 };")
            .expect("Rust language operation");
        assert!(report.semantic_parse_ready, "{report:#?}");
        assert_eq!(report.nsq_capability, "language:rust");
        assert!(report.parameter_guided_parse.invariants.all_pass());
        assert!(!report.parameter_guided_parse.model_weight_execution_claimed);
        assert!(report.no_hidden_download_allowed);
        assert!(report.no_resident_runtime);
    }

    #[test]
    fn every_declared_language_has_a_bounded_semantic_parse_route() {
        let root = repository_root();
        let matrix: LanguageMatrix = read_json(&root.join(LANGUAGE_MATRIX_RELATIVE_PATH)).unwrap();
        assert_eq!(matrix.language_total, matrix.languages.len());
        for language in matrix.languages {
            let report = execute_language_operation(&root, &language.id, "x").unwrap();
            assert!(report.semantic_parse_ready, "{}: {report:#?}", language.id);
            assert!(report.parameter_guided_parse.invariants.all_pass());
            assert!(
                !report.native_materialization_ready || report.target_execution_environment_matches
            );
        }
    }

    #[test]
    fn unknown_language_fails_closed() {
        let error = execute_language_operation(repository_root(), "not_a_declared_language", "x")
            .expect_err("unknown language must fail");
        assert!(error.contains("not declared"));
    }
}
