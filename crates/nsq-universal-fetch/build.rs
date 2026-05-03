use std::env;
use std::fs;
use std::path::PathBuf;

fn fnv1a64(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in text.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(&needle.to_ascii_lowercase())
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir.join("../..");
    let source_path = root.join("apps/nsq/universal_fetch.nsq");
    let source = fs::read_to_string(&source_path)
        .unwrap_or_else(|e| panic!("failed to read NSQ carrier {}: {e}", source_path.display()));

    let required = [
        "NSQ_CANONICAL_IDENTITY",
        "NSQ_UNIVERSAL_FETCH_AUTHORITY",
        "NSQ_REPOSITORY_LOCALITY_RULE",
        "NSQ_SINGLE_FILE_MULTI_FRAME_RULE",
        "NSQ_DIALECT_COMPATIBILITY",
        "NSQ_STARTUP_FLAGS",
        "repo_already_inside_BRAXON_does_not_use_shim = true",
        "dialects_can_be_interwoven_in_same_document = true",
        "target_compiled_in = false",
        "source_compiled_in = false",
        "raw_body_transfer = forbidden",
        "nsq_wire_units_required = true",
        "all_frames_internal_to_nsq = true",
    ];

    for item in required {
        if !source.contains(item) {
            panic!("NSQ carrier missing required authority item: {item}");
        }
    }

    let forbidden = [
        "nsq asm macro band",
        "ultra wide banding",
        "ultra-wide banding",
        "pointer_stub",
        "pointer stubs",
        "catalog_complete_pointer",
        "model.safetensors",
        concat!("hugging", "face"),
        concat!("bound", "ary"),
        "external_tool_host",
        "raw_model",
    ];

    for item in forbidden {
        if contains_ci(&source, item) {
            panic!("NSQ carrier contains forbidden drift term: {item}");
        }
    }

    let digest = fnv1a64(&source);
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let generated = out_dir.join("generated_nsq_app.rs");

    fs::write(
        &generated,
        format!(
            r#"
pub const NSQ_APP_SOURCE_PATH: &str = "apps/nsq/universal_fetch.nsq";
pub const NSQ_APP_FNV1A64: &str = "{digest:016x}";
pub const NSQ_APP_COMPILED_FROM_NSQ: bool = true;
pub const NSQ_TARGET_COMPILED_IN: bool = false;
pub const NSQ_SOURCE_COMPILED_IN: bool = false;
pub const NSQ_RAW_BODY_TRANSFER_FORBIDDEN: bool = true;
pub const NSQ_WIRE_UNITS_REQUIRED: bool = true;
pub const NSQ_INTERNAL_ONLY: bool = true;
pub const NSQ_LOCAL_REPO_FIRST: bool = true;
pub const NSQ_REPO_IN_ROOT_USES_SHIM: bool = false;
pub const NSQ_DIALECTS_CAN_INTERWEAVE: bool = true;
"#,
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed={}", source_path.display());
}
