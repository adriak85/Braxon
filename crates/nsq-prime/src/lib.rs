use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct PrimeReport {
    pub nsq_source_present: bool,
    pub nsq_compile_present: bool,
    pub nsq_pack_present: bool,
    pub nsq_inspect_present: bool,
    pub ok: bool,
}

pub fn prime_report(root: &str) -> PrimeReport {
    let p = Path::new(root);
    let nsq_source_present = p.join("crates/nsq-source/src/lib.rs").exists();
    let nsq_compile_present = p.join("crates/nsq-compile/src/main.rs").exists();
    let nsq_pack_present = p.join("crates/nsq-pack/src/lib.rs").exists();
    let nsq_inspect_present = p.join("crates/nsq-inspect/src/lib.rs").exists();
    let ok = nsq_source_present && nsq_compile_present && nsq_pack_present && nsq_inspect_present;

    PrimeReport {
        nsq_source_present,
        nsq_compile_present,
        nsq_pack_present,
        nsq_inspect_present,
        ok,
    }
}
