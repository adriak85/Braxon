use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: nsq-generate <scale_count> <density> <out_dir>");
        exit(2);
    }

    let scale: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("invalid scale_count");
        exit(2);
    });
    let density: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("invalid density");
        exit(2);
    });
    let out_dir = &args[3];

    if scale == 0 || density == 0 {
        eprintln!("scale_count and density must be > 0");
        exit(2);
    }

    fs::create_dir_all(out_dir).unwrap_or_else(|e| {
        eprintln!("mkdir error: {e}");
        exit(2);
    });

    let noise_path = Path::new(out_dir).join("noise_large.nsq");
    let structured_path = Path::new(out_dir).join("structured_large.nsq");
    let membrane_path = Path::new(out_dir).join("membrane_large.nsq");
    let calibration_path = Path::new(out_dir).join("calibration_large.nsq");

    let symbols = [
        "wake",
        "delta",
        "signal",
        "graph",
        "membrane",
        "state",
        "matrix",
        "cell",
        "sovereign",
        "address",
        "route",
        "field",
        "lattice",
        "anchor",
        "vector",
        "phase",
        "corridor",
        "seal",
        "bridge",
        "spine",
        "memory",
        "current",
        "vessel",
        "gate",
    ];

    let relations = [
        "semantic:address",
        "delta:route:align",
        "cell:membrane:matrix",
        "signal:field:link",
        "graph:state:route",
        "vector:lattice:bind",
        "anchor:phase:lock",
        "sovereign:address:seal",
        "memory:bridge:current",
        "gate:vector:spine",
    ];

    let macro_names = [
        "wake:self",
        "delta:route:align",
        "noise:signal:field",
        "graph:state:route",
        "vector:lattice:bind",
        "anchor:phase:lock",
        "membrane:cell:shift",
        "sovereign:seal:hold",
        "memory:bridge:current",
        "gate:vector:spine",
    ];

    let mut noise = String::from("# generated large noise surface\n");
    let mut structured = String::from("# generated large structured surface\n");
    let mut membrane = String::from("# generated large membrane surface\n");
    let mut calibration = String::from("# generated large calibration surface\n");

    for i in 0..scale {
        let cluster = i % density;
        let s1 = symbols[(i + cluster) % symbols.len()];
        let s2 = symbols[(i + cluster * 2 + 3) % symbols.len()];
        let s3 = symbols[(i + cluster * 3 + 5) % symbols.len()];
        let rel = relations[(i / density + cluster) % relations.len()];
        let mac = macro_names[(i / density + cluster) % macro_names.len()];

        let a = ((i * 3 + cluster * 5) % 64) as u8;
        let b = ((i * 5 + cluster * 7 + 11) % 64) as u8;
        let pos = 100 + (cluster as u32 * 64) + (i as u32 * 9);
        let amp = 20 + ((i + cluster * 13) as u16 % 220);

        noise.push_str(&format!(
            "noise {s1} :macro {mac} :a {a} :b {b} :pos {pos} :amp {amp}\n"
        ));

        let layer = ((i + cluster) % 32) as u8;
        let plane = ((i * 7 + cluster * 3) % 32) as u8;
        let anchor = 80 + (cluster as u32 * 40) + (i as u32 * 13);
        let weight = 8 + ((i * 3 + cluster * 17) as u16 % 700);
        let flags = ((i + cluster) % 8) as u8;

        structured.push_str(&format!(
            "triple {s1} -> {rel} -> {s2} :layer {layer} :plane {plane} :anchor {anchor} :weight {weight} :flags {flags}\n"
        ));

        let flux = 24 + ((i * 5 + cluster * 19) as u16 % 500);
        let gate = ((i * 11 + cluster * 2) % 32) as u8;
        let phase = ((i * 13 + cluster * 5) % 32) as u8;

        membrane.push_str(&format!(
            "membrane {s2} :state {s3} :flux {flux} :gate {gate} :phase {phase}\n"
        ));

        let gain = 4 + ((i + cluster * 3) as u16 % 96);
        let window = 12 + ((i * 2 + cluster * 11) as u16 % 192);
        let cphase = ((i * 3 + cluster) % 16) as u8;

        calibration.push_str(&format!(
            "calibrate {rel} :basis {s1} :gain {gain} :window {window} :phase {cphase}\n"
        ));

        if cluster.is_multiple_of(2) {
            structured.push_str(&format!(
                "triple {s2} -> {rel} -> {s3} :layer {} :plane {} :anchor {} :weight {} :flags {}\n",
                (layer + 1) % 32,
                (plane + 2) % 32,
                anchor + 5,
                weight + 1,
                (flags + 1) % 8
            ));
        }

        if cluster.is_multiple_of(3) {
            membrane.push_str(&format!(
                "membrane {s1} :state {s2} :flux {} :gate {} :phase {}\n",
                flux + 3,
                (gate + 1) % 32,
                (phase + 1) % 32
            ));
        }

        if cluster.is_multiple_of(4) {
            noise.push_str(&format!(
                "noise {s3} :macro {mac} :a {} :b {} :pos {} :amp {}\n",
                (a + 1) % 64,
                (b + 2) % 64,
                pos + 3,
                amp + 2
            ));
        }
    }

    fs::write(&noise_path, noise).unwrap();
    fs::write(&structured_path, structured).unwrap();
    fs::write(&membrane_path, membrane).unwrap();
    fs::write(&calibration_path, calibration).unwrap();

    println!("generated_dir={out_dir}");
    println!("scale={scale}");
    println!("density={density}");
    println!("noise_file={}", noise_path.display());
    println!("structured_file={}", structured_path.display());
    println!("membrane_file={}", membrane_path.display());
    println!("calibration_file={}", calibration_path.display());
}
