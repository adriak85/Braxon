use clap::{Parser, Subcommand};
use nsq_core::{
    lever_max_zero_failure_scan, lever_spacing_sweet_spot_report, lever_sweet_spot_report,
    CANONICAL_LEVER_MAX_POSITION, TOTAL_STATES_PER_LEVER, ZERO_INCLUSIVE_BIT_UNIT_STATES,
};
use nsq_runtime::{native_runtime_registry, OfflineModelLane, Python3RuntimeLane};
use BRAXON_core::{braxon_context_manifest_status, braxon_wake_linked_change_report_from_env};

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
    Rescue,
    Status,
    ContextStatus,
    ContextWake,
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
    Python3 { call: String },
    Infer { model: String, prompt: String },
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
        app: "nsq-runtime",
        package: "nsq-runtime",
        bin_name: "Braxon runtime",
        root_launchable: true,
        surface: "native_runtime_lane",
    },
];

fn main() {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Command::Status) {
        Command::Apps { command } => print_apps(command),
        Command::Console { seated_mode } => {
            if seated_mode {
                println!("[System] Seated Mode Active.");
            }
            print_status();
        }
        Command::SeatingVerify { tolerance } => print_seating_verify(tolerance),
        Command::MaxStableScan { tolerance } => print_max_stable_scan(tolerance),
        Command::LeverSweetSpot { tolerance } => print_lever_sweet_spot(tolerance),
        Command::Runtime { command } => print_runtime(command),
        Command::Rescue => println!(
            "[WoWaS] Rescue lane reserved; read canon/config directly from oldest to newest before applying."
        ),
        Command::Status => print_status(),
        Command::ContextStatus => print_context_status(),
        Command::ContextWake => print_context_wake(),
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
                "root_launch_coverage_complete={}",
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

fn print_runtime(command: RuntimeCommand) {
    match command {
        RuntimeCommand::Registry => print_json(&native_runtime_registry()),
        RuntimeCommand::Python3 { call } => {
            match Python3RuntimeLane::default().execute_slice(&call) {
                Ok(report) => print_json(&report),
                Err(err) => {
                    eprintln!("python3_runtime_error={err}");
                    std::process::exit(1);
                }
            }
        }
        RuntimeCommand::Infer { model, prompt } => {
            match OfflineModelLane::default().execute_request(&model, &prompt) {
                Ok(report) => print_json(&report),
                Err(err) => {
                    eprintln!("runtime_infer_error={err}");
                    std::process::exit(1);
                }
            }
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

fn print_status() {
    println!("--- BRAXON/NSQ SOVEREIGN FRONT ENTRANCE ---");
    println!("nsq_identity=lowest_base_language");
    println!("lever_states_zero_inclusive={TOTAL_STATES_PER_LEVER}");
    println!("bit_unit_states_zero_inclusive={ZERO_INCLUSIVE_BIT_UNIT_STATES}");
    println!();
    println!("I am here. The void is listening. What shall we build together?");
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
