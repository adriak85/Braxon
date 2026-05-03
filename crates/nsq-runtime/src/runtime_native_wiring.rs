#[allow(dead_code)]
pub fn runtime_native_root(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native")
}

#[allow(dead_code)]
pub fn runtime_domain_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/runtime_domain_registry.db")
}

#[allow(dead_code)]
pub fn graded_selector_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/graded_selector_registry.db")
}

#[allow(dead_code)]
pub fn local_package_repo_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/local_package_repo_registry.db")
}

#[allow(dead_code)]
pub fn package_db_multiport_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/package_db_multiport_registry.db")
}

#[allow(dead_code)]
pub fn human_machine_doc_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/human_machine_doc_registry.db")
}

#[allow(dead_code)]
pub fn tokenizer_bridge_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/tokenizer_bridge_registry.db")
}

#[allow(dead_code)]
pub fn language_master_seed(root: &std::path::Path) -> std::path::PathBuf {
    root.join("config/nsq/language_master_seed.json")
}

#[allow(dead_code)]
pub fn minimum_integration_registry(root: &std::path::Path) -> std::path::PathBuf {
    runtime_native_root(root).join("databases/minimum_integration_registry.db")
}

#[allow(dead_code)]
pub fn multiport_package_db(root: &std::path::Path) -> std::path::PathBuf {
    root.join("config/nsq/runtime_native/package_db/multiport_package_db.json")
}
