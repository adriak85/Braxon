use std::fs;
use std::io;
use std::path::Path;

pub fn compose_repo_surface(lines: &[String], out_path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_path, lines.join("\n") + "\n")
}
