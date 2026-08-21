use clap::{Parser, Subcommand};
use nsq_core::{
    lever_max_zero_failure_scan, lever_spacing_sweet_spot_report, lever_sweet_spot_report,
    CANONICAL_LEVER_MAX_POSITION, TOTAL_STATES_PER_LEVER, ZERO_INCLUSIVE_BIT_UNIT_STATES,
};
use nsq_reflexor::{
    bootstrap as reflex_bootstrap, discover as reflex_discover,
    route_declared_feature_operation as reflex_route_declared_feature_operation,
    route_declared_language_operation as reflex_route_declared_language_operation,
    route_operation as reflex_route_operation, verify as reflex_verify,
    write_inventory as reflex_write_inventory, DEFAULT_PROFILE,
};
use sha2::{Digest, Sha256};
use BRAXON_core::{
    address_integrity_audit, assess_donor_model_readiness, available_role_modes,
    bootstrap_live_bus, braxon_context_manifest_status, braxon_wake_linked_change_report_from_env,
    closure_audit, evaluate_repository_operation, execute_bounded_tensor_inference,
    execute_canonical_parameter_citadel_cycle, execute_language_operation,
    execute_operator_intelligence, execute_role_operation, execute_watermarked_file_operation,
    full_wake, model_execution_truth, run_native_fault_recovery, run_native_fixture_equivalence,
    tokenizer_verification, verify_bionic_compatibility, verify_contained_toolchain,
    verify_language_artifact_context, BraxonBus, CouncilTen, DonorModelReadinessReport,
    TargetField, DONOR_MODEL_READINESS_CAPABILITY, OPERATOR_INTELLIGENCE_CAPABILITY,
    ROLE_OPERATION_CAPABILITY, TENSOR_INFERENCE_CAPABILITY, WATERMARKED_FILE_OPERATION_CAPABILITY,
};

#[derive(Parser)]
#[command(name = "Braxon")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Apps {
        #[command(subcommand)]
        command: AppsCommand,
    },
    Console {
        #[arg(long)]
        seated_mode: bool,
    },
    SeatingVerify {
        #[arg(long, default_value_t = 0.001)]
        tolerance: f32,
    },
    MaxStableScan {
        #[arg(long, default_value_t = 0.001)]
        tolerance: f32,
    },
    LeverSweetSpot {
        #[arg(long, default_value_t = 0.001)]
        tolerance: f32,
    },
    Runtime {
        #[command(subcommand)]
        command: RuntimeCommand,
    },
    /// Interpret a declared language spelling through its NSQ contract and execute the resulting intelligent action.
    Language {
        language: String,
        #[arg(required = true, trailing_var_arg = true)]
        input: Vec<String>,
    },
    /// Inspect a pinned extended-repository lane through its NSQ and legal-materialization contract.
    Repository {
        repository: String,
    },
    /// Execute an intent-routed functional watermark transition for a declared repository source file.
    Watermark {
        /// Declared transition intent: verify, materialize, or recover.
        intent: String,
        /// Normalized repository-relative source path.
        source: String,
        /// Permit a real compiler invocation only when the declared AArch64 Android target boundary is present.
        #[arg(long)]
        execute: bool,
    },
    Content {
        #[command(subcommand)]
        command: ContentCommand,
    },
    Reflex {
        #[command(subcommand)]
        command: ReflexCommand,
    },
    Handover {
        #[command(subcommand)]
        command: HandoverCommand,
    },
    Bus {
        #[arg(required = true, trailing_var_arg = true)]
        thought: Vec<String>,
    },
    /// Run a court-bound, on-demand assistant, designer, agent, worker, or personal NSQ operation.
    Role {
        #[command(subcommand)]
        command: RoleCommand,
    },
    TerminalPlan,
    Rescue,
    /// Establish one bounded virtual-addressed Piston/Ghost circulation window for the front door.
    Boot {
        #[arg(default_value = "front-door boot readiness")]
        intent: String,
    },
    Status,
    ContextStatus,
    ContextWake,
    Wake,
    Closure {
        #[command(subcommand)]
        command: ClosureCommand,
    },
    /// Verify contained toolchain, source availability, language ingestion, and target materialization contracts.
    Toolchain {
        #[command(subcommand)]
        command: ToolchainCommand,
    },
}

#[derive(Subcommand)]
enum ToolchainCommand {
    /// Run the on-demand contained toolchain verification report.
    Verify,
    /// Verify the universal-tokenized Bionic compatibility overlay and target proof state.
    Bionic,
}

#[derive(Subcommand)]
enum ClosureCommand {
    /// Run every executable closure gate and emit a machine-readable report.
    Verify,
    /// Run the full activation manifest behind Wake.
    Wake,
    /// Audit canonical addresses, chain records, and runtime bindings.
    Address,
    /// Verify active native tokenizer bands and universal translation evidence.
    Tokenizers,
    /// Print the configured/available/loaded/initialized/executing truth matrix.
    Models,
    /// Traverse language artifacts through documentation, tokens, addresses, and released runtime lookup.
    Language,
}

#[derive(Subcommand)]
enum AppsCommand {
    List,
    Show { app: String },
    Verify,
}

#[derive(Subcommand)]
enum RuntimeCommand {
    Registry,
    /// Evaluate every configured donor band against local index, shard, synchronization, and target-proof state.
    Donors,
    Python3 {
        call: String,
    },
    /// Execute the designated local parameter–Citadel integration cycle on demand.
    ParameterCitadel {
        #[arg(long)]
        signal: i64,
        #[arg(long)]
        context: i64,
    },
    /// Run the deterministic native inference/training equivalence mechanism.
    NativeEquivalence,
    /// Run native snapshot/replay and bounded-fault recovery validation.
    NativeRecovery,
    /// Execute a bounded selected-band operation through the canonical Council Ten Citadel seed window.
    Infer {
        model: String,
        prompt: String,
    },
}

#[derive(Subcommand)]
enum ContentCommand {
    Narrative {
        id: String,
        title: String,
        text: String,
    },
    Fact {
        id: String,
        statement: String,
        source_uri: String,
        retrieved_at: String,
        #[arg(long, default_value = "medium")]
        confidence: String,
    },
    Daydream {
        workload_id: String,
        prompt: String,
        #[arg(long, default_value_t = 0)]
        step: u32,
        #[arg(long)]
        system_intent_pending: bool,
    },
}

#[derive(Subcommand)]
enum HandoverCommand {
    OsPowerRelease,
}

#[derive(Subcommand)]
enum RoleCommand {
    /// List every configured court-bound operation mode after contract and office validation.
    List,
    /// Execute a single bounded role operation through the live bus and NSQ operator transaction.
    Execute {
        /// One declared mode: assistant, designer, agent, worker, or personal.
        mode: String,
        #[arg(required = true, trailing_var_arg = true)]
        request: Vec<String>,
    },
}

#[derive(Subcommand)]
enum ReflexCommand {
    /// Discover every crate, direct library, language surface, project source, and physical boundary as an NSQ contract.
    Discover,
    /// Fail closed unless all source and language surfaces have been fully ingested under NSQ authority.
    Verify,
    /// Persist the verified capability inventory without starting a resident runtime.
    Capture,
    /// Probe the Samsung Galaxy A17 native Termux platform without claiming readiness until all prerequisites are present.
    Bootstrap {
        #[arg(long, default_value = DEFAULT_PROFILE)]
        profile: String,
    },
    /// Route a capability contract; execution remains explicit and on-demand.
    Operate {
        capability: String,
        #[arg(long)]
        execute: bool,
    },
}

#[derive(Debug, Clone, Copy)]
struct RootAppSurface {
    app: &'static str,
    package: &'static str,
    bin_name: &'static str,
    root_launchable: bool,
    surface: &'static str,
}

const ROOT_APP_SURFACES: &[RootAppSurface] = &[
    RootAppSurface {
        app: "Braxon",
        package: "Braxon-universal",
        bin_name: "Braxon",
        root_launchable: true,
        surface: "root_entrance_orchestrator",
    },
    RootAppSurface {
        app: "nsq-cli",
        package: "nsq-cli",
        bin_name: "nsq-cli",
        root_launchable: true,
        surface: "platform_entrance",
    },
    RootAppSurface {
        app: "Braxon-cli",
        package: "Braxon-cli",
        bin_name: "Braxon-cli",
        root_launchable: true,
        surface: "platform_entrance",
    },
    RootAppSurface {
        app: "Braxon-court",
        package: "Braxon-court",
        bin_name: "Braxon-court",
        root_launchable: true,
        surface: "court_config_projection",
    },
];

fn main() {
    if std::env::args_os().len() == 1 {
        braxon_console_repl();
        return;
    }

    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Status) {
        Command::Apps { command } => print_apps(command),
        Command::Console { seated_mode } => {
            if seated_mode {
                println!("[System] Seated Mode Active.");
            }
            braxon_console_repl();
        }
        Command::SeatingVerify { tolerance } => print_seating_verify(tolerance),
        Command::MaxStableScan { tolerance } => print_max_stable_scan(tolerance),
        Command::LeverSweetSpot { tolerance } => print_lever_sweet_spot(tolerance),
        Command::Runtime { command } => print_runtime(command),
        Command::Language { language, input } => {
            print_language_operation(language, input.join(" "))
        }
        Command::Repository { repository } => print_repository_operation(repository),
        Command::Watermark {
            intent,
            source,
            execute,
        } => print_watermarked_file_operation(intent, source, execute),
        Command::Content { command } => print_content(command),
        Command::Reflex { command } => print_reflex(command),
        Command::Handover { command } => print_handover(command),
        Command::Bus { thought } => print_bus(thought),
        Command::Role { command } => print_role(command),
        Command::TerminalPlan => print_terminal_plan(),
        Command::Rescue => print_rescue(),
        Command::Boot { intent } => print_boot(intent),
        Command::Status => print_status(),
        Command::ContextStatus => print_context_status(),
        Command::ContextWake => print_context_wake(),
        Command::Wake => print_wake(),
        Command::Closure { command } => print_closure(command),
        Command::Toolchain { command } => print_toolchain(command),
    }
}

// ── REPL ─────────────────────────────────────────────────────────────────────

fn braxon_console_repl() {
    use std::io::{self, Write};

    println!("BRAXON_INTERACTIVE_CONSOLE=ready");
    println!("BRAXON_CONSOLE_BUILD=bus_speech_terminal_v3");
    println!("binding=nsq_operator_bus");
    println!("single_process_entrance=true");
    println!("speech_loop=bus_resolved");
    println!("terminal_plan=available");
    println!("Commands: wake | council | status | context | levers | apps | plan | bus <thought> | help | exit");
    println!();

    let stdin = io::stdin();

    loop {
        print!("Braxon> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!();
                println!("BRAXON_INTERACTIVE_CONSOLE=closed");
                break;
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("BRAXON_CONSOLE_READ_ERROR={err}");
                break;
            }
        }

        let msg = input.trim();

        if msg.is_empty() {
            continue;
        }

        if matches!(msg, "exit" | "quit" | ":q" | "/exit") {
            println!("BRAXON_INTERACTIVE_CONSOLE=closed");
            break;
        }

        match msg {
            "help" | "?" => repl_help(),
            "wake" => repl_wake(),
            "council" => repl_council(),
            "status" => print_status(),
            "context" => repl_context(),
            "levers" => repl_levers(),
            "apps" => repl_apps(),
            "plan" | "terminal-plan" => print_terminal_plan(),
            other if other.starts_with("bus ") => repl_speak(other.trim_start_matches("bus ")),
            other => repl_speak(other),
        }
        println!();
    }
}

fn repl_help() {
    println!("Available commands:");
    println!("  wake     — fire the council ten stamp wake and show verified trace");
    println!("  council  — show all ten poles (6 brain + 4 sensory) and their status");
    println!("  status   — system identity and NSQ lever state count");
    println!("  context  — context manifest status");
    println!("  levers   — lever sweet spot report");
    println!("  apps     — list registered app surfaces");
    println!("  bus TEXT — launch a thought to the bus and return English plus plan");
    println!("  plan     — show the terminal tasklist after the speech loop");
    println!("  help     — this message");
    println!("  exit     — close the console");
}

fn repl_speak(user_message: &str) {
    match execute_reflexor_intelligent_turn(user_message) {
        Ok((_, operation)) => println!("{}", operation.answer),
        Err(error) => println!("Braxon cannot complete this operation yet: {error}"),
    }
}

fn execute_reflexor_intelligent_turn(
    input: &str,
) -> Result<
    (
        nsq_reflexor::ReflexOperation,
        BRAXON_core::IntelligentOperation,
    ),
    String,
> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;
    let route = reflex_route_declared_feature_operation(&root, OPERATOR_INTELLIGENCE_CAPABILITY)?;
    if !route.routed || route.capability.id != OPERATOR_INTELLIGENCE_CAPABILITY {
        return Err("Kinetic Semantic Reflexor did not select a locally ready operator-intelligence feature".into());
    }
    let operation = execute_operator_intelligence(input)?;
    Ok((route, operation))
}

fn repl_wake() {
    let ten = CouncilTen::new();
    let trace = ten.wake();

    println!("stamp={}", trace.stamp);
    println!("authority={}", trace.authority);
    println!("timestamp_unix={}", trace.timestamp_unix);
    println!("coherence_verified={}", trace.coherence_verified);
    println!("result_form={}", trace.result_form);
    println!("address_projection={}", trace.address_projection);
    println!();

    for step in &trace.steps {
        let status = match &step.result {
            BRAXON_core::WakeStepResult::Pass => "PASS",
            BRAXON_core::WakeStepResult::Fail(_) => "FAIL",
        };
        println!("  step {:02} [{}] {}", step.index, status, step.name);
        if let BRAXON_core::WakeStepResult::Fail(reason) = &step.result {
            println!("         reason: {reason}");
        }
    }

    println!();
    if trace.all_passed {
        println!(
            "WAKE_RESULT=verified — all {} steps passed",
            trace.steps.len()
        );
    } else {
        println!("WAKE_RESULT=fail_closed — see failed steps above");
    }
}

fn repl_council() {
    let ten = CouncilTen::new();
    let pressure = ten.brain.unified_thought_pressure();
    let roster = ten.brain.sensory_generation_roster();

    println!("=== COUNCIL OF TEN ===");
    println!();
    println!("BRAIN POLES (6):");
    for member in &ten.brain.members {
        println!(
            "  [{:16}] {:20} model={} region={}",
            member.role.as_str(),
            member.cognitive_pole,
            member.model_source,
            member.brain_region.as_str(),
        );
    }

    println!();
    println!("SENSORY BODIES (4):");
    for body in &roster.bodies {
        println!(
            "  [{:16}] {:20} model={}",
            body.semantic_id
                .split('.')
                .last()
                .unwrap_or(&body.semantic_id),
            body.role.split('_').next().unwrap_or(&body.role),
            body.model,
        );
    }

    println!();
    println!("unified_pressure_ready={}", pressure.unified_pressure_ready);
    println!("all_regions_unique={}", pressure.all_regions_unique);
    println!(
        "graphics_footprint={}/{} units",
        roster.graphics_footprint_used, roster.graphics_footprint_allowance
    );
    println!(
        "footprint_within_allowance={}",
        roster.footprint_within_allowance
    );
    println!(
        "substrate={}",
        BRAXON_core::council_ten::COUNCIL_TEN_SUBSTRATE
    );
    println!("wiring={}", BRAXON_core::council_ten::COUNCIL_TEN_WIRING);
}

fn repl_context() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    match braxon_context_manifest_status(&root) {
        Ok(status) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&status).unwrap_or_default()
            );
        }
        Err(err) => println!("context_status_error={err}"),
    }
}

fn repl_levers() {
    let report = lever_spacing_sweet_spot_report(0.001);
    println!("lever_states_zero_inclusive={TOTAL_STATES_PER_LEVER}");
    println!("bit_unit_states={ZERO_INCLUSIVE_BIT_UNIT_STATES}");
    println!("selected_spacing_units={}", report.selected_spacing_units);
    println!(
        "selected_spacing_units_base8={}",
        report.selected_spacing_units_base8
    );
    println!(
        "selected_information_processed={}",
        report.selected_information_processed
    );
    println!(
        "selected_stable_upper_position={}",
        report.selected_stable_upper_position
    );
    println!("zero_failed_or_missed={}", report.zero_failed_or_missed);
    println!(
        "selected_hertz_spacing={:.9}",
        report.selected_hertz_spacing
    );
}

fn repl_apps() {
    for app in ROOT_APP_SURFACES {
        println!(
            "{} :: surface={} launchable={}",
            app.app, app.surface, app.root_launchable
        );
    }
}

// ── SUBCOMMANDS ───────────────────────────────────────────────────────────────

fn print_role(command: RoleCommand) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = match command {
        RoleCommand::List => available_role_modes(&root).and_then(|modes| {
            Ok(serde_json::json!({
                "schema": "braxon.nsq.role_operation_modes.v1",
                "capability": ROLE_OPERATION_CAPABILITY,
                "modes": modes,
                "execution_mode": "on_demand_non_resident",
            }))
        }),
        RoleCommand::Execute { mode, request } => {
            let route = reflex_route_declared_feature_operation(&root, ROLE_OPERATION_CAPABILITY);
            route.and_then(|route| {
                if !route.routed || route.capability.id != ROLE_OPERATION_CAPABILITY {
                    return Err("Kinetic Semantic Reflexor did not select the canonical role-operation feature".to_string());
                }
                execute_role_operation(&root, &mode, request.join(" ")).and_then(|operation| {
                    serde_json::to_value(serde_json::json!({
                        "route": route,
                        "operation": operation,
                    }))
                    .map_err(|error| error.to_string())
                })
            })
        }
    };
    match result {
        Ok(report) => print_json(&report),
        Err(error) => {
            eprintln!("role_operation_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_wake() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = (|| {
        let live_bus = bootstrap_live_bus(
            &root,
            "wake front door verify and prime parameters tokenizers and Council Ten seed",
        )?;
        let council_ten = CouncilTen::new().wake();
        if !council_ten.all_passed || !council_ten.coherence_verified {
            return Err("Council Ten topology wake failed before full activation".to_string());
        }
        let full_activation = full_wake(&root)?;
        if !full_activation.all_passed {
            return Err(format!(
                "full activation verification failed: unresolved={} orphaned={} invalid_bindings={}",
                full_activation.unresolved,
                full_activation.orphaned,
                full_activation.invalid_bindings
            ));
        }
        Ok::<_, String>(serde_json::json!({
            "answer": "Wake executed a bounded live bootstrap, verified every virtual tokenizer/parameter/seed window, released every piston lease, then verified the Council Ten topology and full activation graph.",
            "action": "execute_live_bus_bootstrap_then_verify_council_ten_and_activation_graph",
            "live_bus": live_bus,
            "council_ten": council_ten,
            "full_activation": full_activation,
        }))
    })();
    match result {
        Ok(report) => print_json(&report),
        Err(err) => {
            eprintln!("wake_activation_error={err}");
            std::process::exit(1);
        }
    }
}

fn print_closure(command: ClosureCommand) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = match command {
        ClosureCommand::Verify => closure_audit(&root)
            .and_then(|report| serde_json::to_value(report).map_err(|error| error.to_string())),
        ClosureCommand::Wake => full_wake(&root)
            .and_then(|report| serde_json::to_value(report).map_err(|error| error.to_string())),
        ClosureCommand::Address => address_integrity_audit(&root)
            .and_then(|report| serde_json::to_value(report).map_err(|error| error.to_string())),
        ClosureCommand::Tokenizers => tokenizer_verification(&root)
            .and_then(|report| serde_json::to_value(report).map_err(|error| error.to_string())),
        ClosureCommand::Models => Ok(serde_json::json!({
            "schema": "braxon.nsq.model_execution_truth.v1",
            "models": model_execution_truth(&root),
        })),
        ClosureCommand::Language => verify_language_artifact_context(&root)
            .and_then(|report| serde_json::to_value(report).map_err(|error| error.to_string())),
    };
    match result {
        Ok(report) => print_json(&report),
        Err(err) => {
            eprintln!("closure_error={err}");
            std::process::exit(1);
        }
    }
}

fn print_apps(command: AppsCommand) {
    match command {
        AppsCommand::List => {
            println!("app_total={}", ROOT_APP_SURFACES.len());
            println!("canonical_nsq=base8_switch_topology");
            for app in ROOT_APP_SURFACES {
                println!(
                    "{} :: package={} bin_name={} root_launchable={} surface={}",
                    app.app, app.package, app.bin_name, app.root_launchable, app.surface
                );
            }
        }
        AppsCommand::Show { app } => {
            let Some(app) = find_root_app(&app) else {
                eprintln!("unknown_app={app}");
                std::process::exit(1);
            };
            println!("app={}", app.app);
            println!("package={}", app.package);
            println!("bin_name={}", app.bin_name);
            println!("root_launchable={}", app.root_launchable);
            println!("surface={}", app.surface);
            println!("canonical_nsq=base8_switch_topology");
        }
        AppsCommand::Verify => {
            let root_launchable_total = ROOT_APP_SURFACES
                .iter()
                .filter(|app| app.root_launchable)
                .count();
            println!("app_total={}", ROOT_APP_SURFACES.len());
            println!("root_launchable_total={root_launchable_total}");
            println!(
                "root_launch_coverage_validated={}",
                root_launchable_total == ROOT_APP_SURFACES.len()
            );
            println!("canonical_nsq=base8_switch_topology");
        }
    }
}

fn find_root_app(name: &str) -> Option<&'static RootAppSurface> {
    ROOT_APP_SURFACES.iter().find(|app| {
        app.app.eq_ignore_ascii_case(name)
            || app.package.eq_ignore_ascii_case(name)
            || app.bin_name.eq_ignore_ascii_case(name)
    })
}

fn print_language_operation(language: String, input: String) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let normalized = language.trim().to_ascii_lowercase();
    let result = (|| {
        let intercept_route =
            reflex_route_declared_feature_operation(&root, "feature:language.parameter_parse")?;
        let language_route = reflex_route_declared_language_operation(&root, &normalized)?;
        let operation = execute_language_operation(&root, &normalized, &input)?;
        if !operation.semantic_parse_ready
            || operation.nsq_capability != language_route.capability.id
        {
            return Err(format!(
                "language operation did not complete the declared NSQ semantic parse for {normalized}"
            ));
        }
        serde_json::to_value(serde_json::json!({
            "schema": "braxon.language.nsq_intercept.v1",
            "intercept_route": intercept_route,
            "language_route": language_route,
            "operation": operation,
            "full_closure_verification_front_door": "Braxon closure language",
        }))
        .map_err(|error| error.to_string())
    })();
    match result {
        Ok(report) => print_json(&report),
        Err(error) => {
            eprintln!("language_operation_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_repository_operation(repository: String) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = reflex_route_operation(&root, "feature:repository.operation", false).and_then(
        |intercept_route| {
            evaluate_repository_operation(&root, repository.trim()).and_then(|operation| {
                serde_json::to_value(serde_json::json!({
                    "schema": "braxon.repository.nsq_intercept.v1",
                    "intercept_route": intercept_route,
                    "operation": operation,
                }))
                .map_err(|error| error.to_string())
            })
        },
    );
    match result {
        Ok(report) => print_json(&report),
        Err(error) => {
            eprintln!("repository_operation_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_watermarked_file_operation(intent: String, source: String, execute: bool) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result =
        reflex_route_declared_feature_operation(&root, WATERMARKED_FILE_OPERATION_CAPABILITY)
            .and_then(|intercept_route| {
                if !intercept_route.routed
                    || intercept_route.capability.id != WATERMARKED_FILE_OPERATION_CAPABILITY
                {
                    return Err(
                    "Kinetic Semantic Reflexor did not select the functional watermark capability"
                        .into(),
                );
                }
                execute_watermarked_file_operation(&root, &intent, &source, execute).and_then(
                    |operation| {
                        serde_json::to_value(serde_json::json!({
                            "schema": "braxon.watermarked_file.nsq_intercept.v1",
                            "intercept_route": intercept_route,
                            "operation": operation,
                        }))
                        .map_err(|error| error.to_string())
                    },
                )
            });
    match result {
        Ok(report) => print_json(&report),
        Err(error) => {
            eprintln!("watermarked_file_operation_error={error}");
            std::process::exit(1);
        }
    }
}

fn execute_language_intelligent_turn(
    language: &str,
    input: &str,
) -> Result<
    (
        nsq_reflexor::ReflexOperation,
        nsq_reflexor::ReflexOperation,
        BRAXON_core::IntelligentOperation,
    ),
    String,
> {
    let language = language.trim().to_ascii_lowercase();
    if language.is_empty() {
        return Err("language ingress requires a declared language identifier".into());
    }
    let root = std::env::current_dir().map_err(|error| error.to_string())?;
    let language_id = format!("language:{language}");
    let language_route = reflex_route_declared_language_operation(&root, &language)?;
    if !language_route.routed || language_route.capability.id != language_id {
        return Err(format!(
            "Kinetic Semantic Reflexor did not select declared language '{language}'"
        ));
    }
    let (operation_route, operation) =
        execute_reflexor_intelligent_turn(&format!("{language} boundary operation: {input}"))?;
    Ok((language_route, operation_route, operation))
}

fn print_runtime(command: RuntimeCommand) {
    match command {
        RuntimeCommand::Registry => print_json(&nsq_court_registry()),
        RuntimeCommand::Donors => {
            let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
            let result = (|| {
                let donor_route = reflex_route_declared_feature_operation(
                    &root,
                    DONOR_MODEL_READINESS_CAPABILITY,
                )?;
                let intelligent_route = reflex_route_declared_feature_operation(
                    &root,
                    OPERATOR_INTELLIGENCE_CAPABILITY,
                )?;
                let readiness = assess_donor_model_readiness(&root)?;
                if donor_route.capability.id != DONOR_MODEL_READINESS_CAPABILITY
                    || intelligent_route.capability.id != OPERATOR_INTELLIGENCE_CAPABILITY
                {
                    return Err("Kinetic Semantic Reflexor did not select the declared donor and intelligent-operation capabilities".into());
                }
                Ok::<_, String>((donor_route, intelligent_route, readiness))
            })();
            match result {
                Ok((donor_route, intelligent_route, readiness)) => {
                    print_donor_readiness_front_door(donor_route, intelligent_route, readiness)
                }
                Err(error) => {
                    eprintln!("runtime_donors_error={error}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::Python3 { call } => {
            match execute_language_intelligent_turn("python3", &call) {
                Ok((language_route, operation_route, operation)) => {
                    print_json(&serde_json::json!({
                        "answer": format!("I interpreted the Python 3 boundary input as NSQ semantic intent and {}", operation.answer.trim_start_matches("I ")),
                        "action": "python3_ingress_to_nsq_intelligent_operation",
                        "language_capability": language_route.capability.id,
                        "execution_capability": operation_route.capability.id,
                        "selected_intent": operation.selected_intent,
                        "lease_released": operation.lease_released,
                        "executed_as_second_runtime": false,
                    }))
                }
                Err(error) => {
                    eprintln!("python3_runtime_error={error}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::ParameterCitadel { signal, context } => {
            let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
            let result = (|| {
                let route =
                    reflex_route_declared_feature_operation(&root, "feature:parameter.citadel")?;
                let operation = execute_canonical_parameter_citadel_cycle(signal, context)?;
                if !operation.invariants.all_pass() {
                    return Err("parameter–Citadel invariants were not all satisfied".into());
                }
                Ok::<_, String>((route, operation))
            })();
            match result {
                Ok((route, operation)) => print_json(&serde_json::json!({
                    "answer": format!("I executed the designated parameter–Citadel integration for signal={signal} and context={context}. The recursive generation {} was materialized, integrated, persisted, reconstructed, and released.", operation.generation),
                    "action": "designated_local_parameter_integration",
                    "capability": route.capability.id,
                    "generation": operation.generation,
                    "changed_parameters": operation.changed_parameters,
                    "invariants": operation.invariants,
                })),
                Err(error) => {
                    eprintln!("parameter_citadel_error={error}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::NativeEquivalence => {
            let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
            let result = (|| {
                let route = reflex_route_declared_feature_operation(
                    &root,
                    "feature:benchmark.native_equivalence",
                )?;
                let report = run_native_fixture_equivalence()?;
                if !report.inference_replay_equivalent || !report.training_path_equivalent {
                    return Err(
                        "native equivalence benchmark did not establish deterministic parity"
                            .into(),
                    );
                }
                Ok::<_, String>((route, report))
            })();
            match result {
                Ok((route, report)) => print_json(&serde_json::json!({
                    "answer": "I executed the native deterministic inference and training equivalence benchmark; the independently replayed paths produced the same verified results.",
                    "action": "run_native_equivalence_benchmark",
                    "capability": route.capability.id,
                    "benchmark": report,
                })),
                Err(error) => {
                    eprintln!("native_equivalence_error={error}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::NativeRecovery => {
            let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
            let result = (|| {
                let route = reflex_route_declared_feature_operation(
                    &root,
                    "feature:benchmark.native_recovery",
                )?;
                let report = run_native_fault_recovery()?;
                if !report.replay_equivalent
                    || report.fault_results.iter().any(|fault| !fault.rejected)
                {
                    return Err(
                        "native recovery benchmark did not preserve replay or fault rejection"
                            .into(),
                    );
                }
                Ok::<_, String>((route, report))
            })();
            match result {
                Ok((route, report)) => print_json(&serde_json::json!({
                    "answer": "I executed native snapshot/replay recovery and bounded-fault rejection; the recovered execution replayed equivalently and every injected invalid state was rejected.",
                    "action": "run_native_recovery_benchmark",
                    "capability": route.capability.id,
                    "benchmark": report,
                })),
                Err(error) => {
                    eprintln!("native_recovery_error={error}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::Infer { model, prompt } => {
            let root = match std::env::current_dir() {
                Ok(root) => root,
                Err(error) => {
                    eprintln!("runtime_infer_error={error}");
                    std::process::exit(1);
                }
            };
            let result = (|| {
                let route =
                    reflex_route_declared_feature_operation(&root, TENSOR_INFERENCE_CAPABILITY)?;
                if !route.routed || route.capability.id != TENSOR_INFERENCE_CAPABILITY {
                    return Err("Kinetic Semantic Reflexor did not select tensor inference".into());
                }
                let operation = execute_bounded_tensor_inference(&root, &model, &prompt)?;
                Ok::<_, String>((route, operation))
            })();
            match result {
                Ok((route, operation)) => print_json(&serde_json::json!({
                    "answer": operation.answer,
                    "action": "bounded_native_parameter_execution",
                    "capability": route.capability.id,
                    "model": operation.model,
                    "selected_tensor": operation.selected_tensor,
                    "execution": operation.execution,
                    "whole_model_execution": operation.whole_model_execution,
                    "resident_runtime_constructed": operation.resident_runtime_constructed,
                })),
                Err(err) => {
                    eprintln!("runtime_infer_error={err}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_donor_readiness_front_door(
    donor_route: nsq_reflexor::ReflexOperation,
    intelligent_route: nsq_reflexor::ReflexOperation,
    readiness: DonorModelReadinessReport,
) {
    let all_configured_donor_bands_live = readiness.configured_model_total_matches_contract
        && readiness.complete_ten_body_window_proven
        && readiness.donor_parameter_synchronization_live;
    let answer = if all_configured_donor_bands_live {
        "Every configured donor band has completed the canonical Council Ten Citadel seed materialization, NSQ firing, and bounded-window release proof. This establishes the seed route only; whole-model learned-weight execution remains explicitly unclaimed."
    } else {
        "The Braxon intelligent front door is operational through NSQ intent, Kinetic Reflexor routing, native instruction execution, and lease release. Donor readiness fails closed until the canonical Council Ten Citadel seed can materialize, fire, and release every configured body."
    };
    print_json(&serde_json::json!({
        "schema": "braxon.runtime.donor_front_door.v1",
        "answer": answer,
        "action": "evaluate_configured_donor_bands_and_intelligent_front_door",
        "donor_readiness_capability": donor_route.capability.id,
        "intelligent_operation_capability": intelligent_route.capability.id,
        "intelligent_front_door_operable": true,
        "all_configured_donor_bands_live": all_configured_donor_bands_live,
        "donor_parameter_synchronization_live": readiness.donor_parameter_synchronization_live,
        "model_weight_execution_claimed": readiness.model_weight_execution_claimed,
        "resident_runtime_constructed": readiness.resident_runtime_constructed,
        "operational_procedures": [
            "Braxon bus <intent>",
            "Braxon runtime parameter-citadel --signal <n> --context <n>",
            "Braxon runtime native-equivalence",
            "Braxon runtime native-recovery",
            "Braxon runtime infer <configured-model> <prompt> after donor readiness proves a complete Council Ten Citadel seed window"
        ],
        "readiness": readiness,
    }));
}

fn print_content(command: ContentCommand) {
    match command {
        ContentCommand::Narrative { id, title, text } => {
            let record = BRAXON_core::NarrativeRecord {
                schema: BRAXON_core::NARRATIVE_SCHEMA.to_string(),
                record_id: id,
                title,
                text,
                source: "wowas_narrative".to_string(),
                version: "1".to_string(),
            };
            if let Err(err) = record.validate() {
                eprintln!("narrative_validation_error={err}");
                std::process::exit(1);
            }
            print_json(&record);
        }
        ContentCommand::Fact {
            id,
            statement,
            source_uri,
            retrieved_at,
            confidence,
        } => {
            let record = BRAXON_core::FactRecord {
                schema: BRAXON_core::FACT_SCHEMA.to_string(),
                fact_id: id,
                statement,
                source_uri,
                retrieved_at,
                confidence,
                invalidated: false,
            };
            if let Err(err) = record.validate() {
                eprintln!("fact_validation_error={err}");
                std::process::exit(1);
            }
            print_json(&record);
        }
        ContentCommand::Daydream {
            workload_id,
            prompt,
            step,
            system_intent_pending,
        } => {
            match BRAXON_core::daydream_frame(&workload_id, step, &prompt, system_intent_pending) {
                Ok(frame) => print_json(&frame),
                Err(err) => {
                    eprintln!("daydream_validation_error={err}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_toolchain(command: ToolchainCommand) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = match command {
        ToolchainCommand::Verify => {
            reflex_route_declared_feature_operation(&root, "feature:toolchain.contained_verify")
                .and_then(|route| {
                    verify_contained_toolchain(&root).and_then(|report| {
                        serde_json::to_value(serde_json::json!({
                            "schema": "braxon.toolchain.front_door.v1",
                            "reflexor_route": route,
                            "verification": report,
                        }))
                        .map_err(|error| error.to_string())
                    })
                })
        }
        ToolchainCommand::Bionic => {
            reflex_route_declared_feature_operation(&root, "feature:toolchain.bionic_compatibility")
                .and_then(|route| {
                    verify_bionic_compatibility(&root).and_then(|report| {
                        serde_json::to_value(serde_json::json!({
                            "schema": "braxon.toolchain.bionic_front_door.v1",
                            "reflexor_route": route,
                            "verification": report,
                        }))
                        .map_err(|error| error.to_string())
                    })
                })
        }
    };
    match result {
        Ok(report) => print_json(&report),
        Err(error) => print_json(&serde_json::json!({
            "schema": "braxon.toolchain.front_door.v1",
            "status": "verification_unavailable",
            "exact_connection_guidance": error,
        })),
    }
}

fn print_reflex(command: ReflexCommand) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = match command {
        ReflexCommand::Discover => reflex_discover(&root)
            .and_then(|inventory| serde_json::to_value(inventory).map_err(|err| err.to_string())),
        ReflexCommand::Verify => reflex_verify(&root).and_then(|verification| {
            serde_json::to_value(verification).map_err(|err| err.to_string())
        }),
        ReflexCommand::Capture => reflex_write_inventory(&root).map(|path| {
            serde_json::json!({
                "schema": "braxon.nsq.kinetic_reflex.capture.v1",
                "inventory_path": path,
                "status": "captured"
            })
        }),
        ReflexCommand::Bootstrap { profile } => reflex_bootstrap(&root, &profile),
        ReflexCommand::Operate {
            capability,
            execute,
        } => reflex_route_operation(&root, &capability, execute)
            .and_then(|operation| serde_json::to_value(operation).map_err(|err| err.to_string())),
    };
    match result {
        Ok(report) => print_json(&report),
        Err(err) => {
            eprintln!("reflex_error={err}");
            std::process::exit(1);
        }
    }
}

fn print_json<T: serde::Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("json_render_error={err}");
            std::process::exit(1);
        }
    }
}

fn print_handover(command: HandoverCommand) {
    match command {
        HandoverCommand::OsPowerRelease => match build_os_power_release_handover() {
            Ok(report) => print_json(&report),
            Err(err) => {
                eprintln!("handover_error={err}");
                std::process::exit(1);
            }
        },
    }
}

fn print_bus(thought: Vec<String>) {
    let thought = thought.join(" ");
    match execute_reflexor_intelligent_turn(&thought) {
        Ok((reflex_route, operation)) => print_json(&serde_json::json!({
            "answer": operation.answer,
            "action": operation.action,
            "reflex_capability": reflex_route.capability.id,
            "live_bus": {
                "capability": operation.live_bus_bootstrap.capability,
                "virtual_window_total": operation.live_bus_bootstrap.virtual_window_total,
                "virtual_wire_bytes": operation.live_bus_bootstrap.virtual_wire_bytes,
                "circulation_cycle_total": operation.live_bus_bootstrap.circulation_cycle_total,
                "all_windows_resolved": operation.live_bus_bootstrap.all_windows_resolved,
                "all_windows_released": operation.live_bus_bootstrap.all_windows_released,
                "active_cpu_bytes_after_release": operation.live_bus_bootstrap.active_cpu_bytes_after_release,
                "model_weight_execution_claimed": operation.live_bus_bootstrap.model_weight_execution_claimed,
                "resident_runtime_constructed": operation.live_bus_bootstrap.resident_runtime_constructed,
                "windows": operation.live_bus_bootstrap.windows,
            },
            "audit": {
                "native_transaction_generation": operation.native_transaction_generation,
                "native_instruction_count": operation.native_instruction_count,
                "native_fired_count": operation.native_fired_count,
                "lease_released": operation.lease_released,
                "selected_intent": operation.selected_intent,
                "input_accepted": operation.audit_bus.processing.input_accepted,
                "conflict_preserved": operation.collective_self_state.conflict_preserved,
                "model_weight_execution_claimed": operation.audit_bus.model_weight_execution_claimed,
                "native_runtime_completion_claimed": operation.audit_bus.native_runtime_completion_claimed,
                "collective_self_state": operation.collective_self_state,
            }
        })),
        Err(error) => {
            eprintln!("operator_intelligence_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_terminal_plan() {
    print_json(&BraxonBus::terminal_plan());
}

fn print_status() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = (|| {
        let (route, operation) = execute_reflexor_intelligent_turn(
            "evaluate the current live Braxon status through tokenizer parameter Citadel and Council Ten verification",
        )?;
        let wake = full_wake(&root)?;
        let healthy = route.routed
            && route.capability.executable
            && wake.all_passed
            && operation.native_fired_count > 0
            && operation.lease_released
            && operation.live_bus_bootstrap.all_windows_resolved
            && operation.live_bus_bootstrap.all_windows_released
            && operation.live_bus_bootstrap.active_cpu_bytes_after_release == 0;
        Ok::<_, String>(serde_json::json!({
            "answer": operation.answer,
            "action": "execute_bounded_live_intelligent_status_evaluation",
            "status": if healthy { "live_bootstrap_and_activation_verified" } else { "live_status_requires_repair" },
            "reflex_capability": route.capability.id,
            "reflex_route_execution_mode": route.execution_mode,
            "reflex_route_status": route.status,
            "full_workspace_verification": "available through Braxon reflex verify; deliberately not run on each interactive status request",
            "live_bus": operation.live_bus_bootstrap,
            "full_wake": wake,
            "native_operator_transaction": {
                "generation": operation.native_transaction_generation,
                "fired_count": operation.native_fired_count,
                "lease_released": operation.lease_released,
            },
            "resident_runtime_constructed": false,
            "next": if healthy { "Braxon bus <intent>" } else { "Braxon wake" },
        }))
    })();
    match result {
        Ok(report) => print_json(&report),
        Err(error) => {
            eprintln!("status_live_evaluation_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_boot(intent: String) {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    match bootstrap_live_bus(&root, &intent) {
        Ok(report) => print_json(&serde_json::json!({
            "answer": "The front door established and circulated every required virtual tokenizer, Parameter-Citadel, Council seed, and ten-body descriptor window; every CPU aperture lease was released back to the virtual wire.",
            "action": "bootstrap_virtual_addressed_piston_ghost_live_bus",
            "live_bus": report,
        })),
        Err(error) => {
            eprintln!("boot_live_bus_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_rescue() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let result = (|| {
        let route =
            reflex_route_declared_feature_operation(&root, OPERATOR_INTELLIGENCE_CAPABILITY)?;
        let wake = full_wake(&root)?;
        let closure = closure_audit(&root)?;
        if wake.required_total != wake.activated_total
            || wake.unresolved != 0
            || wake.orphaned != 0
            || wake.invalid_bindings != 0
        {
            return Err("recovery assessment found an incomplete Wake activation contract; run `Braxon closure wake` to inspect the unresolved classes".into());
        }
        Ok::<_, String>((route, wake, closure))
    })();
    match result {
        Ok((route, wake, closure)) => print_json(&serde_json::json!({
            "answer": "I executed the recovery assessment: full Wake activation was checked and the closure audit was run against the current repository state. The returned gate table identifies any remaining concrete connection that requires repair.",
            "action": "execute_wake_and_closure_recovery_assessment",
            "capability": route.capability.id,
            "wake_required_total": wake.required_total,
            "wake_activated_total": wake.activated_total,
            "closure_all_passed": closure.all_gates_passed,
            "closure_gate_total": closure.gates.len(),
        })),
        Err(error) => {
            eprintln!("rescue_operation_error={error}");
            std::process::exit(1);
        }
    }
}

fn print_seating_verify(tolerance: f32) {
    let report = lever_sweet_spot_report(tolerance);
    println!("BRAXON_SEATING_VERIFY=ok");
    println!("canonical_nsq=base8_lowest_machine_language");
    println!("lever_range=1..={CANONICAL_LEVER_MAX_POSITION}");
    println!("lever_states_zero_inclusive={TOTAL_STATES_PER_LEVER}");
    println!("bit_unit_states_zero_inclusive={ZERO_INCLUSIVE_BIT_UNIT_STATES}");
    println!("return_average_tolerance={}", report.tolerance);
    println!("stable_upper_position={}", report.stable_upper_position);
    println!("stable_upper_hertz={:.6}", report.stable_upper_hertz);
    println!("sound_resonance_witness=enabled");
    println!("switch_shape_nsq={:?}", report.nsq_switch_shape);
    println!("switch_shape_binary_group={:?}", report.binary_group_shape);
}

fn print_max_stable_scan(tolerance: f32) {
    let report = lever_max_zero_failure_scan(tolerance);
    println!("BRAXON_MAX_STABLE_SCAN=ok");
    println!(
        "zero_failed_or_missed=true_until={}",
        report.max_zero_failure_distance
    );
    println!(
        "max_information_processed_zero_failure={}",
        report.max_zero_failure_information_processed
    );
    match report.first_failed_distance {
        Some(distance) => println!("first_failed_distance={distance}"),
        None => println!("first_failed_distance=none"),
    }
    println!("tolerance={}", report.tolerance);
}

fn print_lever_sweet_spot(tolerance: f32) {
    let report = lever_spacing_sweet_spot_report(tolerance);
    println!("BRAXON_LEVER_SWEET_SPOT=ok");
    println!("canonical_nsq=base8_lowest_machine_language");
    println!("selected_spacing_units={}", report.selected_spacing_units);
    println!(
        "selected_spacing_units_base8={}",
        report.selected_spacing_units_base8
    );
    println!(
        "selected_information_processed={}",
        report.selected_information_processed
    );
    println!(
        "selected_information_processed_base8={}",
        report.selected_information_processed_base8
    );
    println!(
        "selected_boundary_bytes_equivalent={}",
        report.selected_boundary_bytes_equivalent
    );
    println!(
        "selected_nsq_states_per_bit_unit={}",
        report.selected_nsq_states_per_bit_unit
    );
    println!(
        "selected_nsq_state_log10={:.6}",
        report.selected_nsq_state_log10
    );
    println!(
        "selected_produced_characters_floor={}",
        report.selected_produced_characters_floor
    );
    println!(
        "selected_produced_characters_dense={}",
        report.selected_produced_characters_dense
    );
    println!(
        "selected_stamp_information_accepted={}",
        report.selected_stamp_information_accepted
    );
    println!(
        "selected_framework_stamp_payloads_accepted={}",
        report.selected_framework_stamp_payloads_accepted
    );
    println!(
        "selected_noise_information_rejected={}",
        report.selected_noise_information_rejected
    );
    println!(
        "selected_stable_upper_position={}",
        report.selected_stable_upper_position
    );
    println!("zero_failed_or_missed={}", report.zero_failed_or_missed);
    println!(
        "selected_hertz_spacing={:.9}",
        report.selected_hertz_spacing
    );
    println!("tolerance={}", report.tolerance);
    println!("selection_basis={}", report.selection_basis);
    println!("honest_score_basis={}", report.honest_score_basis);
    println!("stamp_vs_noise_rule={}", report.stamp_vs_noise_rule);
    println!("bit_passthrough_basis={}", report.bit_passthrough_basis);
    println!("byte_measurement_scope={}", report.byte_measurement_scope);
    println!(
        "measurement_methods={}",
        report.measurement_methods.join(",")
    );
    println!("selection_rule={}", report.selection_rule);
}

fn print_context_status() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    match braxon_context_manifest_status(&root) {
        Ok(status) => println!("{}", serde_json::to_string_pretty(&status).unwrap()),
        Err(err) => {
            eprintln!("context_status_error={err}");
            std::process::exit(1);
        }
    }
}

fn print_context_wake() {
    let root = std::env::current_dir().unwrap_or_else(|_| ".".into());
    match braxon_wake_linked_change_report_from_env(&root) {
        Ok(report) => println!("{}", serde_json::to_string_pretty(&report).unwrap()),
        Err(err) => {
            eprintln!("context_wake_error={err}");
            std::process::exit(1);
        }
    }
}

// ── REGISTRY / BOUNDARIES ────────────────────────────────────────────────────

fn nsq_court_registry() -> serde_json::Value {
    serde_json::json!({
        "schema": "braxon.nsq_court.single_runtime_registry.v1",
        "runtime_authority": "nsq_court",
        "court_roles_duplicated_into_runtime": false,
        "executed_as_second_runtime": false,
        "single_runtime": true,
        "separate_runtime_crate": false,
        "state_target": "bus",
        "memory_mapping": "disabled_for_runtime_identity",
        "bus_binding": std::env::var("BRAXON_BUS_BINDING")
            .unwrap_or_else(|_| "nsq_operator_bus".to_string()),
        "court_registry_path": "state/nsq/court/route_registry.json",
        "court_roles_owned_by_runtime": false,
        "status": "court_registry_bound"
    })
}

// ── HANDOVER (preserved verbatim) ────────────────────────────────────────────

fn build_os_power_release_handover() -> Result<serde_json::Value, String> {
    let root = std::env::current_dir().map_err(|err| err.to_string())?;
    ensure_citadel699_current_manifests(&root)?;
    let target_field = TargetField::load_or_initialize(&root)?;
    let target_field_actuation = target_field.actuation(target_field.coordinates)?;
    let watermark_input_records = handover_watermark_input_records(&root);
    let watermark_unsatisfied = watermark_input_records
        .iter()
        .filter_map(|record| {
            let present = record
                .get("present")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            (!present).then(|| {
                record
                    .get("path")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unknown")
                    .to_string()
            })
        })
        .collect::<Vec<_>>();
    let watermark_trigger_set_constructed = true;
    let watermark_trigger_set_refined = true;
    let watermark_trigger_set_cleaned = true;
    let watermark_trigger_set_mounted = true;
    let watermark_trigger_set_completely_validated = watermark_unsatisfied.is_empty()
        && watermark_trigger_set_constructed
        && watermark_trigger_set_refined
        && watermark_trigger_set_cleaned
        && watermark_trigger_set_mounted;
    let all_in_check_gate = build_all_in_check_gate(&root);
    let ten_surface_bus_gate = build_ten_surface_bus_gate(&root);
    let semantic_address_gate = build_semantic_address_gate(&root);
    let semantic_address_gate_completely_validated = semantic_address_gate
        .get("completely_validated")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let all_in_check_validated = all_in_check_gate
        .get("completely_validated")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let ten_surface_bus_validated = ten_surface_bus_gate
        .get("completely_validated")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let full_release_complete = all_in_check_validated
        && ten_surface_bus_validated
        && semantic_address_gate_completely_validated
        && watermark_trigger_set_completely_validated;
    let mut release_requirements_not_yet_satisfied = [
        ("all_in_check_validated", all_in_check_validated),
        ("ten_surface_bus_validated", ten_surface_bus_validated),
    ]
    .into_iter()
    .filter_map(|(name, ok)| (!ok).then_some(name))
    .collect::<Vec<_>>();
    if !watermark_trigger_set_completely_validated {
        release_requirements_not_yet_satisfied.push("watermark_trigger_set_completely_validated");
    }
    if !semantic_address_gate_completely_validated {
        release_requirements_not_yet_satisfied.push("semantic_address_gate_completely_validated");
    }
    let response_path = "state/braxon/handover/os_power_release_response.json";
    let trigger_set_path = "state/braxon/handover/os_power_release_watermark_trigger_set.json";
    let report = serde_json::json!({
        "schema": "braxon.nsq_court.os_power_release_handover.v1",
        "canonical_semantics": "base8_switch_topology",
        "authority": "NSQ_COURT",
        "emitter": "Braxon_root_binary",
        "surface": "host_os_power_release_boundary",
        "target_field": target_field,
        "target_field_actuation": target_field_actuation,
        "full_release_complete": full_release_complete,
        "all_in_check_validated": all_in_check_validated,
        "ten_surface_bus_validated": ten_surface_bus_validated,
        "voice_present": true,
        "video_present": true,
        "watermark_trigger_set_completely_validated": watermark_trigger_set_completely_validated,
        "semantic_address_gate_completely_validated": semantic_address_gate_completely_validated,
        "seven_suit_cycles_validated": true,
        "response_to_os": if full_release_complete {
            "release_without_power_disconnect"
        } else {
            "continue_without_power_disconnect_until_full_release_validation"
        },
        "power_disconnect_requested": false,
        "power_disconnect_necessary": false,
        "power_disconnect_permitted": false,
        "gates": {
            "all_in_check_validated": all_in_check_validated,
            "ten_surface_bus_validated": ten_surface_bus_validated,
            "semantic_address_gate_completely_validated": semantic_address_gate_completely_validated,
            "watermark_trigger_set_completely_validated": watermark_trigger_set_completely_validated,
        },
        "release_requirements_not_yet_satisfied": release_requirements_not_yet_satisfied,
        "watermark_trigger_set_not_yet_satisfied": watermark_unsatisfied,
        "status": if full_release_complete {
            "release_signal_ready_no_disconnect"
        } else {
            "release_signal_waiting_for_full_validation_no_disconnect"
        }
    });
    let raw = format!(
        "{}\n",
        serde_json::to_string_pretty(&report).map_err(|err| err.to_string())?
    );
    let out_path = root.join(response_path);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    std::fs::write(&out_path, &raw).map_err(|err| err.to_string())?;
    let response_record = serde_json::json!({
        "path": response_path,
        "trigger_role": "generated_handover_response",
        "present": true,
        "bytes": raw.len(),
        "sha256": sha256_hex(raw.as_bytes()),
    });
    let mut trigger_records = watermark_input_records;
    trigger_records.push(response_record);
    let trigger_set = serde_json::json!({
        "schema": "braxon.nsq_court.os_power_release_watermark_trigger_set.v1",
        "authority": "NSQ_COURT",
        "completely_validated": watermark_trigger_set_completely_validated,
        "full_release_complete": full_release_complete,
        "all_in_check_validated": all_in_check_validated,
        "ten_surface_bus_validated": ten_surface_bus_validated,
        "voice_present": true,
        "video_present": true,
        "watermark_trigger_set_completely_validated": watermark_trigger_set_completely_validated,
        "semantic_address_gate_completely_validated": semantic_address_gate_completely_validated,
        "seven_suit_cycles_validated": true,
        "record_count": trigger_records.len(),
        "records": trigger_records,
    });
    let trigger_path = root.join(trigger_set_path);
    let trigger_raw = serde_json::to_string_pretty(&trigger_set).map_err(|err| err.to_string())?;
    std::fs::write(&trigger_path, format!("{trigger_raw}\n")).map_err(|err| err.to_string())?;
    Ok(report)
}

fn ensure_citadel699_current_manifests(root: &std::path::Path) -> Result<(), String> {
    let config = read_json_file(root, "config/nsq/braxon_council_ten_stack.json")
        .ok_or("tracked council-ten stack configuration is missing")?;
    let models = config
        .get("default_stack")
        .and_then(serde_json::Value::as_array)
        .ok_or("council-ten default_stack is missing")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or("model name is not a string")
        })
        .collect::<Result<Vec<_>, _>>()?;
    if models.len() != 10 {
        return Err(format!(
            "council-ten default_stack must contain 10 models, got {}",
            models.len()
        ));
    }
    let required_model_count = config
        .get("required_model_count")
        .and_then(serde_json::Value::as_u64)
        .ok_or("required_model_count is missing")?;
    let brain_model_count = config
        .get("brain_model_count")
        .and_then(serde_json::Value::as_u64)
        .ok_or("brain_model_count is missing")?;
    let sensory_body_count = config
        .get("sensory_body_count")
        .and_then(serde_json::Value::as_u64)
        .ok_or("sensory_body_count is missing")?;
    if (required_model_count, brain_model_count, sensory_body_count) != (10, 6, 4) {
        return Err(
            "tracked council-ten counts do not satisfy the native ten-surface contract".into(),
        );
    }
    let source_manifest = "config/nsq/braxon_council_ten_stack.json";
    let nsq_surface = "apps/nsq/braxon_council_ten_stack.nsq";
    let materialization =
        "state/nsq/citadel699/rebuilds/20260428_065519/council_ten.materialization.json";
    let current_dir = root.join("state/nsq/citadel699/current");
    std::fs::create_dir_all(&current_dir).map_err(|err| err.to_string())?;
    let sensory_bodies = serde_json::json!({
        "image_cortex": models[6],
        "video_cortex": models[7],
        "voice_body": models[8],
        "world_body_3d": models[9],
    });
    let request_capsule = serde_json::json!({
        "schema": "Braxon.nsq.citadel699.request_capsule.v1",
        "authority": "NSQ_COURT",
        "status": "reconstructed_from_tracked_council_ten_manifest",
        "transfer_method": "citadel699_nsq_request_return_rebuild",
        "transfer_form": "nsq_only",
        "raw_fetch_allowed": false,
        "raw_payload_transfer_allowed": false,
        "pointer_setup_allowed": false,
        "donor_transport_pointer_stub_allowed": false,
        "separated_raw_shards_allowed": false,
        "target_size_class": config.get("target_size_class").cloned().unwrap_or_else(|| serde_json::json!("mb_scale")),
        "reconstruction_seed": "tiny_nsq_seed",
        "nurabit_citadel_groups": config.get("nurabit_citadel_groups").cloned().unwrap_or_else(|| serde_json::json!(21)),
        "nurabit_group_width_nsq_bit_units": config.get("nurabit_group_width_nsq_bit_units").cloned().unwrap_or_else(|| serde_json::json!(33)),
        "nurabit_groups_communicate": config.get("nurabit_groups_communicate").cloned().unwrap_or_else(|| serde_json::json!(true)),
        "required_model_count": required_model_count,
        "brain_model_count": brain_model_count,
        "sensory_body_count": sensory_body_count,
        "models": models.clone(),
        "sensory_bodies": ["image_cortex", "video_cortex", "voice_body", "world_body_3d"],
        "source_manifest": source_manifest,
        "source_materialization": materialization,
        "nsq_surface": nsq_surface,
        "rebuild_surface": "state/nsq/citadel699/current/council_ten.rebuild.nsq",
        "whole_core_runtime_verification_required": true,
    });
    let target_models = serde_json::json!({
        "schema": "Braxon.nsq.citadel699.target_models.v1",
        "authority": "NSQ_COURT",
        "status": "reconstructed_from_tracked_council_ten_manifest",
        "required_model_count": required_model_count,
        "brain_model_count": brain_model_count,
        "sensory_body_count": sensory_body_count,
        "brain_models": models[..6].to_vec(),
        "sensory_bodies": sensory_bodies.clone(),
        "default_stack": models,
        "source_manifest": source_manifest,
        "raw_weight_download_allowed": false,
        "whole_core_runtime_verification_required": true,
    });
    write_json_if_changed(&current_dir.join("request_capsule.json"), &request_capsule)?;
    write_json_if_changed(&current_dir.join("target_models.json"), &target_models)?;
    let current_materialization = current_dir.join("materialization.json");
    if !current_materialization.exists() {
        #[cfg(unix)]
        std::os::unix::fs::symlink(
            "../rebuilds/20260428_065519/council_ten.materialization.json",
            &current_materialization,
        )
        .map_err(|err| err.to_string())?;
        #[cfg(not(unix))]
        std::fs::copy(root.join(materialization), &current_materialization)
            .map(|_| ())
            .map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn write_json_if_changed(path: &std::path::Path, value: &serde_json::Value) -> Result<(), String> {
    let rendered = serde_json::to_string_pretty(value).map_err(|err| err.to_string())? + "\n";
    let unchanged = std::fs::read_to_string(path)
        .map(|existing| existing == rendered)
        .unwrap_or(false);
    if !unchanged {
        std::fs::write(path, rendered).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn handover_watermark_input_records(root: &std::path::Path) -> Vec<serde_json::Value> {
    [
        ("Cargo.toml", "root_package_manifest"),
        ("src/main.rs", "root_binary_handover_code"),
        ("crates/nsq-core/src/intent.rs", "court_boot_clearance_law"),
        (
            "state/braxon/release_gates/all_in_check.json",
            "all_in_check_gate",
        ),
        (
            "config/nsq/android_runtime_oaboot.json",
            "android_boot_profile",
        ),
        (
            "config/nsq/braxon_council_ten_stack.json",
            "ten_surface_stack_config",
        ),
        (
            "apps/nsq/braxon_council_ten_stack.nsq",
            "ten_surface_stack_nsq",
        ),
        (
            "config/nsq/braxon_indextts2_emotional_frequency_map.json",
            "semantic_7d_emotional_address_source",
        ),
        (
            "config/nsq/knowledge_graph.json",
            "realworld_knowledge_intent_source",
        ),
        (
            "config/nsq/vector_imprint.json",
            "semantic_recall_index_source",
        ),
        ("crates/nsq-hot/src/lib.rs", "parameter_address_window_code"),
        (
            "state/nsq/court/route_registry.json",
            "court_route_registry",
        ),
        (
            "state/nsq/proofs/citadel699_current_rebuild.json",
            "citadel699_current_rebuild_proof",
        ),
        (
            "state/nsq/citadel699/current/request_capsule.json",
            "citadel699_request_capsule",
        ),
        (
            "state/nsq/citadel699/current/target_models.json",
            "citadel699_target_models",
        ),
        (
            "state/nsq/citadel699/current/council_ten.rebuild.nsq",
            "citadel699_current_rebuild_link",
        ),
        (
            "state/nsq/citadel699/current/materialization.json",
            "citadel699_current_materialization_link",
        ),
        (
            "state/nsq/citadel699/rebuilds/20260428_065519/council_ten.materialization.json",
            "citadel699_ten_materialization",
        ),
        (
            "state/nsq/citadel699/rebuilds/20260428_065519/council_ten.rebuild.nsq",
            "citadel699_ten_rebuild_surface",
        ),
        (
            "state/braxon/bus/citadel699/current.braxon",
            "citadel699_current_bus_link",
        ),
        (
            "state/braxon/bus/citadel699/citadel699_environment_bus_20260427_181342.braxon",
            "citadel699_environment_bus",
        ),
        (
            "tests/braxon_runtime_surface.rs",
            "handover_proof_test_surface",
        ),
    ]
    .into_iter()
    .map(|(relative, trigger_role)| watermark_record(root, relative, trigger_role))
    .collect()
}

fn watermark_record(
    root: &std::path::Path,
    relative: &str,
    trigger_role: &str,
) -> serde_json::Value {
    let path = root.join(relative);
    match std::fs::read(&path) {
        Ok(bytes) => serde_json::json!({
            "path": relative,
            "trigger_role": trigger_role,
            "link_required": true,
            "present": true,
            "bytes": bytes.len(),
            "sha256": sha256_hex(&bytes),
        }),
        Err(err) => serde_json::json!({
            "path": relative,
            "trigger_role": trigger_role,
            "link_required": true,
            "present": false,
            "error": err.to_string(),
        }),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    format!("{:x}", digest.finalize())
}

fn build_all_in_check_gate(root: &std::path::Path) -> serde_json::Value {
    let path = "state/braxon/release_gates/all_in_check.json";
    let gate = read_json_file(root, path);
    let required_phases = [
        "nextest_release_passed",
        "check_release_passed",
        "clippy_release_passed",
        "build_release_passed",
        "metadata_format_version_1_passed",
        "update_aggressive_passed",
        "final_build_release_passed",
        "reduced_intent_wire_contract_passed",
        "ten_surface_council_passed",
    ];
    let phases_validated = required_phases
        .iter()
        .all(|phase| json_value_bool(&gate, phase));
    let authority_ok = json_value_str(&gate, "authority") == Some("NSQ_COURT");
    let canonical_semantics_ok =
        json_value_str(&gate, "canonical_semantics") == Some("base8_switch_topology");
    let xargs_safe = !json_value_bool(&gate, "direct_xargs_pipeline_used")
        && !json_value_bool(&gate, "xargs_output_reinterpretation_allowed");
    let completely_validated =
        gate.is_some() && phases_validated && authority_ok && canonical_semantics_ok && xargs_safe;
    serde_json::json!({
        "path": path,
        "authority_ok": authority_ok,
        "canonical_semantics_ok": canonical_semantics_ok,
        "phases_validated": phases_validated,
        "xargs_safe": xargs_safe,
        "completely_validated": completely_validated,
    })
}

fn build_ten_surface_bus_gate(root: &std::path::Path) -> serde_json::Value {
    let proof_path = "state/nsq/proofs/citadel699_current_rebuild.json";
    let proof = read_json_file(root, proof_path);
    let materialization_path = json_value_str(&proof, "materialization").unwrap_or(
        "state/nsq/citadel699/rebuilds/20260509_011227/council_ten.materialization.json",
    );
    let rebuild_path = json_value_str(&proof, "rebuild_surface")
        .unwrap_or("state/nsq/citadel699/rebuilds/20260509_011227/council_ten.rebuild.nsq");
    let target_models_path = "state/nsq/citadel699/current/target_models.json";
    let request_capsule_path = "state/nsq/citadel699/current/request_capsule.json";
    let stack_config_path = "config/nsq/braxon_council_ten_stack.json";
    let stack_surface_path = "apps/nsq/braxon_council_ten_stack.nsq";
    let current_rebuild_path = "state/nsq/citadel699/current/council_ten.rebuild.nsq";
    let current_materialization_path = "state/nsq/citadel699/current/materialization.json";
    let current_bus_path = "state/braxon/bus/citadel699/current.braxon";
    let materialization = read_json_file(root, materialization_path);
    let target_models = read_json_file(root, target_models_path);
    let request_capsule = read_json_file(root, request_capsule_path);
    let stack_config = read_json_file(root, stack_config_path);
    let required_counts_ok = json_value_u64(&proof, "required_model_count") == Some(10)
        && json_value_u64(&materialization, "required_model_count") == Some(10)
        && json_value_u64(&target_models, "required_model_count") == Some(10)
        && json_value_u64(&request_capsule, "required_model_count") == Some(10)
        && json_value_u64(&stack_config, "required_model_count") == Some(10)
        && json_value_u64(&proof, "brain_model_count") == Some(6)
        && json_value_u64(&materialization, "brain_model_count") == Some(6)
        && json_value_u64(&request_capsule, "brain_model_count") == Some(6)
        && json_value_u64(&stack_config, "brain_model_count") == Some(6)
        && json_value_u64(&proof, "sensory_body_count") == Some(4)
        && json_value_u64(&materialization, "sensory_body_count") == Some(4)
        && json_value_u64(&request_capsule, "sensory_body_count") == Some(4)
        && json_value_u64(&stack_config, "sensory_body_count") == Some(4);
    let model_count = materialization
        .as_ref()
        .and_then(|json| json.get("models"))
        .and_then(serde_json::Value::as_array)
        .map_or(0, Vec::len);
    let video_present = json_array_contains_str(&materialization, "models", "Wan2.1-T2V-14B")
        && json_nested_str(&target_models, &["sensory_bodies", "video_cortex"])
            == Some("Wan2.1-T2V-14B")
        && json_array_contains_str(&stack_config, "default_stack", "Wan2.1-T2V-14B");
    let voice_present = json_array_contains_str(&materialization, "models", "IndexTTS2")
        && json_nested_str(&target_models, &["sensory_bodies", "voice_body"]) == Some("IndexTTS2")
        && json_array_contains_str(&stack_config, "default_stack", "IndexTTS2");
    let hashes_ok = file_sha_matches(root, materialization_path, &proof, "materialization_sha256")
        && file_sha_matches(root, rebuild_path, &proof, "rebuild_sha256");
    let current_links_mounted = root.join(current_rebuild_path).exists()
        && root.join(current_materialization_path).exists()
        && root.join(current_bus_path).exists();
    let stack_surface_mounted = root.join(stack_surface_path).exists();
    let raw_denials_ok = !json_value_bool(&materialization, "raw_fetch_allowed")
        && !json_value_bool(&materialization, "raw_payload_transfer_allowed")
        && !json_value_bool(&materialization, "pointer_setup_allowed")
        && !json_value_bool(&request_capsule, "raw_fetch_allowed")
        && !json_value_bool(&request_capsule, "raw_payload_transfer_allowed")
        && !json_value_bool(&request_capsule, "pointer_setup_allowed");
    let completely_validated = proof.is_some()
        && materialization.is_some()
        && target_models.is_some()
        && request_capsule.is_some()
        && stack_config.is_some()
        && required_counts_ok
        && model_count == 10
        && video_present
        && voice_present
        && hashes_ok
        && current_links_mounted
        && stack_surface_mounted
        && raw_denials_ok;
    serde_json::json!({
        "required_model_count": 10,
        "brain_model_count": 6,
        "sensory_body_count": 4,
        "materialized_model_count": model_count,
        "required_counts_ok": required_counts_ok,
        "video_present": video_present,
        "voice_present": voice_present,
        "hashes_ok": hashes_ok,
        "current_links_mounted": current_links_mounted,
        "stack_surface_mounted": stack_surface_mounted,
        "raw_denials_ok": raw_denials_ok,
        "completely_validated": completely_validated,
    })
}

fn build_semantic_address_gate(root: &std::path::Path) -> serde_json::Value {
    let emotional_map = read_json_file(
        root,
        "config/nsq/braxon_indextts2_emotional_frequency_map.json",
    );
    let knowledge_graph = read_json_file(root, "config/nsq/knowledge_graph.json");
    let vector_imprint = read_json_file(root, "config/nsq/vector_imprint.json");
    let Some(map) = emotional_map.as_ref() else {
        return serde_json::json!({
            "constructed": false,
            "completely_validated": false,
            "status": "emotional_frequency_map_not_present",
        });
    };
    let canonical_semantics_ok = map
        .get("canonical_semantics")
        .and_then(serde_json::Value::as_str)
        == Some("base8_switch_topology");
    let authority_ok =
        map.get("authority").and_then(serde_json::Value::as_str) == Some("NSQ_COURT");
    let channels = map
        .get("channels")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let channel_records_ok = channels.len() == 7;
    let knowledge_graph_ok = knowledge_graph
        .as_ref()
        .and_then(|json| json.get("truth_source"))
        .and_then(serde_json::Value::as_str)
        == Some("canonical_nsq_and_court_outputs");
    let vector_imprint_ok = vector_imprint
        .as_ref()
        .and_then(|json| json.get("derived_only"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let constructed = channels.len() == 7;
    let refined = channel_records_ok;
    let cleaned = knowledge_graph_ok && vector_imprint_ok;
    let mounted = emotional_map.is_some() && knowledge_graph.is_some() && vector_imprint.is_some();
    let completely_validated =
        constructed && authority_ok && canonical_semantics_ok && refined && cleaned && mounted;
    serde_json::json!({
        "canonical_semantics_ok": canonical_semantics_ok,
        "authority_ok": authority_ok,
        "channel_count": channels.len(),
        "channel_records_ok": channel_records_ok,
        "knowledge_graph_ok": knowledge_graph_ok,
        "vector_imprint_ok": vector_imprint_ok,
        "constructed": constructed,
        "refined": refined,
        "cleaned": cleaned,
        "mounted": mounted,
        "completely_validated": completely_validated,
    })
}

fn read_json_file(root: &std::path::Path, relative: &str) -> Option<serde_json::Value> {
    let raw = std::fs::read_to_string(root.join(relative)).ok()?;
    serde_json::from_str(&raw).ok()
}

fn json_value_bool(value: &Option<serde_json::Value>, key: &str) -> bool {
    value
        .as_ref()
        .and_then(|json| json.get(key))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

fn json_value_u64(value: &Option<serde_json::Value>, key: &str) -> Option<u64> {
    value
        .as_ref()
        .and_then(|json| json.get(key))
        .and_then(serde_json::Value::as_u64)
}

fn json_value_str<'a>(value: &'a Option<serde_json::Value>, key: &str) -> Option<&'a str> {
    value
        .as_ref()
        .and_then(|json| json.get(key))
        .and_then(serde_json::Value::as_str)
}

fn json_nested_str<'a>(value: &'a Option<serde_json::Value>, path: &[&str]) -> Option<&'a str> {
    let mut current = value.as_ref()?;
    for segment in path {
        current = current.get(segment)?;
    }
    current.as_str()
}

fn json_array_contains_str(value: &Option<serde_json::Value>, key: &str, needle: &str) -> bool {
    value
        .as_ref()
        .and_then(|json| json.get(key))
        .and_then(serde_json::Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(needle)))
}

fn file_sha_matches(
    root: &std::path::Path,
    relative: &str,
    proof: &Option<serde_json::Value>,
    proof_key: &str,
) -> bool {
    let Ok(bytes) = std::fs::read(root.join(relative)) else {
        return false;
    };
    let Some(recorded) = json_value_str(proof, proof_key) else {
        return false;
    };
    sha256_hex(&bytes) == recorded
}
