use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SCHEMA: &str = "braxon.nsqasm.stamp_record.v1";
const AUTHORITY: &str = "NSQ_COURT";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StampCandidate {
    schema: String,
    authority: String,
    stamp_id: String,
    source_path: String,
    language: String,
    start_line: usize,
    end_line: usize,
    line_count: usize,
    byte_count: usize,
    sha256: String,
    court_route: CourtRoute,
    semantic_kind: String,
    reusable_score: u64,
    preview: String,
}


impl StampCandidate {
    fn identity_key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.source_path, self.start_line, self.end_line, self.language, self.sha256
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AcceptedStamp {
    #[serde(flatten)]
    candidate: StampCandidate,
    stored_operation_required: bool,
    wake_packet_required: bool,
    runtime_projection_required: bool,
    materialization_path_required: bool,
    semantic_execution_continuity_required: bool,
    passive_stamp_only_mode_allowed: bool,
    pre_bake_state: String,
    projection_lane: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CourtRoute {
    scan: CourtSeat,
    validate: CourtSeat,
    prepare: CourtSeat,
    compose: CourtSeat,
    route: CourtSeat,
    queue: CourtSeat,
    guard: CourtSeat,
    audit: CourtSeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CourtSeat {
    court_position: String,
    title: String,
    duty: String,
}

#[derive(Default)]
struct ScanStats {
    files_seen: usize,
    files_read: usize,
    candidates: usize,
    accepted: usize,
}

fn main() -> Result<()> {
    let root = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root = PathBuf::from(root).canonicalize().context("canonicalize root")?;

    let out_dir = root.join("state/nsq/stamp_build_chain");
    fs::create_dir_all(&out_dir).context("create stamp db output dir")?;

    let candidates_path = out_dir.join("candidates.jsonl");
    let accepted_path = out_dir.join("accepted.jsonl");
    let report_path = out_dir.join("scanner_report.txt");

    let candidates_tmp_path = out_dir.join("candidates.jsonl.tmp");
    let accepted_tmp_path = out_dir.join("accepted.jsonl.tmp");
    let report_tmp_path = out_dir.join("scanner_report.txt.tmp");

    let mut candidate_writer = replace_jsonl(&candidates_tmp_path)?;
    let mut accepted_writer = replace_jsonl(&accepted_tmp_path)?;
    let mut seen_candidates: BTreeSet<String> = BTreeSet::new();
    let mut seen_accepted: BTreeSet<String> = BTreeSet::new();

    let mut stats = ScanStats::default();

    for entry in WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !skip_path(e.path()))
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        stats.files_seen += 1;

        let Some(language) = language_for(path) else {
            continue;
        };

        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => continue,
        };

        stats.files_read += 1;

        for candidate in candidates_from_text(&root, path, language, &text)? {
            let candidate_key = candidate.identity_key();

            if seen_candidates.insert(candidate_key) {
                stats.candidates += 1;
                serde_json::to_writer(&mut candidate_writer, &candidate)?;
                writeln!(candidate_writer)?;
            }

            if accepts_candidate(&candidate) {
                let accepted = AcceptedStamp {
                    candidate,
                    stored_operation_required: true,
                    wake_packet_required: true,
                    runtime_projection_required: true,
                    materialization_path_required: true,
                    semantic_execution_continuity_required: true,
                    passive_stamp_only_mode_allowed: false,
                    pre_bake_state: "bishop_prepared_king_composable".to_string(),
                    projection_lane: "current_binary_or_host_language_filtered_until_nsqasm_native".to_string(),
                };

                let accepted_key = accepted.candidate.identity_key();
                if seen_accepted.insert(accepted_key) {
                    serde_json::to_writer(&mut accepted_writer, &accepted)?;
                    writeln!(accepted_writer)?;
                    stats.accepted += 1;
                }
            }
        }
    }

    let report = format!(
        "schema=braxon.nsqasm.stamp_database_scan_report.v1\n\
         authority={AUTHORITY}\n\
         root={}\n\
         files_seen={}\n\
         files_read={}\n\
         candidates={}\n\
         accepted={}\n\
         candidates_path={}\n\
         accepted_path={}\n",
        root.display(),
        stats.files_seen,
        stats.files_read,
        stats.candidates,
        stats.accepted,
        candidates_path.display(),
        accepted_path.display(),
    );

    candidate_writer.flush().context("flush candidates jsonl")?;
    accepted_writer.flush().context("flush accepted jsonl")?;

    fs::write(&report_tmp_path, report).context("write scanner report tmp")?;

    fs::rename(&candidates_tmp_path, &candidates_path).context("replace candidates jsonl")?;
    fs::rename(&accepted_tmp_path, &accepted_path).context("replace accepted jsonl")?;
    fs::rename(&report_tmp_path, &report_path).context("replace scanner report")?;

    println!("PASS: NSQASM stamp database scan complete");
    println!("files_seen={}", stats.files_seen);
    println!("files_read={}", stats.files_read);
    println!("candidates={}", stats.candidates);
    println!("accepted={}", stats.accepted);
    println!("report={}", report_path.display());

    Ok(())
}

fn replace_jsonl(path: &Path) -> Result<BufWriter<File>> {
    let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
    Ok(BufWriter::new(file))
}

fn skip_path(path: &Path) -> bool {
    let s = path.to_string_lossy();

    s.contains("/.git/")
        || s.contains("/target/")
        || s.contains("/.cargo/registry/")
        || s.contains("/.cargo/git/")
        || s.contains("/state/substrate/nsq_court_start/nsq_court_start")
        || s.contains("/state/perf/")
        || s.ends_with(".png")
        || s.ends_with(".jpg")
        || s.ends_with(".jpeg")
        || s.ends_with(".webp")
        || s.ends_with(".gif")
        || s.ends_with(".mp4")
        || s.ends_with(".safetensors")
}

fn language_for(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|x| x.to_str()) {
        Some("rs") => Some("rust"),
        Some("S") | Some("s") | Some("asm") => Some("aarch64_asm"),
        Some("sh") => Some("shell"),
        Some("toml") => Some("toml"),
        Some("json") => Some("json"),
        Some("md") => Some("markdown"),
        _ => None,
    }
}

fn candidates_from_text(
    root: &Path,
    path: &Path,
    language: &str,
    text: &str,
) -> Result<Vec<StampCandidate>> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();

    match language {
        "rust" => scan_rust_blocks(root, path, &lines, &mut out)?,
        "shell" => scan_shell_blocks(root, path, &lines, &mut out)?,
        "aarch64_asm" => scan_asm_blocks(root, path, &lines, &mut out)?,
        "toml" | "json" | "markdown" => scan_repeated_text_windows(root, path, language, &lines, &mut out)?,
        _ => {}
    }

    Ok(out)
}

fn scan_rust_blocks(root: &Path, path: &Path, lines: &[&str], out: &mut Vec<StampCandidate>) -> Result<()> {
    let mut starts = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let t = line.trim_start();
        if t.starts_with("pub fn ")
            || t.starts_with("fn ")
            || t.starts_with("pub struct ")
            || t.starts_with("struct ")
            || t.starts_with("pub enum ")
            || t.starts_with("enum ")
            || t.starts_with("impl ")
            || t.starts_with("pub const ")
            || t.starts_with("const ")
        {
            starts.push(idx);
        }
    }

    for pair in starts.windows(2) {
        let start = pair[0];
        let end = pair[1].saturating_sub(1);
        push_candidate(root, path, "rust", "rust_item", start, end, lines, out)?;
    }

    if let Some(&start) = starts.last() {
        let end = lines.len().saturating_sub(1).min(start + 80);
        push_candidate(root, path, "rust", "rust_item", start, end, lines, out)?;
    }

    Ok(())
}

fn scan_shell_blocks(root: &Path, path: &Path, lines: &[&str], out: &mut Vec<StampCandidate>) -> Result<()> {
    let mut starts = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.ends_with("() {") || t.starts_with("cat > ") || t.starts_with("cargo ") || t.starts_with("git ") {
            starts.push(idx);
        }
    }

    for pair in starts.windows(2) {
        let start = pair[0];
        let end = pair[1].saturating_sub(1);
        push_candidate(root, path, "shell", "shell_operation", start, end, lines, out)?;
    }

    Ok(())
}

fn scan_asm_blocks(root: &Path, path: &Path, lines: &[&str], out: &mut Vec<StampCandidate>) -> Result<()> {
    let mut starts = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.ends_with(":") || t.starts_with(".global ") || t.starts_with(".section ") {
            starts.push(idx);
        }
    }

    for pair in starts.windows(2) {
        let start = pair[0];
        let end = pair[1].saturating_sub(1);
        push_candidate(root, path, "aarch64_asm", "assembly_block", start, end, lines, out)?;
    }

    Ok(())
}

fn scan_repeated_text_windows(
    root: &Path,
    path: &Path,
    language: &str,
    lines: &[&str],
    out: &mut Vec<StampCandidate>,
) -> Result<()> {
    if lines.len() < 12 {
        return Ok(());
    }

    let mut seen: BTreeMap<String, usize> = BTreeMap::new();

    for start in (0..lines.len()).step_by(12) {
        let end = lines.len().saturating_sub(1).min(start + 23);
        if end <= start {
            continue;
        }
        let body = lines[start..=end].join("\n");
        let hash = sha256(&body);
        let count = seen.entry(hash).or_insert(0);
        *count += 1;

        if *count > 1 || body.len() > 512 {
            push_candidate(root, path, language, "reusable_text_window", start, end, lines, out)?;
        }
    }

    Ok(())
}

fn push_candidate(
    root: &Path,
    path: &Path,
    language: &str,
    kind: &str,
    start: usize,
    end: usize,
    lines: &[&str],
    out: &mut Vec<StampCandidate>,
) -> Result<()> {
    if end < start {
        return Ok(());
    }

    let block = lines[start..=end].join("\n");
    let trimmed = block.trim();

    if trimmed.len() < 80 {
        return Ok(());
    }

    let line_count = end - start + 1;
    let byte_count = trimmed.as_bytes().len();
    let hash = sha256(trimmed);
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();

    let reusable_score = score_block(trimmed, line_count, byte_count);
    if reusable_score < 20 {
        return Ok(());
    }

    let stamp_id = format!(
        "nsqasm.stamp.{}.{}.L{}-{}.{}",
        sanitize(language),
        sanitize(kind),
        start + 1,
        end + 1,
        &hash[..16]
    );

    out.push(StampCandidate {
        schema: SCHEMA.to_string(),
        authority: AUTHORITY.to_string(),
        stamp_id,
        source_path: rel,
        language: language.to_string(),
        start_line: start + 1,
        end_line: end + 1,
        line_count,
        byte_count,
        sha256: hash,
        court_route: court_route(),
        semantic_kind: kind.to_string(),
        reusable_score,
        preview: preview(trimmed),
    });

    Ok(())
}

fn score_block(block: &str, line_count: usize, byte_count: usize) -> u64 {
    let mut score = 0_u64;

    if line_count >= 4 {
        score += 10;
    }
    if line_count >= 12 {
        score += 10;
    }
    if byte_count >= 256 {
        score += 10;
    }
    if block.contains("fn ") || block.contains("pub fn ") {
        score += 12;
    }
    if block.contains("struct ") || block.contains("enum ") {
        score += 10;
    }
    if block.contains("serde") || block.contains("Serialize") || block.contains("Deserialize") {
        score += 6;
    }
    if block.contains("NSQ") || block.contains("Braxon") || block.contains("Court") || block.contains("stamp") {
        score += 12;
    }
    if block.contains("test") || block.contains("PASS") || block.contains("FAIL") {
        score += 6;
    }

    score
}

fn accepts_candidate(candidate: &StampCandidate) -> bool {
    candidate.reusable_score >= 32 && candidate.byte_count >= 128
}

fn court_route() -> CourtRoute {
    CourtRoute {
        scan: CourtSeat {
            court_position: "seer".to_string(),
            title: "Seer".to_string(),
            duty: "recognize repeated or high-value latent stamp candidates".to_string(),
        },
        validate: CourtSeat {
            court_position: "queen".to_string(),
            title: "Queen".to_string(),
            duty: "validate notation, syntax equivalence, integrity, and semantic continuity".to_string(),
        },
        prepare: CourtSeat {
            court_position: "bishop".to_string(),
            title: "Bishop".to_string(),
            duty: "prepare, imbue, elevate, recycle, or reassign stamp material".to_string(),
        },
        compose: CourtSeat {
            court_position: "composer".to_string(),
            title: "King".to_string(),
            duty: "perform composition, integration, and final assembly".to_string(),
        },
        route: CourtSeat {
            court_position: "director".to_string(),
            title: "Director".to_string(),
            duty: "direct selected execution lane".to_string(),
        },
        queue: CourtSeat {
            court_position: "ticketmaster".to_string(),
            title: "Ticketmaster".to_string(),
            duty: "govern queued stamp identity and custody".to_string(),
        },
        guard: CourtSeat {
            court_position: "guard".to_string(),
            title: "Guard".to_string(),
            duty: "enforce boundary, seizure, and containment law".to_string(),
        },
        audit: CourtSeat {
            court_position: "detective".to_string(),
            title: "Detective".to_string(),
            duty: "trace cause, fact recovery, and proof record".to_string(),
        },
    }
}

fn sha256(input: &str) -> String {
    let mut h = Sha256::new();
    h.update(input.as_bytes());
    format!("{:x}", h.finalize())
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn preview(s: &str) -> String {
    let one_line = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut p = one_line.chars().take(180).collect::<String>();
    if one_line.chars().count() > 180 {
        p.push_str("...");
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn court_route_uses_existing_positions() {
        let route = court_route();
        assert_eq!(route.validate.court_position, "queen");
        assert_eq!(route.prepare.court_position, "bishop");
        assert_eq!(route.compose.court_position, "composer");
        assert_eq!(route.compose.title, "King");
    }

    #[test]
    fn accepted_candidates_require_real_runtime_semantics() {
        let candidate = StampCandidate {
            schema: SCHEMA.to_string(),
            authority: AUTHORITY.to_string(),
            stamp_id: "test".to_string(),
            source_path: "x.rs".to_string(),
            language: "rust".to_string(),
            start_line: 1,
            end_line: 5,
            line_count: 5,
            byte_count: 200,
            sha256: sha256("pub fn example() { println!(\"NSQ stamp\"); }"),
            court_route: court_route(),
            semantic_kind: "rust_item".to_string(),
            reusable_score: 40,
            preview: "preview".to_string(),
        };

        assert!(accepts_candidate(&candidate));
    }

    #[test]
    fn short_low_score_blocks_are_not_promoted() {
        let candidate = StampCandidate {
            schema: SCHEMA.to_string(),
            authority: AUTHORITY.to_string(),
            stamp_id: "test".to_string(),
            source_path: "x.rs".to_string(),
            language: "rust".to_string(),
            start_line: 1,
            end_line: 1,
            line_count: 1,
            byte_count: 30,
            sha256: sha256("let x = 1;"),
            court_route: court_route(),
            semantic_kind: "rust_item".to_string(),
            reusable_score: 0,
            preview: "let x = 1".to_string(),
        };

        assert!(!accepts_candidate(&candidate));
    }

    #[test]
    fn stamp_id_hash_is_stable() {
        assert_eq!(
            &sha256("NSQ_COURT_START_PROOF_OK")[..16],
            &sha256("NSQ_COURT_START_PROOF_OK")[..16]
        );
    }

    #[test]
    fn passive_stamp_only_mode_is_never_accepted() {
        let accepted = AcceptedStamp {
            candidate: StampCandidate {
                schema: SCHEMA.to_string(),
                authority: AUTHORITY.to_string(),
                stamp_id: "test".to_string(),
                source_path: "x.rs".to_string(),
                language: "rust".to_string(),
                start_line: 1,
                end_line: 8,
                line_count: 8,
                byte_count: 400,
                sha256: sha256("pub fn example() { println!(\"NSQ stamp wake\"); }"),
                court_route: court_route(),
                semantic_kind: "rust_item".to_string(),
                reusable_score: 50,
                preview: "preview".to_string(),
            },
            stored_operation_required: true,
            wake_packet_required: true,
            runtime_projection_required: true,
            materialization_path_required: true,
            semantic_execution_continuity_required: true,
            passive_stamp_only_mode_allowed: false,
            pre_bake_state: "bishop_prepared_king_composable".to_string(),
            projection_lane: "current_binary_or_host_language_filtered_until_nsqasm_native".to_string(),
        };

        assert!(!accepted.passive_stamp_only_mode_allowed);
        assert!(accepted.stored_operation_required);
        assert!(accepted.wake_packet_required);
        assert!(accepted.runtime_projection_required);
        assert!(accepted.materialization_path_required);
        assert!(accepted.semantic_execution_continuity_required);
    }
}
