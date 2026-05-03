//! BASE / CANONICAL SURFACE
//! Remove width-class truth from this surface.
//! Any host-carrier widths that remain are bugs or temporary boundary leaks.

use nsq_core::NsqSurfaceValue;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// Source-ingress compatibility forms only.
// Runtime authority belongs to native NSQ execution lanes, not these labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceIngressForm {
    Canonical,
    SExpressionIngress,
    LuaIngressShape,
    PythonIngressShape,
    RustNativeIngress,
}

impl SourceIngressForm {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceIngressForm::Canonical => "canonical",
            SourceIngressForm::SExpressionIngress => "sexpr",
            SourceIngressForm::LuaIngressShape => "lua_shape",
            SourceIngressForm::PythonIngressShape => "python_shape",
            SourceIngressForm::RustNativeIngress => "rust_native_ingress",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimeNode {
    pub kind: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimeEdge {
    pub left: String,
    pub rel: String,
    pub right: String,
    pub layer: NsqSurfaceValue,
    pub plane: NsqSurfaceValue,
    pub anchor: NsqSurfaceValue,
    pub weight: NsqSurfaceValue,
    pub flags: NsqSurfaceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimeState {
    pub target: String,
    pub state: String,
    pub flux: NsqSurfaceValue,
    pub gate: NsqSurfaceValue,
    pub phase: NsqSurfaceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimeRepresentation {
    #[serde(alias = "dialect")]
    pub source_form: String,
    pub spine_lines: Vec<String>,
    pub nodes: Vec<PrimeNode>,
    pub edges: Vec<PrimeEdge>,
    pub states: Vec<PrimeState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SanitizedIngress {
    pub source_form: String,
    pub input_line_count: usize,
    pub sanitized_line_count: usize,
    pub stripped_line_count: usize,
    pub aligned_lines: Vec<String>,
}

pub fn split_tokens_preserving_quotes(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if let Some(q) = quote {
            if ch == '\\' {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            } else if ch == q {
                quote = None;
            } else {
                cur.push(ch);
            }
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some(ch),
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(cur.clone());
                    cur.clear();
                }
            }
            _ => cur.push(ch),
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }

    out
}

pub fn strip_outer_parens(s: &str) -> &str {
    let t = s.trim();
    if t.starts_with('(') && t.ends_with(')') && t.len() >= 2 {
        &t[1..t.len() - 1]
    } else {
        t
    }
}

pub fn detect_source_ingress_form(text: &str) -> SourceIngressForm {
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let lower = line.to_ascii_lowercase();

        if lower == "@source_form sexpr"
            || lower == "!source_form sexpr"
            || lower == "source_form sexpr"
            || lower == "@dialect sexpr"
            || lower == "!dialect sexpr"
            || lower == "dialect sexpr"
        {
            return SourceIngressForm::SExpressionIngress;
        }
        if lower == "@source_form lua_shape"
            || lower == "!source_form lua_shape"
            || lower == "source_form lua_shape"
            || lower == "@dialect lua_shape"
            || lower == "!dialect lua_shape"
            || lower == "dialect lua_shape"
        {
            return SourceIngressForm::LuaIngressShape;
        }
        if lower == "@source_form python_shape"
            || lower == "!source_form python_shape"
            || lower == "source_form python_shape"
            || lower == "@dialect python_shape"
            || lower == "!dialect python_shape"
            || lower == "dialect python_shape"
        {
            return SourceIngressForm::PythonIngressShape;
        }
        if lower == "@source_form rust_native_ingress"
            || lower == "!source_form rust_native_ingress"
            || lower == "source_form rust_native_ingress"
            || lower == "@dialect rust_native_ingress"
            || lower == "!dialect rust_native_ingress"
            || lower == "dialect rust_native_ingress"
        {
            return SourceIngressForm::RustNativeIngress;
        }

        if line.starts_with('(') && line.ends_with(')') {
            return SourceIngressForm::SExpressionIngress;
        }

        if line.starts_with("rust ") || line.starts_with("cargo ") || line.starts_with("crate ") {
            return SourceIngressForm::RustNativeIngress;
        }

        if line.contains('=')
            && (line.starts_with("noise ")
                || line.starts_with("triple ")
                || line.starts_with("membrane "))
        {
            if line.contains('(') && line.ends_with(')') {
                return SourceIngressForm::PythonIngressShape;
            }
            return SourceIngressForm::LuaIngressShape;
        }

        if line.starts_with("noise ")
            || line.starts_with("triple ")
            || line.starts_with("membrane ")
        {
            return SourceIngressForm::Canonical;
        }
    }

    SourceIngressForm::Canonical
}

pub fn spine_sexpr_line(line: &str) -> Option<String> {
    let inner = strip_outer_parens(line);
    let toks = split_tokens_preserving_quotes(inner);
    if toks.is_empty() {
        return None;
    }

    match toks[0].as_str() {
        "noise" => {
            if toks.len() < 2 {
                return None;
            }
            let mut out = vec!["noise".to_string(), toks[1].clone()];
            let mut i = 2usize;
            while i + 1 < toks.len() {
                out.push(toks[i].clone());
                out.push(toks[i + 1].clone());
                i += 2;
            }
            Some(out.join(" "))
        }
        "triple" => {
            if toks.len() < 4 {
                return None;
            }
            let mut out = vec![
                "triple".to_string(),
                toks[1].clone(),
                "->".to_string(),
                toks[2].clone(),
                "->".to_string(),
                toks[3].clone(),
            ];
            let mut i = 4usize;
            while i + 1 < toks.len() {
                out.push(toks[i].clone());
                out.push(toks[i + 1].clone());
                i += 2;
            }
            Some(out.join(" "))
        }
        "membrane" => {
            if toks.len() < 2 {
                return None;
            }
            let mut out = vec!["membrane".to_string(), toks[1].clone()];
            let mut i = 2usize;
            while i + 1 < toks.len() {
                out.push(toks[i].clone());
                out.push(toks[i + 1].clone());
                i += 2;
            }
            Some(out.join(" "))
        }
        _ => None,
    }
}

pub fn spine_kv_shape_line(line: &str) -> Option<String> {
    let normalized = line.replace(['(', ')', ','], " ");
    let toks = split_tokens_preserving_quotes(&normalized);

    if toks.is_empty() {
        return None;
    }

    let head = toks[0].as_str();
    let mut positional: Vec<String> = Vec::new();
    let mut kv = BTreeMap::<String, String>::new();

    for tok in toks.iter().skip(1) {
        if let Some(eq) = tok.find('=') {
            let k = tok[..eq].trim().trim_end_matches(':').to_string();
            let v = tok[eq + 1..].trim().to_string();
            kv.insert(k, v);
        } else {
            positional.push(tok.clone());
        }
    }

    match head {
        "noise" => {
            let symbol = positional.first()?.clone();
            let mut out = vec!["noise".to_string(), symbol];
            if let Some(v) = kv.get("macro") {
                out.push(":macro".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("a") {
                out.push(":a".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("b") {
                out.push(":b".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("pos") {
                out.push(":pos".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("amp") {
                out.push(":amp".into());
                out.push(v.clone());
            }
            Some(out.join(" "))
        }
        "triple" => {
            let subject = positional
                .first()
                .cloned()
                .or_else(|| kv.get("subject").cloned())?;
            let relation = kv
                .get("rel")
                .cloned()
                .or_else(|| kv.get("relation").cloned())
                .or_else(|| positional.get(1).cloned())?;
            let object = kv
                .get("obj")
                .cloned()
                .or_else(|| kv.get("object").cloned())
                .or_else(|| positional.get(2).cloned())?;
            let mut out = vec![
                "triple".to_string(),
                subject,
                "->".to_string(),
                relation,
                "->".to_string(),
                object,
            ];
            if let Some(v) = kv.get("layer") {
                out.push(":layer".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("plane") {
                out.push(":plane".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("anchor") {
                out.push(":anchor".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("weight") {
                out.push(":weight".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("flags") {
                out.push(":flags".into());
                out.push(v.clone());
            }
            Some(out.join(" "))
        }
        "membrane" => {
            let cell = positional
                .first()
                .cloned()
                .or_else(|| kv.get("cell").cloned())?;
            let mut out = vec!["membrane".to_string(), cell];
            if let Some(v) = kv.get("state") {
                out.push(":state".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("flux") {
                out.push(":flux".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("gate") {
                out.push(":gate".into());
                out.push(v.clone());
            }
            if let Some(v) = kv.get("phase") {
                out.push(":phase".into());
                out.push(v.clone());
            }
            Some(out.join(" "))
        }
        _ => None,
    }
}

pub fn spine_rust_native_ingress_line(line: &str) -> Option<String> {
    let toks = split_tokens_preserving_quotes(line);
    if toks.is_empty() {
        return None;
    }

    match toks[0].as_str() {
        "rust" => {
            let mut crate_name = "workspace".to_string();
            let mut target = "release".to_string();
            let mut command = "build".to_string();

            let mut i = 1usize;
            while i + 1 < toks.len() {
                match toks[i].as_str() {
                    "crate" | ":crate" => crate_name = toks[i + 1].clone(),
                    "target" | ":target" => target = toks[i + 1].clone(),
                    "cmd" | ":cmd" => command = toks[i + 1].clone(),
                    _ => {}
                }
                i += 2;
            }

            Some(format!(
                "triple rust.crate.{} -> invokes -> rust.cmd.{} :layer 1 :plane 1 :anchor 900 :weight 50 :flags 1\nmembrane rust.target.{} :state sealed :flux 1 :gate 1 :phase 1",
                crate_name, command, target
            ))
        }
        "cargo" => {
            let command = toks.get(1).cloned().unwrap_or_else(|| "build".to_string());
            Some(format!(
                "triple rust.cargo -> invokes -> rust.cmd.{} :layer 1 :plane 1 :anchor 910 :weight 50 :flags 1",
                command
            ))
        }
        "crate" => {
            let name = toks
                .get(1)
                .cloned()
                .unwrap_or_else(|| "workspace".to_string());
            Some(format!(
                "triple rust.crate.{} -> family -> rust_substrate :layer 1 :plane 1 :anchor 920 :weight 50 :flags 1",
                name
            ))
        }
        _ => None,
    }
}

pub fn spine_source(text: &str) -> (SourceIngressForm, Vec<String>) {
    let source_form = detect_source_ingress_form(text);
    let mut out = Vec::new();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let lower = line.to_ascii_lowercase();
        if lower.starts_with("@source_form ")
            || lower.starts_with("!source_form ")
            || lower.starts_with("source_form ")
            || lower.starts_with("@dialect ")
            || lower.starts_with("!dialect ")
            || lower.starts_with("dialect ")
        {
            continue;
        }

        let condensed = match source_form {
            SourceIngressForm::Canonical => Some(line.to_string()),
            SourceIngressForm::SExpressionIngress => spine_sexpr_line(line),
            SourceIngressForm::LuaIngressShape | SourceIngressForm::PythonIngressShape => {
                spine_kv_shape_line(line)
            }
            SourceIngressForm::RustNativeIngress => spine_rust_native_ingress_line(line),
        };

        if let Some(v) = condensed {
            for inner in v.lines() {
                let inner = inner.trim();
                if !inner.is_empty() {
                    out.push(inner.to_string());
                }
            }
        }
    }

    let mut dedup = BTreeSet::new();
    let mut normalized = Vec::new();
    for line in out {
        if dedup.insert(line.clone()) {
            normalized.push(line);
        }
    }

    (source_form, normalized)
}

pub fn sanitize_source_ingress(text: &str) -> SanitizedIngress {
    let input_line_count = text.lines().count();
    let (source_form, aligned_lines) = spine_source(text);
    let sanitized_line_count = aligned_lines.len();
    let stripped_line_count = input_line_count.saturating_sub(sanitized_line_count);

    SanitizedIngress {
        source_form: source_form.as_str().to_string(),
        input_line_count,
        sanitized_line_count,
        stripped_line_count,
        aligned_lines,
    }
}

fn canonical_surface_value(s: &str) -> NsqSurfaceValue {
    NsqSurfaceValue::new(s).unwrap_or_else(|_| NsqSurfaceValue::zero())
}

pub fn build_prime_representation(text: &str) -> PrimeRepresentation {
    let (source_form, spine_lines) = spine_source(text);
    let mut nodes: BTreeMap<String, PrimeNode> = BTreeMap::new();
    let mut edges_map = BTreeMap::<String, PrimeEdge>::new();
    let mut states_map = BTreeMap::<String, PrimeState>::new();

    for line in &spine_lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }

        match toks[0] {
            "triple" => {
                if toks.len() < 6 {
                    continue;
                }
                let left = toks[1].to_string();
                let rel = toks[3].to_string();
                let right = toks[5].to_string();

                nodes.entry(left.clone()).or_insert(PrimeNode {
                    kind: "symbol".into(),
                    name: left.clone(),
                });
                nodes.entry(rel.clone()).or_insert(PrimeNode {
                    kind: "macro".into(),
                    name: rel.clone(),
                });
                nodes.entry(right.clone()).or_insert(PrimeNode {
                    kind: "symbol".into(),
                    name: right.clone(),
                });

                let mut layer = NsqSurfaceValue::zero();
                let mut plane = NsqSurfaceValue::zero();
                let mut anchor = NsqSurfaceValue::zero();
                let mut weight = NsqSurfaceValue::zero();
                let mut flags = NsqSurfaceValue::zero();

                let mut i = 6usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":layer" => layer = canonical_surface_value(toks[i + 1]),
                        ":plane" => plane = canonical_surface_value(toks[i + 1]),
                        ":anchor" => anchor = canonical_surface_value(toks[i + 1]),
                        ":weight" => weight = canonical_surface_value(toks[i + 1]),
                        ":flags" => flags = canonical_surface_value(toks[i + 1]),
                        _ => {}
                    }
                    i += 2;
                }

                let edge = PrimeEdge {
                    left,
                    rel,
                    right,
                    layer,
                    plane,
                    anchor,
                    weight,
                    flags,
                };
                let key = format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    edge.left,
                    edge.rel,
                    edge.right,
                    edge.layer.as_text(),
                    edge.plane.as_text(),
                    edge.anchor.as_text(),
                    edge.weight.as_text(),
                    edge.flags.as_text()
                );
                edges_map.insert(key, edge);
            }
            "membrane" => {
                if toks.len() < 2 {
                    continue;
                }
                let target = toks[1].to_string();
                nodes.entry(target.clone()).or_insert(PrimeNode {
                    kind: "symbol".into(),
                    name: target.clone(),
                });

                let mut state = "<?>".to_string();
                let mut flux = NsqSurfaceValue::zero();
                let mut gate = NsqSurfaceValue::zero();
                let mut phase = NsqSurfaceValue::zero();

                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":state" => state = toks[i + 1].to_string(),
                        ":flux" => flux = canonical_surface_value(toks[i + 1]),
                        ":gate" => gate = canonical_surface_value(toks[i + 1]),
                        ":phase" => phase = canonical_surface_value(toks[i + 1]),
                        _ => {}
                    }
                    i += 2;
                }

                let st = PrimeState {
                    target,
                    state,
                    flux,
                    gate,
                    phase,
                };
                let key = format!(
                    "{}|{}|{}|{}|{}",
                    st.target,
                    st.state,
                    st.flux.as_text(),
                    st.gate.as_text(),
                    st.phase.as_text()
                );
                states_map.insert(key, st);
            }
            _ => {}
        }
    }

    PrimeRepresentation {
        source_form: source_form.as_str().to_string(),
        spine_lines,
        nodes: nodes.into_values().collect(),
        edges: edges_map.into_values().collect(),
        states: states_map.into_values().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_native_ingress_detects_without_hook_alias() {
        assert_eq!(
            detect_source_ingress_form("@source_form rust_native_ingress\nrust crate nsq-runtime"),
            SourceIngressForm::RustNativeIngress
        );
    }

    #[test]
    fn rust_native_ingress_spine_preserves_rust_spine_terms() {
        let (source_form, spine_lines) =
            spine_source("@source_form rust_native_ingress\ncargo build");
        assert_eq!(source_form, SourceIngressForm::RustNativeIngress);
        assert!(spine_lines
            .iter()
            .any(|line| line.contains("triple rust.cargo -> invokes -> rust.cmd.build")));
    }

    #[test]
    fn sanitize_source_ingress_strips_marker_and_comment_lines() {
        let sanitized = sanitize_source_ingress(
            "# comment\n@source_form rust_native_ingress\ncargo build\n\n# ignored\n",
        );
        assert_eq!(sanitized.source_form, "rust_native_ingress");
        assert_eq!(sanitized.input_line_count, 5);
        assert_eq!(sanitized.sanitized_line_count, 1);
        assert_eq!(sanitized.aligned_lines.len(), 1);
        assert!(
            sanitized.aligned_lines[0].contains("triple rust.cargo -> invokes -> rust.cmd.build")
        );
    }

    #[test]
    fn sanitize_source_ingress_accepts_legacy_dialect_marker_as_ingress_only() {
        let sanitized = sanitize_source_ingress("@dialect python_shape\nnoise alpha macro=beta");
        assert_eq!(sanitized.source_form, "python_shape");
        assert_eq!(
            sanitized.aligned_lines,
            vec!["noise alpha :macro beta".to_string()]
        );
    }
}

#[allow(dead_code)]
pub mod native_wiring;
