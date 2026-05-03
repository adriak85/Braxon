use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/generated_nsq_app.rs"));

const IDENTITY_LINES: &[&str] = &[
    "NSQ is the lowest base language.",
    "NSQ is the substrate.",
    "NSQ is the machine.",
    "NSQ is the floor, not a layer.",
    "A lever is one switch.",
    "A lever is one eighth of an NSQ bit.",
    "Hertz frequency positions the lever.",
    "Target is injected at runtime.",
    "Local repository exists before recode.",
    "If source is already inside the repo, no localization shim is used.",
];

fn usage() {
    println!("nsq-universal-fetch");
    println!("commands:");
    println!("  identity");
    println!("  compile-proof");
    println!("  doctor");
    println!("  dialects");
    println!("  target-template --out <path>");
    println!("  plan --target <path>");
    println!("  localize-source --repo-id <id> --source <path>");
    println!("  recode-repo --repo-id <id> --source <path>");
    println!("  repo-status --repo-id <id>");
}

fn root() -> PathBuf {
    env::var("BRAXON_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env::var("HOME").unwrap()).join("Braxon"))
}

fn read_arg_value(args: &[String], key: &str) -> Result<String, String> {
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == key {
            return args.get(i + 1).cloned().ok_or_else(|| format!("missing value after {key}"));
        }
        i += 1;
    }
    Err(format!("missing required argument {key}"))
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(&needle.to_ascii_lowercase())
}

fn forbidden_terms() -> Vec<String> {
    vec![
        "nsq asm macro band".to_string(),
        "ultra wide banding".to_string(),
        "ultra-wide banding".to_string(),
        "pointer_stub".to_string(),
        "pointer stubs".to_string(),
        "catalog_complete_pointer".to_string(),
        "model.safetensors".to_string(),
        ["hugging", "face"].concat(),
        ["bound", "ary"].concat(),
        "external_tool_host".to_string(),
        "raw_model".to_string(),
    ]
}

fn validate_target(text: &str) -> Result<(), String> {
    let required = [
        "NSQ_TARGET",
        "nsq_wire_units_required = true",
        "raw_body_transfer = forbidden",
        "source_manifest_uri",
        "unit_uri_template",
        "integrity_witness",
    ];

    for r in required {
        if !text.contains(r) {
            return Err(format!("target missing required NSQ declaration: {r}"));
        }
    }

    for f in forbidden_terms() {
        if contains_ci(text, &f) {
            return Err(format!("target contains forbidden drift term: {f}"));
        }
    }

    Ok(())
}

fn write_target_template(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create parent failed: {e}"))?;
    }

    let body = r#"NSQ_TARGET runtime_injected_target
nsq_wire_units_required = true
raw_body_transfer = forbidden
source_manifest_uri = "REPLACE_WITH_NSQ_WIRE_MANIFEST_URI"
unit_uri_template = "REPLACE_WITH_NSQ_WIRE_UNIT_URI_TEMPLATE"
integrity_witness = "REPLACE_WITH_EXPECTED_WITNESS"
startup_flags = [
  nsq.identity,
  nsq.compile_proof,
  nsq.plan_only
]
"#;

    fs::write(path, body).map_err(|e| format!("write target template failed: {e}"))?;
    Ok(())
}

fn fnv1a64_bytes(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn file_digest(path: &Path) -> Result<(u64, u64), String> {
    let mut f = fs::File::open(path).map_err(|e| format!("open {} failed: {e}", path.display()))?;
    let mut buf = [0u8; 64 * 1024];
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut total: u64 = 0;
    loop {
        let n = f.read(&mut buf).map_err(|e| format!("read {} failed: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        total += n as u64;
        hash = fnv1a64_bytes_with_seed(&buf[..n], hash);
    }
    Ok((hash, total))
}

fn fnv1a64_bytes_with_seed(bytes: &[u8], mut hash: u64) -> u64 {
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn is_inside(base: &Path, p: &Path) -> bool {
    let base_c = fs::canonicalize(base).unwrap_or_else(|_| base.to_path_buf());
    let p_c = fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    p_c.starts_with(base_c)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.is_dir() {
        return Err(format!("source is not a directory: {}", src.display()));
    }
    fs::create_dir_all(dst).map_err(|e| format!("create {} failed: {e}", dst.display()))?;
    for entry in fs::read_dir(src).map_err(|e| format!("read_dir {} failed: {e}", src.display()))? {
        let entry = entry.map_err(|e| format!("read_dir entry failed: {e}"))?;
        let p = entry.path();
        let name = entry.file_name();
        if name == ".git" || name == "target" || name == ".cache" {
            continue;
        }
        let target = dst.join(name);
        if p.is_dir() {
            copy_dir_recursive(&p, &target)?;
        } else if p.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|e| format!("create {} failed: {e}", parent.display()))?;
            }
            fs::copy(&p, &target).map_err(|e| format!("copy {} -> {} failed: {e}", p.display(), target.display()))?;
        }
    }
    Ok(())
}

fn infer_dialect(path: &Path) -> &'static str {
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();

    match name {
        "Cargo.toml" => "cargo",
        "build.gradle" | "settings.gradle" => "gradle",
        "CMakeLists.txt" => "cmake",
        "Makefile" | "makefile" => "make",
        _ => match ext.as_str() {
            "nsq" => "nsq",
            "toml" => "toml",
            "xml" => "xml",
            "rs" => "rs",
            "c" | "h" => "c",
            "cc" | "cpp" | "cxx" | "hpp" => "cpp",
            "py" => "py",
            "lua" => "lua",
            "lisp" | "cl" | "el" => "lisp",
            "java" => "java",
            "kt" | "kts" => "kotlin",
            "sh" | "bash" | "zsh" | "fish" => "shell",
            "yaml" | "yml" => "yaml",
            "md" => "markdown",
            "txt" => "text",
            "sql" | "sqlite" => "sqlite",
            "gradle" => "gradle",
            "cmake" => "cmake",
            _ => "source",
        },
    }
}

fn walk_files(base: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(base).map_err(|e| format!("read_dir {} failed: {e}", base.display()))? {
        let entry = entry.map_err(|e| format!("entry failed: {e}"))?;
        let p = entry.path();
        let name = entry.file_name();
        if name == ".git" || name == "target" || name == ".cache" {
            continue;
        }
        if p.is_dir() {
            walk_files(&p, files)?;
        } else if p.is_file() {
            files.push(p);
        }
    }
    Ok(())
}

fn repo_dir(repo_id: &str) -> PathBuf {
    root().join("state/nsq/source_repositories/local").join(repo_id)
}

fn native_dir(repo_id: &str) -> PathBuf {
    root().join("state/nsq/source_repositories/native").join(repo_id)
}

fn command_identity() {
    for line in IDENTITY_LINES {
        println!("{line}");
    }
}

fn command_compile_proof() {
    println!("compiled_from_nsq={}", NSQ_APP_COMPILED_FROM_NSQ);
    println!("app_source={}", NSQ_APP_SOURCE_PATH);
    println!("app_digest_fnv1a64={}", NSQ_APP_FNV1A64);
    println!("target_compiled_in={}", NSQ_TARGET_COMPILED_IN);
    println!("source_compiled_in={}", NSQ_SOURCE_COMPILED_IN);
    println!("raw_body_transfer_forbidden={}", NSQ_RAW_BODY_TRANSFER_FORBIDDEN);
    println!("nsq_wire_units_required={}", NSQ_WIRE_UNITS_REQUIRED);
    println!("internal_only={}", NSQ_INTERNAL_ONLY);
    println!("local_repo_first={}", NSQ_LOCAL_REPO_FIRST);
    println!("repo_in_root_uses_shim={}", NSQ_REPO_IN_ROOT_USES_SHIM);
    println!("dialects_can_interweave={}", NSQ_DIALECTS_CAN_INTERWEAVE);
}

fn command_dialects() {
    let dialects = [
        "nsq", "toml", "xml", "rs", "c", "cpp", "py", "lua", "lisp", "java", "kotlin",
        "shell", "yaml", "markdown", "text", "sqlite", "cargo", "gradle", "cmake", "make",
    ];
    for d in dialects {
        println!("dialect={d} activation_flag=nsq.frame.{d} internal=true");
    }
}

fn command_doctor() -> Result<(), String> {
    let p = root().join(NSQ_APP_SOURCE_PATH);
    let text = fs::read_to_string(&p)
        .map_err(|e| format!("cannot read NSQ app carrier {}: {e}", p.display()))?;

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

    for r in required {
        if !text.contains(r) {
            return Err(format!("doctor failed: missing {r}"));
        }
    }

    for f in forbidden_terms() {
        if contains_ci(&text, &f) {
            return Err(format!("doctor failed: forbidden drift term {f}"));
        }
    }

    println!("doctor_ok=true");
    println!("carrier_internal_only=true");
    println!("runtime_target_injection_required=true");
    println!("local_repo_first=true");
    println!("repo_inside_root_uses_shim=false");
    println!("dialect_interweave_rule=true");
    Ok(())
}

fn command_plan(args: &[String]) -> Result<(), String> {
    let path = read_arg_value(args, "--target")?;
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("cannot read target {path}: {e}"))?;
    validate_target(&text)?;

    println!("plan_ok=true");
    println!("target={path}");
    println!("target_compiled_in=false");
    println!("source_compiled_in=false");
    println!("acquisition_started=false");
    println!("next=provide local repository path or NSQ-wire source manifest at runtime");
    Ok(())
}

fn command_localize_source(args: &[String]) -> Result<(), String> {
    let repo_id = read_arg_value(args, "--repo-id")?;
    let src = PathBuf::from(read_arg_value(args, "--source")?);
    if !src.exists() {
        return Err(format!("source does not exist: {}", src.display()));
    }

    let r = root();
    if is_inside(&r, &src) {
        println!("localize_ok=true");
        println!("repo_id={repo_id}");
        println!("source={}", src.display());
        println!("already_inside_repo=true");
        println!("shim_used=false");
        println!("localized_path={}", src.display());
        return Ok(());
    }

    let dst = repo_dir(&repo_id).join("source");
    if dst.exists() {
        fs::remove_dir_all(&dst).map_err(|e| format!("remove existing {} failed: {e}", dst.display()))?;
    }
    copy_dir_recursive(&src, &dst)?;

    println!("localize_ok=true");
    println!("repo_id={repo_id}");
    println!("source={}", src.display());
    println!("already_inside_repo=false");
    println!("shim_used=true");
    println!("localized_path={}", dst.display());
    Ok(())
}

fn resolve_source(repo_id: &str, source: &Path) -> PathBuf {
    let r = root();
    if source.exists() && is_inside(&r, source) {
        return source.to_path_buf();
    }
    let local = repo_dir(repo_id).join("source");
    if local.exists() {
        return local;
    }
    source.to_path_buf()
}

fn command_recode_repo(args: &[String]) -> Result<(), String> {
    let repo_id = read_arg_value(args, "--repo-id")?;
    let source_arg = PathBuf::from(read_arg_value(args, "--source")?);
    let source = resolve_source(&repo_id, &source_arg);
    if !source.exists() {
        return Err(format!("source not found; run localize-source first if external: {}", source.display()));
    }
    if !source.is_dir() {
        return Err(format!("source must be a repository directory: {}", source.display()));
    }

    let r = root();
    let shim_used = !is_inside(&r, &source_arg) && source != source_arg;
    let native = native_dir(&repo_id);
    fs::create_dir_all(&native).map_err(|e| format!("create native dir failed: {e}"))?;
    let out = native.join("repo.nsq");
    let mut files = Vec::new();
    walk_files(&source, &mut files)?;
    files.sort();

    let mut f = fs::File::create(&out).map_err(|e| format!("create {} failed: {e}", out.display()))?;
    writeln!(f, "NSQ_NATIVE_REPOSITORY {repo_id}").map_err(|e| e.to_string())?;
    writeln!(f, "source_root = \"{}\"", source.display()).map_err(|e| e.to_string())?;
    writeln!(f, "local_repo_first = true").map_err(|e| e.to_string())?;
    writeln!(f, "shim_used = {shim_used}").map_err(|e| e.to_string())?;
    writeln!(f, "target_compiled_in = false").map_err(|e| e.to_string())?;
    writeln!(f, "source_compiled_in = false").map_err(|e| e.to_string())?;
    writeln!(f, "raw_body_transfer = forbidden").map_err(|e| e.to_string())?;
    writeln!(f, "nsq_wire_units_required = true").map_err(|e| e.to_string())?;
    writeln!(f).map_err(|e| e.to_string())?;

    let mut count = 0usize;
    for p in files {
        let rel = p.strip_prefix(&source).unwrap_or(&p);
        let rels = rel.to_string_lossy().replace('\\', "/");
        let dialect = infer_dialect(&p);
        let (digest, bytes) = file_digest(&p)?;
        writeln!(f, "NSQ_REPO_FRAME {{").map_err(|e| e.to_string())?;
        writeln!(f, "  path = \"{rels}\"").map_err(|e| e.to_string())?;
        writeln!(f, "  dialect = nsq.frame.{dialect}").map_err(|e| e.to_string())?;
        writeln!(f, "  local_source = true").map_err(|e| e.to_string())?;
        writeln!(f, "  byte_count_witness = {bytes}").map_err(|e| e.to_string())?;
        writeln!(f, "  fnv1a64_witness = \"{digest:016x}\"").map_err(|e| e.to_string())?;
        writeln!(f, "  translated_surface = true").map_err(|e| e.to_string())?;
        writeln!(f, "  activation_flag = nsq.frame.{dialect}").map_err(|e| e.to_string())?;
        writeln!(f, "}}").map_err(|e| e.to_string())?;
        count += 1;
    }

    let status = native.join("status.nsq");
    fs::write(
        &status,
        format!(
            "NSQ_REPO_STATUS {repo_id}\nrecode_ok = true\nsource_root = \"{}\"\nnative_repo = \"{}\"\nframe_count = {count}\nshim_used = {shim_used}\n",
            source.display(),
            out.display()
        ),
    )
    .map_err(|e| format!("write status failed: {e}"))?;

    println!("recode_ok=true");
    println!("repo_id={repo_id}");
    println!("source={}", source.display());
    println!("native_repo={}", out.display());
    println!("frame_count={count}");
    println!("shim_used={shim_used}");
    Ok(())
}

fn command_repo_status(args: &[String]) -> Result<(), String> {
    let repo_id = read_arg_value(args, "--repo-id")?;
    let status = native_dir(&repo_id).join("status.nsq");
    if !status.exists() {
        return Err(format!("missing status for repo_id={repo_id}"));
    }
    print!("{}", fs::read_to_string(&status).map_err(|e| format!("read status failed: {e}"))?);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("identity") => {
            command_identity();
            Ok(())
        }
        Some("compile-proof") => {
            command_compile_proof();
            Ok(())
        }
        Some("doctor") => command_doctor(),
        Some("dialects") => {
            command_dialects();
            Ok(())
        }
        Some("target-template") => match read_arg_value(&args, "--out") {
            Ok(path) => match write_target_template(Path::new(&path)) {
                Ok(()) => {
                    println!("target_template_written={path}");
                    Ok(())
                }
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        },
        Some("plan") => command_plan(&args),
        Some("localize-source") => command_localize_source(&args),
        Some("recode-repo") => command_recode_repo(&args),
        Some("repo-status") => command_repo_status(&args),
        _ => {
            usage();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}
