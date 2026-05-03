#[allow(dead_code)]
pub fn runtime_domain_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/runtime_domain_registry.db")
}

#[allow(dead_code)]
pub fn package_db_multiport_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/package_db_multiport_registry.db")
}

#[allow(dead_code)]
pub fn human_machine_doc_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/human_machine_doc_registry.db")
}

#[allow(dead_code)]
pub fn tokenizer_bridge_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/tokenizer_bridge_registry.db")
}

#[allow(dead_code)]
pub fn court_canonical(root: &std::path::Path) -> std::path::PathBuf {
    root.join("config/kingdom/court_canonical.json")
}
