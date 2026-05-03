use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct RuntimeSemanticContext {
    pub entry_terms: Vec<String>,
    pub compass_tokens: Vec<String>,
    pub group_terms: BTreeMap<String, Vec<String>>,
    pub source_kind_counts: BTreeMap<String, usize>,
    pub active_state_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSemanticEvidence {
    pub consumers_ready: bool,
    pub feed_entries: usize,
    pub compass_seed_tokens: usize,
    pub patch_anchor_count: usize,
    pub tests_present: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SemanticBias {
    pub dialect_score: i64,
    pub route_score: i64,
    pub proof_score: i64,
    pub repair_score: i64,
    pub authority_score: i64,
    pub code_score: i64,
    pub emotion_score: i64,
}

impl SemanticBias {
    pub fn total(self) -> i64 {
        self.dialect_score
            + self.route_score
            + self.proof_score
            + self.repair_score
            + self.authority_score
            + self.code_score
            + self.emotion_score
    }
}

static RUNTIME_SEMANTIC_CONTEXT: OnceLock<RuntimeSemanticContext> = OnceLock::new();
const RUNTIME_SEMANTIC_PATCH_MARKERS: [&str; 4] = [
    "BRAXON_runtime_semantic_patch::lane",
    "BRAXON_runtime_semantic_patch::execute_slice",
    "BRAXON_runtime_semantic_patch::algorithm_lever_from_semantic_text",
    "BRAXON_runtime_semantic_patch::execute_request",
];
const RUNTIME_SEMANTIC_PATCH_SOURCE: &str = include_str!("lib.rs");
const RUNTIME_SEMANTIC_PATCH_TEST_SOURCE: &str =
    include_str!("../tests/runtime_semantic_context_patch.rs");

pub fn runtime_semantic_context() -> &'static RuntimeSemanticContext {
    RUNTIME_SEMANTIC_CONTEXT.get_or_init(load_runtime_semantic_context_default)
}

pub fn load_runtime_semantic_context_default() -> RuntimeSemanticContext {
    let Some(root) = resolve_BRAXON_root() else {
        return RuntimeSemanticContext::default();
    };
    load_runtime_semantic_context_from_root(&root)
}

pub fn load_runtime_semantic_context_from_root(root: &Path) -> RuntimeSemanticContext {
    let tok_path = root.join("assets/braxon_core/tokenizer/braxon_unified_tokenizer.json");
    let Ok(raw) = fs::read_to_string(&tok_path) else {
        return RuntimeSemanticContext::default();
    };
    let Ok(obj) = serde_json::from_str::<Value>(&raw) else {
        return RuntimeSemanticContext::default();
    };

    let mut ctx = RuntimeSemanticContext::default();
    let mut entry_seen = BTreeSet::<String>::new();
    let mut token_seen = BTreeSet::<String>::new();

    if let Some(feed) = obj.get("semantic_feed").and_then(Value::as_object) {
        if let Some(map) = feed.get("source_kind_counts").and_then(Value::as_object) {
            for (k, v) in map {
                if let Some(n) = v.as_u64() {
                    ctx.source_kind_counts.insert(k.clone(), n as usize);
                }
            }
        }
        if let Some(map) = feed.get("active_state_counts").and_then(Value::as_object) {
            for (k, v) in map {
                if let Some(n) = v.as_u64() {
                    ctx.active_state_counts.insert(k.clone(), n as usize);
                }
            }
        }
        if let Some(entries) = feed.get("entries").and_then(Value::as_array) {
            for entry in entries {
                normalize_entry(entry, &mut ctx, &mut entry_seen);
            }
        }
    }

    if let Some(seed) = obj.get("compass_seed").and_then(Value::as_object) {
        if let Some(tokens) = seed.get("tokens").and_then(Value::as_array) {
            for token in tokens {
                normalize_compass_token(token, &mut ctx, &mut token_seen);
            }
        }
    }

    ctx
}

pub fn runtime_semantic_evidence(ctx: &RuntimeSemanticContext) -> RuntimeSemanticEvidence {
    let patch_anchor_count = RUNTIME_SEMANTIC_PATCH_MARKERS
        .iter()
        .filter(|marker| RUNTIME_SEMANTIC_PATCH_SOURCE.contains(**marker))
        .count();
    let tests_present = RUNTIME_SEMANTIC_PATCH_TEST_SOURCE
        .matches("#[test]")
        .count()
        >= 4
        && RUNTIME_SEMANTIC_PATCH_TEST_SOURCE
            .contains("semantic_context_loader_reads_entries_and_tokens");
    let feed_entries = ctx.entry_terms.len();
    let compass_seed_tokens = ctx.compass_tokens.len();
    let consumers_ready = feed_entries > 0
        && compass_seed_tokens > 0
        && patch_anchor_count == RUNTIME_SEMANTIC_PATCH_MARKERS.len()
        && tests_present;

    RuntimeSemanticEvidence {
        consumers_ready,
        feed_entries,
        compass_seed_tokens,
        patch_anchor_count,
        tests_present,
    }
}

pub fn load_runtime_semantic_evidence_from_root(root: &Path) -> RuntimeSemanticEvidence {
    let ctx = load_runtime_semantic_context_from_root(root);
    runtime_semantic_evidence(&ctx)
}

pub fn semantic_bias_for_text(ctx: &RuntimeSemanticContext, text: &str) -> SemanticBias {
    let lower = text.to_ascii_lowercase();
    let mut out = SemanticBias::default();

    score_from_terms(&lower, &ctx.entry_terms, &mut out);
    score_from_terms(&lower, &ctx.compass_tokens, &mut out);

    bump_direct_keywords(&lower, &mut out);

    out
}

pub fn semantic_algorithm_lever_hint(text: &str) -> Option<i64> {
    let bias = semantic_bias_for_text(runtime_semantic_context(), text);

    if bias.repair_score >= 4 || bias.proof_score >= 4 {
        return Some(960);
    }
    if bias.authority_score + bias.route_score + bias.code_score >= 7 {
        return Some(840);
    }
    if bias.emotion_score >= 4 {
        return Some(720);
    }
    None
}

pub fn semantic_runtime_lane_hint(text: &str) -> Option<&'static str> {
    let bias = semantic_bias_for_text(runtime_semantic_context(), text);

    if bias.authority_score + bias.route_score + bias.code_score >= 7 {
        return Some("offline_model_native_runtime_lane");
    }

    None
}

fn resolve_BRAXON_root() -> Option<PathBuf> {
    for key in ["ROOT", "BRAXON_ROOT"] {
        if let Ok(v) = std::env::var(key) {
            return Some(PathBuf::from(v));
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        for p in cwd.ancestors() {
            let pb = p.to_path_buf();
            if pb
                .join("assets/braxon_core/tokenizer/braxon_unified_tokenizer.json")
                .exists()
            {
                return Some(pb);
            }
        }
    }

    let home_default =
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| String::from("."))).join("Braxon");
    if home_default
        .join("assets/braxon_core/tokenizer/braxon_unified_tokenizer.json")
        .exists()
    {
        return Some(home_default);
    }

    None
}

fn normalize_entry(entry: &Value, ctx: &mut RuntimeSemanticContext, seen: &mut BTreeSet<String>) {
    match entry {
        Value::String(s) => push_entry_term(ctx, seen, s, None),
        Value::Object(map) => {
            let group = map.get("group").and_then(Value::as_str);
            let source_kind = map.get("source_kind").and_then(Value::as_str);
            let active_state = map.get("active_state").and_then(Value::as_str);

            if let Some(sk) = source_kind {
                *ctx.source_kind_counts.entry(sk.to_string()).or_insert(0) += 1;
            }
            if let Some(st) = active_state {
                *ctx.active_state_counts.entry(st.to_string()).or_insert(0) += 1;
            }

            for key in ["term", "token", "label", "surface"] {
                if let Some(s) = map.get(key).and_then(Value::as_str) {
                    push_entry_term(ctx, seen, s, group);
                }
            }
        }
        _ => {}
    }
}

fn normalize_compass_token(
    token: &Value,
    ctx: &mut RuntimeSemanticContext,
    seen: &mut BTreeSet<String>,
) {
    match token {
        Value::String(s) => push_unique(&mut ctx.compass_tokens, seen, s),
        Value::Object(map) => {
            if let Some(s) = map.get("token").and_then(Value::as_str) {
                push_unique(&mut ctx.compass_tokens, seen, s);
            }
        }
        _ => {}
    }
}

fn push_entry_term(
    ctx: &mut RuntimeSemanticContext,
    seen: &mut BTreeSet<String>,
    raw: &str,
    group: Option<&str>,
) {
    let Some(cleaned) = clean_term(raw) else {
        return;
    };

    let key = cleaned.to_ascii_lowercase();
    if seen.insert(key) {
        ctx.entry_terms.push(cleaned.clone());
    }

    if let Some(group_name) = group {
        let bucket = ctx.group_terms.entry(group_name.to_string()).or_default();
        if !bucket.iter().any(|s| s.eq_ignore_ascii_case(&cleaned)) {
            bucket.push(cleaned);
        }
    }
}

fn push_unique(vec: &mut Vec<String>, seen: &mut BTreeSet<String>, raw: &str) {
    let Some(cleaned) = clean_term(raw) else {
        return;
    };
    let key = cleaned.to_ascii_lowercase();
    if seen.insert(key) {
        vec.push(cleaned);
    }
}

fn clean_term(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

fn score_from_terms(lower: &str, terms: &[String], out: &mut SemanticBias) {
    for term in terms {
        let t = term.to_ascii_lowercase();
        if t.is_empty() || !lower.contains(&t) {
            continue;
        }

        if has_any(
            &t,
            &["dialect", "runtime lane", "lane", "route", "selection"],
        ) {
            out.dialect_score += 2;
            out.route_score += 2;
        }
        if has_any(&t, &["proof", "verify", "verification"]) {
            out.proof_score += 3;
        }
        if has_any(&t, &["repair", "triage", "dependency risk"]) {
            out.repair_score += 3;
        }
        if has_any(&t, &["authority", "native authority", "native runtime"]) {
            out.authority_score += 3;
        }
        if has_any(
            &t,
            &[
                "semantic", "code", "compile", "address", "26d", "base8", "delta",
            ],
        ) {
            out.code_score += 2;
        }
        if has_any(
            &t,
            &[
                "emotion",
                "psychological",
                "sociological",
                "perspective",
                "affect",
            ],
        ) {
            out.emotion_score += 1;
        }
    }
}

fn bump_direct_keywords(lower: &str, out: &mut SemanticBias) {
    if has_any(
        lower,
        &["repair", "repair phase", "triage", "dependency risk"],
    ) {
        out.repair_score += 4;
    }
    if has_any(
        lower,
        &["proof", "verify", "verification", "proof obligation"],
    ) {
        out.proof_score += 4;
    }
    if has_any(
        lower,
        &["runtime lane", "native runtime", "authority lane", "route"],
    ) {
        out.route_score += 3;
        out.authority_score += 3;
    }
    if has_any(
        lower,
        &[
            "semantic",
            "compile",
            "code",
            "26d",
            "base8",
            "canonical semantics",
        ],
    ) {
        out.code_score += 3;
    }
    if has_any(
        lower,
        &["emotion", "psychological", "sociological", "perspective"],
    ) {
        out.emotion_score += 2;
    }
}

fn has_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}
