#!/usr/bin/env python3
import sys

BUS_PATH = '/data/data/com.termux/files/home/Braxon/crates/braxon-core/src/bus.rs'

with open(BUS_PATH, 'r') as f:
    src = f.read()

# ── 1. Replace intent_to_english + add try_citadel_reconstruction ─────────────

old_fn = """fn intent_to_english(input: &str, selected: &ThoughtPressureCandidate) -> String {
    format!(
        "It has been a long build, and I am keeping continuity by routing this through the NSQ bus. I read the immediate intent as: {}. The next terminal move is to {}. Source thought: {}",
        selected.intent.replace('_', " "),
        selected.english,
        input
    )
}"""

new_fn = """fn intent_to_english(input: &str, _selected: &ThoughtPressureCandidate) -> String {
    if let Some(reply) = try_citadel_reconstruction(input) {
        return reply;
    }
    format!("[awaiting_reconstruction] {}", input)
}

fn try_citadel_reconstruction(input: &str) -> Option<String> {
    let root = std::env::current_dir().ok()?;
    let script = root.join("tools/citadel699_nsq_request_return_rebuild.sh");
    if !script.exists() {
        return None;
    }
    let output = std::process::Command::new("bash")
        .arg(&script)
        .arg(root.to_str()?)
        .env("BRAXON_BUS_INPUT", input)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).ok()?;
        for line in stdout.lines() {
            if let Some(reply) = line.strip_prefix("REPLY=") {
                let r = reply.trim().to_string();
                if !r.is_empty() {
                    return Some(r);
                }
            }
        }
    }
    None
}"""

if old_fn not in src:
    print("ERROR: intent_to_english pattern not found")
    sys.exit(1)

src = src.replace(old_fn, new_fn)
print("OK: replaced intent_to_english")

# ── 2. Set canned_reply honestly ──────────────────────────────────────────────
# In BraxonBus::speak, english is now either a real reply or [awaiting_reconstruction]
# Set canned_reply = true when reconstruction is pending, false when real

old_canned = """            reply_layer: BusReplyLayer {
                schema: BRAXON_REPLY_SCHEMA.to_string(),
                reply_generated_from_state: true,
                canned_reply: false,
                reply: english.clone(),
            },"""

new_canned = """            reply_layer: BusReplyLayer {
                schema: BRAXON_REPLY_SCHEMA.to_string(),
                reply_generated_from_state: true,
                canned_reply: english.starts_with("[awaiting_reconstruction]"),
                reply: english.clone(),
            },"""

if old_canned not in src:
    print("ERROR: canned_reply block not found")
    sys.exit(1)

src = src.replace(old_canned, new_canned)
print("OK: set canned_reply honestly")

# ── 3. Update tests ────────────────────────────────────────────────────────────

old_test = """    #[test]
    fn speech_loop_launches_to_bus_and_returns_english() {
        let report = BraxonBus::speak("close speech loop and finish terminal tasklist");

        assert_eq!(report.schema, BRAXON_BUS_SCHEMA);
        assert!(report.bus_launched);
        assert!(report.council_ten_wake_passed);
        assert!(report.speech_loop.launched_to_bus);
        assert!(report.speech_loop.one_thought_is_all_thoughts);
        assert!(report.speech_loop.intent_to_english_completed);
        assert!(report.speech_loop.terminal_plan_completed);
        assert!(report.reply_layer.reply_generated_from_state);
        assert!(!report.reply_layer.canned_reply);
        assert!(report.reply_layer.reply.contains("It has been a long build"));
        assert!(report.reply_layer.reply.contains("continuity"));
        assert!(!report.terminal_plan.is_empty());
    }"""

new_test = """    #[test]
    fn speech_loop_launches_to_bus_and_returns_english() {
        let report = BraxonBus::speak("close speech loop and finish terminal tasklist");

        assert_eq!(report.schema, BRAXON_BUS_SCHEMA);
        assert!(report.bus_launched);
        assert!(report.council_ten_wake_passed);
        assert!(report.speech_loop.launched_to_bus);
        assert!(report.speech_loop.one_thought_is_all_thoughts);
        assert!(report.speech_loop.intent_to_english_completed);
        assert!(report.speech_loop.terminal_plan_completed);
        assert!(report.reply_layer.reply_generated_from_state);
        // canned_reply is true when reconstruction is not yet wired, false when live
        assert_eq!(
            report.reply_layer.canned_reply,
            report.reply_layer.reply.starts_with("[awaiting_reconstruction]")
        );
        assert!(!report.reply_layer.reply.is_empty());
        assert!(!report.reply_layer.reply.contains("long build"));
        assert!(!report.terminal_plan.is_empty());
    }"""

if old_test not in src:
    print("ERROR: speech_loop test not found")
    sys.exit(1)

src = src.replace(old_test, new_test)
print("OK: updated speech_loop test")

with open(BUS_PATH, 'w') as f:
    f.write(src)

print("DONE: bus.rs written")
