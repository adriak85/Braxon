#[allow(dead_code)]
pub fn runtime_domain_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/runtime_domain_registry.db")
}

#[allow(dead_code)]
pub fn tokenizer_bridge_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/tokenizer_bridge_registry.db")
}

#[allow(dead_code)]
pub fn human_machine_doc_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/runtime_native/databases/human_machine_doc_registry.db")
}

#[allow(dead_code)]
pub fn multiport_package_db(root: &std::path::Path) -> std::path::PathBuf {
    root.join("config/nsq/runtime_native/package_db/multiport_package_db.json")
}

#[allow(dead_code)]
pub fn write_nsq_package_db_binding(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/write_nsq/databases/package_db_binding_registry.db")
}

#[allow(dead_code)]
pub fn translation_priority_registry(root: &std::path::Path) -> std::path::PathBuf {
    root.join("nsq/write_nsq/databases/translation_priority_registry.db")
}
