fn main() {
    let root = std::env::args()
        .nth(1)
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let regs = nsq_ir::registry_paths(&root);

    println!("nsq-ir");
    println!("version={}", nsq_ir::NSQ_IR_VERSION);
    println!(
        "runtime_domain_registry={}",
        regs.runtime_domain_registry.display()
    );
    println!(
        "graded_selector_registry={}",
        regs.graded_selector_registry.display()
    );
    println!(
        "local_package_repo_registry={}",
        regs.local_package_repo_registry.display()
    );
    println!(
        "package_db_multiport_registry={}",
        regs.package_db_multiport_registry.display()
    );
    println!(
        "human_machine_doc_registry={}",
        regs.human_machine_doc_registry.display()
    );
    println!(
        "tokenizer_bridge_registry={}",
        regs.tokenizer_bridge_registry.display()
    );
    println!(
        "language_master_seed={}",
        regs.language_master_seed.display()
    );
    println!(
        "minimum_integration_registry={}",
        regs.minimum_integration_registry.display()
    );
    println!(
        "translation_priority_registry={}",
        regs.translation_priority_registry.display()
    );
}
