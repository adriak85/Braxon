pub const NSQ_IR_VERSION: &str = "0.1.0";

#[derive(Debug, Clone)]
pub struct NsqIrRegistryPaths {
    pub runtime_domain_registry: std::path::PathBuf,
    pub graded_selector_registry: std::path::PathBuf,
    pub local_package_repo_registry: std::path::PathBuf,
    pub package_db_multiport_registry: std::path::PathBuf,
    pub human_machine_doc_registry: std::path::PathBuf,
    pub tokenizer_bridge_registry: std::path::PathBuf,
    pub language_master_seed: std::path::PathBuf,
    pub minimum_integration_registry: std::path::PathBuf,
    pub translation_priority_registry: std::path::PathBuf,
}

pub fn registry_paths(root: &std::path::Path) -> NsqIrRegistryPaths {
    NsqIrRegistryPaths {
        runtime_domain_registry: root
            .join("nsq/runtime_native/databases/runtime_domain_registry.db"),
        graded_selector_registry: root
            .join("nsq/runtime_native/databases/graded_selector_registry.db"),
        local_package_repo_registry: root
            .join("nsq/runtime_native/databases/local_package_repo_registry.db"),
        package_db_multiport_registry: root
            .join("nsq/runtime_native/databases/package_db_multiport_registry.db"),
        human_machine_doc_registry: root
            .join("nsq/runtime_native/databases/human_machine_doc_registry.db"),
        tokenizer_bridge_registry: root
            .join("nsq/runtime_native/databases/tokenizer_bridge_registry.db"),
        language_master_seed: root.join("config/nsq/language_master_seed.json"),
        minimum_integration_registry: root
            .join("nsq/runtime_native/databases/minimum_integration_registry.db"),
        translation_priority_registry: root
            .join("nsq/write_nsq/databases/translation_priority_registry.db"),
    }
}

pub fn read_if_present(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
