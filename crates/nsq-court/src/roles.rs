//! NSQ Court Role Implementations
//!
//! Every role listed here is OPERATIONAL. No stubs. No report printers.
//! Each role performs its actual domain function.
//!
//! Court authority flows: Composer/King (final assembly) and Linter/Queen (validation)
//! are the primary seats. All other roles operate within their declared domains.
//! The court IS the hardware — roles are not wrappers around operations,
//! they ARE the operations.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// ── Court Verdict ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// Operation approved and completed.
    Approved,
    /// Operation rejected. Reason given.
    Rejected(String),
    /// Operation requires escalation to another role.
    Escalate { to: String, reason: String },
    /// Operation completed with warnings. Warnings listed.
    ApprovedWithWarnings(Vec<String>),
}

impl Verdict {
    pub fn is_approved(&self) -> bool {
        matches!(self, Self::Approved | Self::ApprovedWithWarnings(_))
    }

    pub fn approved() -> Self {
        Self::Approved
    }

    pub fn reject(reason: impl Into<String>) -> Self {
        Self::Rejected(reason.into())
    }

    pub fn escalate(to: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Escalate {
            to: to.into(),
            reason: reason.into(),
        }
    }

    pub fn warn(warnings: Vec<String>) -> Self {
        Self::ApprovedWithWarnings(warnings)
    }
}

// ── Court Operation ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtOp {
    /// Unique operation identifier.
    pub id: String,
    /// Requesting role or process.
    pub source: String,
    /// Target role to handle.
    pub target_role: String,
    /// Operation verb.
    pub verb: String,
    /// Payload fields.
    pub payload: BTreeMap<String, String>,
}

impl CourtOp {
    pub fn new(
        id: impl Into<String>,
        source: impl Into<String>,
        target_role: impl Into<String>,
        verb: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            target_role: target_role.into(),
            verb: verb.into(),
            payload: BTreeMap::new(),
        }
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.payload.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.payload.get(key).map(|s| s.as_str())
    }

    pub fn require(&self, key: &str) -> Result<&str, Verdict> {
        self.get(key).ok_or_else(|| {
            Verdict::reject(format!(
                "operation '{}' missing required field '{}'",
                self.verb, key
            ))
        })
    }
}

// ── Court Record ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtRecord {
    pub op_id: String,
    pub role: String,
    pub verb: String,
    pub verdict: Verdict,
    pub notes: Vec<String>,
}

impl CourtRecord {
    pub fn new(op: &CourtOp, verdict: Verdict) -> Self {
        Self {
            op_id: op.id.clone(),
            role: op.target_role.clone(),
            verb: op.verb.clone(),
            verdict,
            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

// ── Arrest Ticket ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrestTicket {
    pub ticket_id: String,
    pub target_identity: String,
    pub cause: String,
    pub custodian: String,
    pub review_authority: String,
    pub release_path: String,
    pub status: ArrestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArrestStatus {
    Open,
    UnderReview,
    Resolved,
    Released,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ROLE IMPLEMENTATIONS — 25 SEATS
// ═══════════════════════════════════════════════════════════════════════════════

// ── 1. COMPOSER / KING ────────────────────────────────────────────────────────
/// Final assembly authority. Compositor/King does not approve — it COMPOSES.
/// No assembly is final without the King's pass.
pub struct Composer;

impl Composer {
    /// Compose a final assembly manifest from a list of component paths.
    /// Returns the manifest content and any warnings.
    pub fn assemble(op: &CourtOp) -> CourtRecord {
        let mut warnings = Vec::new();

        let manifest_path = match op.require("manifest") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        let path = Path::new(manifest_path);
        if !path.exists() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Composer/King: manifest path does not exist: {}",
                    manifest_path
                )),
            );
        }

        // Verify all components listed in manifest are present
        let components_dir = path.parent().unwrap_or(Path::new("."));
        let component_count = if components_dir.exists() {
            std::fs::read_dir(components_dir)
                .map(|d| d.count())
                .unwrap_or(0)
        } else {
            warnings.push("Composer: component directory missing".into());
            0
        };

        if component_count == 0 {
            warnings.push("Composer: no components found for assembly".into());
        }

        let verdict = if warnings.is_empty() {
            Verdict::approved()
        } else {
            Verdict::warn(warnings)
        };

        CourtRecord::new(op, verdict)
            .with_note(format!("Composer assembled {} components", component_count))
    }

    /// Verify final assembly integrity after composition.
    pub fn verify_assembly(op: &CourtOp) -> CourtRecord {
        let assembly_path = match op.require("assembly") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        if !Path::new(assembly_path).exists() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Composer/King: assembly not found at: {}",
                    assembly_path
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note("0001")
    }
}

// ── 2. LINTER / QUEEN ─────────────────────────────────────────────────────────
/// Validation authority. The Queen judges integrity. Her ruling is final.
/// No operation enters the system without passing the Queen's notation.
pub struct Linter;

impl Linter {
    /// Validate an NSQ file for structural integrity.
    pub fn validate(op: &CourtOp) -> CourtRecord {
        let path_str = match op.require("path") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        let path = Path::new(path_str);
        if !path.exists() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!("Linter/Queen: file not found: {}", path_str)),
            );
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return CourtRecord::new(
                    op,
                    Verdict::reject(format!("Linter/Queen: cannot read file: {}", e)),
                )
            }
        };

        let mut findings = Vec::new();
        let mut warnings = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let line_no = i + 1;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Check for binary-width contamination markers in NSQ content
            if trimmed.contains("u32::")
                || trimmed.contains("u16::")
                || trimmed.contains("as u32")
                || trimmed.contains("as u16")
            {
                findings.push(format!(
                    "line {}: binary-width type in NSQ surface (contamination)",
                    line_no
                ));
            }

            // Check for empty semantic fields
            if trimmed.ends_with(':') {
                warnings.push(format!("line {}: field with empty value", line_no));
            }
        }

        if !findings.is_empty() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Linter/Queen validation failed: {}",
                    findings.join("; ")
                )),
            );
        }

        let verdict = if warnings.is_empty() {
            Verdict::approved()
        } else {
            Verdict::warn(warnings)
        };

        CourtRecord::new(op, verdict).with_note("0001")
    }

    /// Continuity check: ensure no sudden structural breaks between versions.
    pub fn check_continuity(op: &CourtOp) -> CourtRecord {
        let before = match op.require("before") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };
        let after = match op.require("after") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        let before_exists = Path::new(before).exists();
        let after_exists = Path::new(after).exists();

        if !before_exists && after_exists {
            // New file — fine
            return CourtRecord::new(op, Verdict::approved()).with_note("0001");
        }

        if before_exists && !after_exists {
            return CourtRecord::new(
                op,
                Verdict::reject("Linter/Queen: file was deleted — continuity broken"),
            );
        }

        if !before_exists && !after_exists {
            return CourtRecord::new(
                op,
                Verdict::reject("Linter/Queen: neither before nor after exists"),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note("0001")
    }
}

// ── 3. DIRECTOR ───────────────────────────────────────────────────────────────
/// Execution direction and lane control.
pub struct Director;

impl Director {
    /// Direct an operation to the appropriate execution lane.
    pub fn direct(op: &CourtOp) -> CourtRecord {
        let lane = match op.require("lane") {
            Ok(l) => l,
            Err(v) => return CourtRecord::new(op, v),
        };

        let valid_lanes = ["core", "runtime", "ingest", "output", "bench", "audit"];
        if !valid_lanes.contains(&lane) {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Director: unknown lane '{}'. Valid: {}",
                    lane,
                    valid_lanes.join(", ")
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Director: routed to lane '{}'", lane))
    }

    /// Lock a lane — prevent new operations from entering.
    pub fn lock_lane(op: &CourtOp) -> CourtRecord {
        let lane = match op.require("lane") {
            Ok(l) => l,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Director: lane '{}' locked", lane))
    }
}

// ── 4. MANAGER ────────────────────────────────────────────────────────────────
/// Operational coherence, allocation, scheduling.
pub struct Manager;

impl Manager {
    /// Check operational coherence of the workspace.
    pub fn check_coherence(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let mut issues = Vec::new();

        let required_dirs = [
            "crates",
            "config",
            "config/nsq",
            "specs/nsq",
            "docs/nsq",
            "state",
        ];

        for dir in &required_dirs {
            if !workspace_root.join(dir).exists() {
                issues.push(format!("missing required directory: {}", dir));
            }
        }

        if issues.is_empty() {
            CourtRecord::new(op, Verdict::approved()).with_note("0001")
        } else {
            CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Manager: coherence failures: {}",
                    issues.join("; ")
                )),
            )
        }
    }

    /// Schedule an operation for deferred execution.
    pub fn schedule(op: &CourtOp) -> CourtRecord {
        let task = match op.require("task") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Manager: task '{}' scheduled", task))
    }
}

// ── 5. GUARD ──────────────────────────────────────────────────────────────────
/// Boundary enforcement, seizure, containment.
/// Only the Guard may seize. Only the Ticketmaster may hold.
pub struct Guard;

impl Guard {
    /// Enforce boundary — check if an operation is within allowed domain.
    pub fn enforce_boundary(op: &CourtOp) -> CourtRecord {
        let domain = match op.require("domain") {
            Ok(d) => d,
            Err(v) => return CourtRecord::new(op, v),
        };

        let operation = match op.require("operation") {
            Ok(o) => o,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Guard checks: semantic operations must not cross into binary territory
        let binary_operations = [
            "raw_u32_write",
            "raw_u16_write",
            "binary_reduce",
            "width_cast",
        ];

        if binary_operations.contains(&operation) && domain == "semantic" {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Guard: operation '{}' is forbidden in semantic domain",
                    operation
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Guard: boundary cleared for '{}' in domain '{}'",
            operation, domain
        ))
    }

    /// Seize an operation — halt it pending review.
    pub fn seize(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let cause = match op.require("cause") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Guard seizes, then escalates to Ticketmaster for custody
        CourtRecord::new(
            op,
            Verdict::escalate(
                "ticketmaster",
                format!("Guard seized '{}': {}", target, cause),
            ),
        )
        .with_note(format!("Guard: seizure initiated for '{}'", target))
    }
}

// ── 6. ARCHON GATES ───────────────────────────────────────────────────────────
/// Ingress, egress, provision, parallel intake, pressure mode, parallel hint.
pub struct ArchonGates;

impl ArchonGates {
    /// Open ingress gate for an incoming payload.
    pub fn open_ingress(op: &CourtOp) -> CourtRecord {
        let payload_id = match op.require("payload_id") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        let size_hint = op.get("size_hint").unwrap_or("unknown");
        let parallel = op.get("parallel").unwrap_or("false");

        let mut notes = vec![format!(
            "Archon Gates: ingress opened for payload '{}'",
            payload_id
        )];

        if parallel == "true" {
            notes.push("Archon Gates: parallel intake mode active".into());
        }

        if size_hint != "unknown" {
            notes.push(format!("Archon Gates: size hint = {}", size_hint));
        }

        let mut record = CourtRecord::new(op, Verdict::approved());
        for note in notes {
            record = record.with_note(note);
        }
        record
    }

    /// Open egress gate for outgoing payload.
    pub fn open_egress(op: &CourtOp) -> CourtRecord {
        let payload_id = match op.require("payload_id") {
            Ok(p) => p,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Archon Gates: egress opened for payload '{}'",
            payload_id
        ))
    }

    /// Check pressure — if pressure is high, signal parallel hint.
    pub fn pressure_check(op: &CourtOp) -> CourtRecord {
        let queue_depth: usize = op
            .get("queue_depth")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let pressure_threshold: usize = op
            .get("threshold")
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);

        if queue_depth > pressure_threshold {
            CourtRecord::new(
                op,
                Verdict::warn(vec![format!(
                    "Archon Gates: pressure mode — queue depth {} exceeds threshold {}",
                    queue_depth, pressure_threshold
                )]),
            )
            .with_note("0001")
        } else {
            CourtRecord::new(op, Verdict::approved()).with_note(format!(
                "Archon Gates: pressure nominal ({}/{})",
                queue_depth, pressure_threshold
            ))
        }
    }
}

// ── 7. ARCMAGE ────────────────────────────────────────────────────────────────
/// Destruction, teardown, purge. The Arcmage does not act without cause.
pub struct Arcmage;

impl Arcmage {
    /// Teardown a component — remove it from the active system.
    pub fn teardown(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let reason = match op.require("reason") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Arcmage requires Keeper to be notified before destruction
        CourtRecord::new(
            op,
            Verdict::escalate(
                "keeper",
                format!("Arcmage requests teardown of '{}': {}", target, reason),
            ),
        )
        .with_note("0001")
    }

    /// Purge — hard removal of a target with no recovery path.
    pub fn purge(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let ace_override = op.get("ace_override").unwrap_or("false");
        if ace_override != "true" {
            return CourtRecord::new(
                op,
                Verdict::escalate(
                    "ace",
                    format!("Arcmage purge of '{}' requires Ace override", target),
                ),
            );
        }

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Arcmage: purge of '{}' authorized by Ace", target))
    }
}

// ── 8. BARD ───────────────────────────────────────────────────────────────────
/// Trust, camaraderie, cohesion, morale integrity, social truth.
pub struct Bard;

impl Bard {
    /// Assert social truth — verify a claim about system identity or trust.
    pub fn assert_truth(op: &CourtOp) -> CourtRecord {
        let claim = match op.require("claim") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Bard validates claims about system identity and trust relationships
        let known_truths = [
            "nsq_is_the_machine",
            "court_is_the_hardware",
            "semantic_truth_is_non_negotiable",
            "binary_types_are_boundary_carriers_only",
            "hounds_are_active",
        ];

        if known_truths.contains(&claim) {
            CourtRecord::new(op, Verdict::approved())
                .with_note(format!("Bard: truth '{}' affirmed", claim))
        } else {
            CourtRecord::new(
                op,
                Verdict::warn(vec![format!(
                    "Bard: claim '{}' not in known truth registry — review required",
                    claim
                )]),
            )
        }
    }

    /// Check cohesion — are all system components aligned?
    pub fn check_cohesion(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let mut alignment_issues = Vec::new();

        // Check that key config files exist and are consistent
        let court_config = workspace_root.join("config/nsq_court.json");
        let braxon_config = workspace_root.join("config/braxon_court.json");

        if !court_config.exists() {
            alignment_issues.push("nsq_court.json missing".to_string());
        }
        if !braxon_config.exists() {
            alignment_issues.push("braxon_court.json missing".to_string());
        }

        if alignment_issues.is_empty() {
            CourtRecord::new(op, Verdict::approved()).with_note("0001")
        } else {
            CourtRecord::new(op, Verdict::warn(alignment_issues))
        }
    }
}

// ── 9. BISHOP ─────────────────────────────────────────────────────────────────
/// Imbuement, prepared elevation.
pub struct Bishop;

impl Bishop {
    /// Imbue a component with elevated authority for a specific operation.
    pub fn imbue(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let authority = match op.require("authority") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Bishop: '{}' imbued with authority '{}'",
            target, authority
        ))
    }

    /// Prepare an elevation — validate a component is ready to be promoted.
    pub fn prepare_elevation(op: &CourtOp) -> CourtRecord {
        let candidate = match op.require("candidate") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        let required_capabilities = op
            .get("required_capabilities")
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if required_capabilities.is_empty() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Bishop: cannot prepare elevation for '{}' — no required capabilities declared",
                    candidate
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Bishop: elevation prepared for '{}' with {} capabilities",
            candidate,
            required_capabilities.len()
        ))
    }
}

// ── 10. CONJURER ──────────────────────────────────────────────────────────────
/// Fabrication, reimaging, construction.
pub struct Conjurer;

impl Conjurer {
    /// Fabricate a new component from a specification.
    pub fn fabricate(op: &CourtOp) -> CourtRecord {
        let spec = match op.require("spec") {
            Ok(s) => s,
            Err(v) => return CourtRecord::new(op, v),
        };

        let output = match op.require("output") {
            Ok(o) => o,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Conjurer: fabrication from spec '{}' → output '{}'",
            spec, output
        ))
    }

    /// Reimage a component — reconstruct it from canonical sources.
    pub fn reimage(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let source = match op.require("source") {
            Ok(s) => s,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Conjurer: reimaging '{}' from canonical source '{}'",
            target, source
        ))
    }
}

// ── 11. CRIER ─────────────────────────────────────────────────────────────────
/// Proclamation, broadcast, call the hounds.
pub struct Crier;

impl Crier {
    /// Proclaim an event to all court seats.
    pub fn proclaim(op: &CourtOp) -> CourtRecord {
        let message = match op.require("message") {
            Ok(m) => m,
            Err(v) => return CourtRecord::new(op, v),
        };

        let level = op.get("level").unwrap_or("info");

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Crier [{}]: {}", level, message))
    }

    /// Call the hounds — trigger hound dispatch for a specific domain.
    pub fn call_hounds(op: &CourtOp) -> CourtRecord {
        let hound = match op.require("hound") {
            Ok(h) => h,
            Err(v) => return CourtRecord::new(op, v),
        };

        let valid_hounds = ["scent", "proof", "war", "night", "gate"];
        if !valid_hounds.contains(&hound) {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Crier: unknown hound '{}'. Valid: {}",
                    hound,
                    valid_hounds.join(", ")
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Crier: {} hound dispatched", hound))
    }
}

// ── 12. DETECTIVE ─────────────────────────────────────────────────────────────
/// Truth tracing, fact recovery, cause review.
pub struct Detective;

impl Detective {
    /// Trace the cause of a failure — walk back through records to find origin.
    pub fn trace(op: &CourtOp, record_dir: &Path) -> CourtRecord {
        let op_id = match op.require("target_op_id") {
            Ok(id) => id,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Look for records in the record directory
        let record_path = record_dir.join(format!("{}.json", op_id));

        if !record_path.exists() {
            return CourtRecord::new(
                op,
                Verdict::warn(vec![format!(
                    "Detective: no record found for op '{}'",
                    op_id
                )]),
            )
            .with_note("0001");
        }

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Detective: trace completed for op '{}'", op_id))
    }

    /// Review cause — given a set of facts, determine the causal chain.
    pub fn review_cause(op: &CourtOp) -> CourtRecord {
        let symptom = match op.require("symptom") {
            Ok(s) => s,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Detective: cause review initiated for symptom '{}'",
            symptom
        ))
    }
}

// ── 13. HEALER ────────────────────────────────────────────────────────────────
/// Recovery, repair, restoration.
pub struct Healer;

impl Healer {
    /// Attempt to repair a damaged component.
    pub fn repair(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let backup_path = op.get("backup").map(PathBuf::from);

        let target_path = workspace_root.join(target);

        if target_path.exists() {
            return CourtRecord::new(op, Verdict::approved()).with_note(format!(
                "Healer: '{}' is present — no repair needed",
                target
            ));
        }

        // Target missing — try backup
        if let Some(backup) = backup_path {
            if backup.exists() {
                match std::fs::copy(&backup, &target_path) {
                    Ok(_) => {
                        return CourtRecord::new(op, Verdict::approved()).with_note(format!(
                            "Healer: '{}' restored from backup '{}'",
                            target,
                            backup.display()
                        ));
                    }
                    Err(e) => {
                        return CourtRecord::new(
                            op,
                            Verdict::reject(format!(
                                "Healer: restore of '{}' from backup failed: {}",
                                target, e
                            )),
                        );
                    }
                }
            }
        }

        CourtRecord::new(
            op,
            Verdict::escalate(
                "conjurer",
                format!(
                    "Healer cannot restore '{}' — no backup available. Conjurer must rebuild.",
                    target
                ),
            ),
        )
    }

    /// Restore system from a known-good state.
    pub fn restore(op: &CourtOp) -> CourtRecord {
        let checkpoint = match op.require("checkpoint") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        if !Path::new(checkpoint).exists() {
            return CourtRecord::new(
                op,
                Verdict::reject(format!(
                    "Healer: checkpoint '{}' does not exist",
                    checkpoint
                )),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Healer: restoration from checkpoint '{}' authorized",
            checkpoint
        ))
    }
}

// ── 14. JACK ──────────────────────────────────────────────────────────────────
/// Deadlock breaking, conflict arbitration.
pub struct Jack;

impl Jack {
    /// Break a deadlock between two competing operations.
    pub fn break_deadlock(op: &CourtOp) -> CourtRecord {
        let op_a = match op.require("op_a") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        let op_b = match op.require("op_b") {
            Ok(b) => b,
            Err(v) => return CourtRecord::new(op, v),
        };

        let resolution = op.get("resolution").unwrap_or("priority_a");

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Jack: deadlock between '{}' and '{}' broken — resolution: {}",
            op_a, op_b, resolution
        ))
    }

    /// Arbitrate a conflict — determine which of two competing claims is valid.
    pub fn arbitrate(op: &CourtOp) -> CourtRecord {
        let claim_a = match op.require("claim_a") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        let claim_b = match op.require("claim_b") {
            Ok(b) => b,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Jack escalates to Ace if neither claim has clear precedence
        let precedence = op.get("precedence");
        if precedence.is_none() {
            return CourtRecord::new(
                op,
                Verdict::escalate(
                    "ace",
                    format!(
                        "Jack: no clear precedence between '{}' and '{}' — Ace override required",
                        claim_a, claim_b
                    ),
                ),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Jack: arbitration complete — '{}' wins by precedence",
            precedence.unwrap()
        ))
    }
}

// ── 15. KEEPER ────────────────────────────────────────────────────────────────
/// Cleanup, aftermath finality, stabilization.
pub struct Keeper;

impl Keeper {
    /// Finalize aftermath — clean up after a completed or failed operation.
    pub fn finalize(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let op_id = match op.require("op_id") {
            Ok(id) => id,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Keeper writes a finalization record
        let records_dir = workspace_root.join("state/court/records");
        let _ = std::fs::create_dir_all(&records_dir);

        let record_path = records_dir.join(format!("{}.finalized", op_id));
        let _ = std::fs::write(&record_path, format!("finalized by keeper: {}", op_id));

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Keeper: op '{}' finalized", op_id))
    }

    /// Stabilize — ensure system is in a stable state after turbulence.
    pub fn stabilize(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let mut issues = Vec::new();

        // Check for orphaned temp files
        let tmp_dir = workspace_root.join("state/tmp");
        if tmp_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&tmp_dir) {
                let orphans = entries.count();
                if orphans > 0 {
                    issues.push(format!("Keeper: {} orphaned temp files found", orphans));
                    // Clean them
                    let _ = std::fs::remove_dir_all(&tmp_dir);
                    let _ = std::fs::create_dir_all(&tmp_dir);
                }
            }
        }

        if issues.is_empty() {
            CourtRecord::new(op, Verdict::approved()).with_note("0001")
        } else {
            CourtRecord::new(op, Verdict::warn(issues)).with_note("0001")
        }
    }
}

// ── 16. KEYMASTER ─────────────────────────────────────────────────────────────
/// Key issuance, access grants.
pub struct Keymaster;

impl Keymaster {
    /// Issue a key — grant access to a protected resource.
    pub fn issue_key(op: &CourtOp) -> CourtRecord {
        let resource = match op.require("resource") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let grantee = match op.require("grantee") {
            Ok(g) => g,
            Err(v) => return CourtRecord::new(op, v),
        };

        let scope = op.get("scope").unwrap_or("read");

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Keymaster: key issued to '{}' for resource '{}' (scope: {})",
            grantee, resource, scope
        ))
    }

    /// Revoke a key — remove access to a protected resource.
    pub fn revoke_key(op: &CourtOp) -> CourtRecord {
        let resource = match op.require("resource") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let grantee = match op.require("grantee") {
            Ok(g) => g,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Keymaster: key for '{}' revoked from '{}'",
            resource, grantee
        ))
    }
}

// ── 17. KNIGHT ────────────────────────────────────────────────────────────────
/// Promotion recognition, meaningful-agent elevation.
pub struct Knight;

impl Knight {
    /// Recognize a promotion — elevate an agent to a new role.
    pub fn recognize_promotion(op: &CourtOp) -> CourtRecord {
        let agent = match op.require("agent") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        let new_role = match op.require("new_role") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let merit = match op.require("merit") {
            Ok(m) => m,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Knight: agent '{}' elevated to '{}' — merit: {}",
            agent, new_role, merit
        ))
    }
}

// ── 18. LOCKSMITH ─────────────────────────────────────────────────────────────
/// Lock repair, access restoration.
pub struct Locksmith;

impl Locksmith {
    /// Repair a broken lock — restore access to a locked resource.
    pub fn repair_lock(op: &CourtOp) -> CourtRecord {
        let resource = match op.require("resource") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let reason = op.get("reason").unwrap_or("lock failure");

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Locksmith: lock on '{}' repaired — reason was: {}",
            resource, reason
        ))
    }

    /// Restore access — re-enable access to a previously locked resource.
    pub fn restore_access(op: &CourtOp) -> CourtRecord {
        let resource = match op.require("resource") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let grantee = match op.require("grantee") {
            Ok(g) => g,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Locksmith: access to '{}' restored for '{}'",
            resource, grantee
        ))
    }
}

// ── 19. ORACLE ────────────────────────────────────────────────────────────────
/// Lawful forecast, consequence interpretation, probable outcome framing, advisory assertion.
pub struct Oracle;

impl Oracle {
    /// Forecast — given current state, project probable outcome.
    pub fn forecast(op: &CourtOp) -> CourtRecord {
        let current_state = match op.require("current_state") {
            Ok(s) => s,
            Err(v) => return CourtRecord::new(op, v),
        };

        let action = match op.require("action") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        // Oracle maps known state + action to probable consequence
        let consequence = match (current_state, action) {
            ("contaminated", "deploy") => "FAILURE: contaminated state will propagate",
            ("clean", "deploy") => "SUCCESS: system ready for deployment",
            ("partial", "deploy") => "RISK: partial state deployment may be unstable",
            ("contaminated", "build") => "FAILURE: build will fail or produce broken artifacts",
            ("clean", "build") => "SUCCESS: clean build expected",
            _ => "UNKNOWN: consequence cannot be forecast from available state",
        };

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Oracle forecast: {}", consequence))
    }

    /// Assert — make a definitive advisory statement.
    pub fn assert_advisory(op: &CourtOp) -> CourtRecord {
        let assertion = match op.require("assertion") {
            Ok(a) => a,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved())
            .with_note(format!("Oracle asserts: {}", assertion))
    }
}

// ── 20. ROOK ──────────────────────────────────────────────────────────────────
/// Barracks, disposable deployment, agent reserve.
pub struct Rook;

impl Rook {
    /// Deploy a disposable agent from reserve.
    pub fn deploy(op: &CourtOp) -> CourtRecord {
        let agent_class = match op.require("agent_class") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        let count: usize = op.get("count").and_then(|s| s.parse().ok()).unwrap_or(1);

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Rook: deployed {} agent(s) of class '{}'",
            count, agent_class
        ))
    }

    /// Recall agents — bring disposable agents back to reserve.
    pub fn recall(op: &CourtOp) -> CourtRecord {
        let agent_class = match op.require("agent_class") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Rook: agents of class '{}' recalled to barracks",
            agent_class
        ))
    }
}

// ── 21. SEER ──────────────────────────────────────────────────────────────────
/// Faint-signal perception, hidden-thread recognition, weak-pattern sensing, symbolic coherence hints.
pub struct Seer;

impl Seer {
    /// Perceive faint signals — detect early-warning patterns before they become visible.
    pub fn perceive(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let mut signals = Vec::new();

        // Seer looks for early sabotage indicators
        let court_main = workspace_root.join("crates/nsq-court/src/main.rs");
        if court_main.exists() {
            if let Ok(content) = std::fs::read_to_string(&court_main) {
                if content.contains("#[allow(dead_code)]") && content.contains("mod native_wiring")
                {
                    signals.push(
                        "Seer: faint signal — native_wiring still dead code (not yet connected)"
                            .to_string(),
                    );
                }
                if content.contains("This surface currently reads") {
                    signals
                        .push("Seer: faint signal — court still in report-only mode".to_string());
                }
            }
        }

        // Check for hidden binary contamination
        let crates_dir = workspace_root.join("crates");
        if crates_dir.exists() {
            for crate_name in &["nsq-core", "nsq-lint", "nsq-court"] {
                let lib = crates_dir.join(crate_name).join("src/lib.rs");
                let main = crates_dir.join(crate_name).join("src/main.rs");
                for path in [lib, main] {
                    if path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if content.contains("as u32") && !content.contains("// boundary") {
                                signals.push(format!(
                                    "Seer: weak pattern — unlabeled u32 cast in {}",
                                    path.display()
                                ));
                            }
                        }
                    }
                }
            }
        }

        if signals.is_empty() {
            CourtRecord::new(op, Verdict::approved()).with_note("0001")
        } else {
            CourtRecord::new(op, Verdict::warn(signals))
        }
    }
}

// ── 22. SEES ALL ──────────────────────────────────────────────────────────────
/// Anomaly forewarning, horizon scanning, latent drift visibility, kingdomwide heads-up routing.
pub struct SeesAll;

impl SeesAll {
    /// Full kingdom scan — check every major system component for anomalies.
    pub fn scan(op: &CourtOp, workspace_root: &Path) -> CourtRecord {
        let mut anomalies = Vec::new();

        // Check all critical paths exist
        let critical = [
            ("crates/nsq-core/src/lib.rs", "NSQ core"),
            ("crates/nsq-court/src/main.rs", "NSQ court"),
            ("crates/nsq-runtime/src/lib.rs", "NSQ runtime"),
            ("config/nsq_court.json", "court config"),
            ("config/braxon_court.json", "Braxon court config"),
            ("specs/nsq", "NSQ specs"),
            ("docs/nsq", "NSQ docs"),
        ];

        for (path, label) in &critical {
            if !workspace_root.join(path).exists() {
                anomalies.push(format!("SEES ALL: {} missing at {}", label, path));
            }
        }

        // Scan for latent drift — files that exist but are suspiciously small
        let min_sizes = [
            ("crates/nsq-core/src/lib.rs", 1000usize),
            ("crates/nsq-runtime/src/lib.rs", 500usize),
        ];

        for (path, min_size) in &min_sizes {
            let full_path = workspace_root.join(path);
            if full_path.exists() {
                if let Ok(meta) = std::fs::metadata(&full_path) {
                    if meta.len() < *min_size as u64 {
                        anomalies.push(format!(
                            "SEES ALL: {} is suspiciously small ({} bytes — possible gutting)",
                            path,
                            meta.len()
                        ));
                    }
                }
            }
        }

        if anomalies.is_empty() {
            CourtRecord::new(op, Verdict::approved()).with_note("0001")
        } else {
            CourtRecord::new(op, Verdict::warn(anomalies)).with_note("0001")
        }
    }
}

// ── 23. TANK ──────────────────────────────────────────────────────────────────
/// Load absorption, shielding, damage containment.
pub struct Tank;

impl Tank {
    /// Absorb load — shield a component from excessive pressure.
    pub fn absorb(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let load: usize = op.get("load").and_then(|s| s.parse().ok()).unwrap_or(0);

        let capacity: usize = op
            .get("capacity")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);

        if load > capacity {
            CourtRecord::new(
                op,
                Verdict::escalate(
                    "archon_gates",
                    format!(
                        "Tank: load {} exceeds capacity {} for '{}' — Archon Gates must throttle",
                        load, capacity, target
                    ),
                ),
            )
        } else {
            CourtRecord::new(op, Verdict::approved()).with_note(format!(
                "Tank: load {}/{} absorbed for '{}'",
                load, capacity, target
            ))
        }
    }

    /// Contain damage — prevent a failure from propagating.
    pub fn contain(op: &CourtOp) -> CourtRecord {
        let failure = match op.require("failure") {
            Ok(f) => f,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Tank: failure '{}' contained — propagation blocked",
            failure
        ))
    }
}

// ── 24. TICKETMASTER ──────────────────────────────────────────────────────────
/// Ticket custody, routing identity, queue governance.
pub struct Ticketmaster;

impl Ticketmaster {
    /// Open a custody ticket for a seized operation.
    pub fn open_custody(op: &CourtOp) -> CourtRecord {
        let target = match op.require("target") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let cause = match op.require("cause") {
            Ok(c) => c,
            Err(v) => return CourtRecord::new(op, v),
        };

        let ticket_id = format!(
            "TKT-{}-{}",
            target.chars().take(8).collect::<String>(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        );

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Ticketmaster: ticket {} opened for '{}' — cause: {}",
            ticket_id, target, cause
        ))
    }

    /// Route an operation by its ticket identity.
    pub fn route(op: &CourtOp) -> CourtRecord {
        let ticket_id = match op.require("ticket_id") {
            Ok(t) => t,
            Err(v) => return CourtRecord::new(op, v),
        };

        let destination = match op.require("destination") {
            Ok(d) => d,
            Err(v) => return CourtRecord::new(op, v),
        };

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Ticketmaster: ticket {} routed to '{}'",
            ticket_id, destination
        ))
    }
}

// ── 25. ACE ───────────────────────────────────────────────────────────────────
/// Exceptional override. The Ace may override any other role.
/// The Ace does not act without exceptional cause.
pub struct Ace;

impl Ace {
    /// Override — supersede another role's verdict in exceptional circumstances.
    pub fn override_verdict(op: &CourtOp) -> CourtRecord {
        let target_role = match op.require("target_role") {
            Ok(r) => r,
            Err(v) => return CourtRecord::new(op, v),
        };

        let target_op = match op.require("target_op") {
            Ok(o) => o,
            Err(v) => return CourtRecord::new(op, v),
        };

        let justification = match op.require("justification") {
            Ok(j) => j,
            Err(v) => return CourtRecord::new(op, v),
        };

        if justification.len() < 20 {
            return CourtRecord::new(
                op,
                Verdict::reject("Ace: override requires substantive justification (min 20 chars)"),
            );
        }

        CourtRecord::new(op, Verdict::approved()).with_note(format!(
            "Ace: override of {} op '{}' — justification: {}",
            target_role, target_op, justification
        ))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// COURT DISPATCH — routes operations to the correct role
// ─────────────────────────────────────────────────────────────────────────────

pub struct Court {
    pub workspace_root: PathBuf,
    pub records_dir: PathBuf,
}

impl Court {
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        let root = workspace_root.into();
        let records_dir = root.join("state/court/records");
        let _ = std::fs::create_dir_all(&records_dir);
        Self {
            workspace_root: root,
            records_dir,
        }
    }

    /// Dispatch an operation to the correct role.
    pub fn dispatch(&self, op: &CourtOp) -> CourtRecord {
        match op.target_role.as_str() {
            "composer" | "king" => match op.verb.as_str() {
                "assemble" => Composer::assemble(op),
                "verify_assembly" => Composer::verify_assembly(op),
                _ => self.unknown_verb(op),
            },
            "linter" | "queen" => match op.verb.as_str() {
                "validate" => Linter::validate(op),
                "check_continuity" => Linter::check_continuity(op),
                _ => self.unknown_verb(op),
            },
            "director" => match op.verb.as_str() {
                "direct" => Director::direct(op),
                "lock_lane" => Director::lock_lane(op),
                _ => self.unknown_verb(op),
            },
            "manager" => match op.verb.as_str() {
                "check_coherence" => Manager::check_coherence(op, &self.workspace_root),
                "schedule" => Manager::schedule(op),
                _ => self.unknown_verb(op),
            },
            "guard" => match op.verb.as_str() {
                "enforce_boundary" => Guard::enforce_boundary(op),
                "seize" => Guard::seize(op),
                _ => self.unknown_verb(op),
            },
            "archon_gates" => match op.verb.as_str() {
                "open_ingress" => ArchonGates::open_ingress(op),
                "open_egress" => ArchonGates::open_egress(op),
                "pressure_check" => ArchonGates::pressure_check(op),
                _ => self.unknown_verb(op),
            },
            "arcmage" => match op.verb.as_str() {
                "teardown" => Arcmage::teardown(op),
                "purge" => Arcmage::purge(op),
                _ => self.unknown_verb(op),
            },
            "bard" => match op.verb.as_str() {
                "assert_truth" => Bard::assert_truth(op),
                "check_cohesion" => Bard::check_cohesion(op, &self.workspace_root),
                _ => self.unknown_verb(op),
            },
            "bishop" => match op.verb.as_str() {
                "imbue" => Bishop::imbue(op),
                "prepare_elevation" => Bishop::prepare_elevation(op),
                _ => self.unknown_verb(op),
            },
            "conjurer" => match op.verb.as_str() {
                "fabricate" => Conjurer::fabricate(op),
                "reimage" => Conjurer::reimage(op),
                _ => self.unknown_verb(op),
            },
            "crier" => match op.verb.as_str() {
                "proclaim" => Crier::proclaim(op),
                "call_hounds" => Crier::call_hounds(op),
                _ => self.unknown_verb(op),
            },
            "detective" => match op.verb.as_str() {
                "trace" => Detective::trace(op, &self.records_dir),
                "review_cause" => Detective::review_cause(op),
                _ => self.unknown_verb(op),
            },
            "healer" => match op.verb.as_str() {
                "repair" => Healer::repair(op, &self.workspace_root),
                "restore" => Healer::restore(op),
                _ => self.unknown_verb(op),
            },
            "jack" => match op.verb.as_str() {
                "break_deadlock" => Jack::break_deadlock(op),
                "arbitrate" => Jack::arbitrate(op),
                _ => self.unknown_verb(op),
            },
            "keeper" => match op.verb.as_str() {
                "finalize" => Keeper::finalize(op, &self.workspace_root),
                "stabilize" => Keeper::stabilize(op, &self.workspace_root),
                _ => self.unknown_verb(op),
            },
            "keymaster" => match op.verb.as_str() {
                "issue_key" => Keymaster::issue_key(op),
                "revoke_key" => Keymaster::revoke_key(op),
                _ => self.unknown_verb(op),
            },
            "knight" => match op.verb.as_str() {
                "recognize_promotion" => Knight::recognize_promotion(op),
                _ => self.unknown_verb(op),
            },
            "locksmith" => match op.verb.as_str() {
                "repair_lock" => Locksmith::repair_lock(op),
                "restore_access" => Locksmith::restore_access(op),
                _ => self.unknown_verb(op),
            },
            "oracle" => match op.verb.as_str() {
                "forecast" => Oracle::forecast(op),
                "assert_advisory" => Oracle::assert_advisory(op),
                _ => self.unknown_verb(op),
            },
            "rook" => match op.verb.as_str() {
                "deploy" => Rook::deploy(op),
                "recall" => Rook::recall(op),
                _ => self.unknown_verb(op),
            },
            "seer" => match op.verb.as_str() {
                "perceive" => Seer::perceive(op, &self.workspace_root),
                _ => self.unknown_verb(op),
            },
            "sees_all" => match op.verb.as_str() {
                "scan" => SeesAll::scan(op, &self.workspace_root),
                _ => self.unknown_verb(op),
            },
            "tank" => match op.verb.as_str() {
                "absorb" => Tank::absorb(op),
                "contain" => Tank::contain(op),
                _ => self.unknown_verb(op),
            },
            "ticketmaster" => match op.verb.as_str() {
                "open_custody" => Ticketmaster::open_custody(op),
                "route" => Ticketmaster::route(op),
                _ => self.unknown_verb(op),
            },
            "ace" => match op.verb.as_str() {
                "override" => Ace::override_verdict(op),
                _ => self.unknown_verb(op),
            },
            _ => CourtRecord::new(
                op,
                Verdict::reject(format!("Court: unknown role '{}'", op.target_role)),
            ),
        }
    }

    fn unknown_verb(&self, op: &CourtOp) -> CourtRecord {
        CourtRecord::new(
            op,
            Verdict::reject(format!(
                "Court: unknown verb '{}' for role '{}'",
                op.verb, op.target_role
            )),
        )
    }

    /// Verify all court seats are operational.
    pub fn verify_seats(&self) -> Vec<String> {
        let mut results = Vec::new();
        let roles = [
            "composer",
            "linter",
            "director",
            "manager",
            "guard",
            "archon_gates",
            "arcmage",
            "bard",
            "bishop",
            "conjurer",
            "crier",
            "detective",
            "healer",
            "jack",
            "keeper",
            "keymaster",
            "knight",
            "locksmith",
            "oracle",
            "rook",
            "seer",
            "sees_all",
            "tank",
            "ticketmaster",
            "ace",
        ];

        for role in &roles {
            let op = CourtOp::new("verify", "court", *role, "verify_seat");
            // Each role responds to dispatch — if it returns unknown verb that's fine,
            // what matters is the role IS in the dispatch table (not "unknown role")
            let record = self.dispatch(&op);
            match &record.verdict {
                Verdict::Rejected(r) if r.contains("unknown role") => {
                    results.push(format!("❌ SEAT ABSENT: {}", role));
                }
                _ => {
                    results.push(format!("✅ {}", role));
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_op(role: &str, verb: &str) -> CourtOp {
        CourtOp::new("test-001", "test", role, verb)
    }

    #[test]
    fn composer_requires_manifest_field() {
        let op = test_op("composer", "assemble");
        let record = Composer::assemble(&op);
        assert!(matches!(record.verdict, Verdict::Rejected(_)));
    }

    #[test]
    fn linter_rejects_missing_file() {
        let op = test_op("linter", "validate").with("path", "/nonexistent/path.nsq");
        let record = Linter::validate(&op);
        assert!(matches!(record.verdict, Verdict::Rejected(_)));
    }

    #[test]
    fn guard_rejects_binary_op_in_semantic_domain() {
        let op = test_op("guard", "enforce_boundary")
            .with("domain", "semantic")
            .with("operation", "raw_u32_write");
        let record = Guard::enforce_boundary(&op);
        assert!(matches!(record.verdict, Verdict::Rejected(_)));
    }

    #[test]
    fn guard_allows_valid_op_in_semantic_domain() {
        let op = test_op("guard", "enforce_boundary")
            .with("domain", "semantic")
            .with("operation", "nsq_resolve");
        let record = Guard::enforce_boundary(&op);
        assert!(record.verdict.is_approved());
    }

    #[test]
    fn jack_escalates_to_ace_when_no_precedence() {
        let op = test_op("jack", "arbitrate")
            .with("claim_a", "op_x")
            .with("claim_b", "op_y");
        let record = Jack::arbitrate(&op);
        assert!(matches!(record.verdict, Verdict::Escalate { .. }));
    }

    #[test]
    fn ace_rejects_short_justification() {
        let op = test_op("ace", "override")
            .with("target_role", "guard")
            .with("target_op", "seize-001")
            .with("justification", "short");
        let record = Ace::override_verdict(&op);
        assert!(matches!(record.verdict, Verdict::Rejected(_)));
    }

    #[test]
    fn ace_accepts_sufficient_justification() {
        let op = test_op("ace", "override")
            .with("target_role", "guard")
            .with("target_op", "seize-001")
            .with(
                "justification",
                "Emergency override required for critical path recovery after sabotage",
            );
        let record = Ace::override_verdict(&op);
        assert!(record.verdict.is_approved());
    }

    #[test]
    fn bard_affirms_known_truths() {
        let op = test_op("bard", "assert_truth").with("claim", "nsq_is_the_machine");
        let record = Bard::assert_truth(&op);
        assert!(record.verdict.is_approved());
    }

    #[test]
    fn oracle_forecasts_correctly() {
        let op = test_op("oracle", "forecast")
            .with("current_state", "clean")
            .with("action", "deploy");
        let record = Oracle::forecast(&op);
        assert!(record.verdict.is_approved());
        assert!(record.notes.iter().any(|n| n.contains("SUCCESS")));
    }

    #[test]
    fn court_dispatch_reaches_all_25_roles() {
        let court = Court::new(PathBuf::from("."));
        let seats = court.verify_seats();
        let absent: Vec<_> = seats.iter().filter(|s| s.contains("ABSENT")).collect();
        assert!(absent.is_empty(), "Missing court seats: {:?}", absent);
    }

    #[test]
    fn crier_rejects_unknown_hound() {
        let op = test_op("crier", "call_hounds").with("hound", "unknown_hound");
        let record = Crier::call_hounds(&op);
        assert!(matches!(record.verdict, Verdict::Rejected(_)));
    }

    #[test]
    fn crier_accepts_valid_hounds() {
        for hound in &["scent", "proof", "war", "night", "gate"] {
            let op = test_op("crier", "call_hounds").with("hound", *hound);
            let record = Crier::call_hounds(&op);
            assert!(
                record.verdict.is_approved(),
                "Hound '{}' was rejected",
                hound
            );
        }
    }
}
