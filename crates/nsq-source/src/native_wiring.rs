#[allow(dead_code)]
pub fn language_master_seed(root: &std::path::Path) -> std::path::PathBuf {
    root.join("config/nsq/language_master_seed.json")
}

#[allow(dead_code)]
pub fn runtime_domain_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/runtime_domain_registry.db")
}

#[allow(dead_code)]
pub fn graded_selector_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/graded_selector_registry.db")
}

#[allow(dead_code)]
pub fn minimum_integration_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/minimum_integration_registry.db")
}
