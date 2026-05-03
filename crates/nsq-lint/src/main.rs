//! BASE / CANONICAL SURFACE
//! Remove width-class truth from this surface.
//! Any host-carrier widths that remain are bugs or temporary boundary leaks.

use serde::Serialize;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Serialize)]
struct Finding {
    line: usize,
    level: String,
    message: String,
}

#[derive(Serialize)]
struct Report {
    file: String,
    ok: bool,
    findings: Vec<Finding>,
    counts: Counts,
}

#[derive(Default, Serialize)]
struct Counts {
    noise: usize,
    triple: usize,
    membrane: usize,
    calibrate: usize,
}

fn parse_u8(s: &str) -> Option<u8> {
    s.parse::<u8>().ok()
}
fn parse_u16(s: &str) -> Option<u16> {
    s.parse::<u16>().ok()
}
fn parse_u32(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: nsq-lint <input.nsq>");
        exit(2);
    }

    let path = &args[1];
    let text = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("read error: {e}");
        exit(2);
    });

    let mut findings = Vec::<Finding>::new();
    let mut counts = Counts::default();

    for (idx, raw) in text.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }

        match toks[0] {
            "noise" => {
                counts.noise += 1;
                let mut saw_macro = false;
                let mut saw_a = false;
                let mut saw_b = false;
                let mut saw_pos = false;
                let mut saw_amp = false;

                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":macro" => saw_macro = true,
                        ":a" => {
                            saw_a = true;
                            if parse_u8(toks[i + 1]).filter(|v| *v <= 63).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "noise :a out of range".into(),
                                });
                            }
                        }
                        ":b" => {
                            saw_b = true;
                            if parse_u8(toks[i + 1]).filter(|v| *v <= 63).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "noise :b out of range".into(),
                                });
                            }
                        }
                        ":pos" => {
                            saw_pos = true;
                            if parse_u32(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "noise :pos invalid".into(),
                                });
                            }
                        }
                        ":amp" => {
                            saw_amp = true;
                            if parse_u16(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "noise :amp invalid".into(),
                                });
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }

                for (ok, msg) in [
                    (saw_macro, "noise missing :macro"),
                    (saw_a, "noise missing :a"),
                    (saw_b, "noise missing :b"),
                    (saw_pos, "noise missing :pos"),
                    (saw_amp, "noise missing :amp"),
                ] {
                    if !ok {
                        findings.push(Finding {
                            line: line_no,
                            level: "error".into(),
                            message: msg.into(),
                        });
                    }
                }
            }
            "triple" => {
                counts.triple += 1;
                if toks.len() < 6 || toks.get(2) != Some(&"->") || toks.get(4) != Some(&"->") {
                    findings.push(Finding {
                        line: line_no,
                        level: "error".into(),
                        message: "triple malformed".into(),
                    });
                    continue;
                }

                let mut saw_layer = false;
                let mut saw_plane = false;
                let mut saw_anchor = false;
                let mut saw_weight = false;
                let mut saw_flags = false;

                let mut i = 6usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":layer" => {
                            saw_layer = true;
                            if parse_u8(toks[i + 1]).filter(|v| *v <= 31).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "triple :layer out of range".into(),
                                });
                            }
                        }
                        ":plane" => {
                            saw_plane = true;
                            if parse_u8(toks[i + 1]).filter(|v| *v <= 31).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "triple :plane out of range".into(),
                                });
                            }
                        }
                        ":anchor" => {
                            saw_anchor = true;
                            if parse_u32(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "triple :anchor invalid".into(),
                                });
                            }
                        }
                        ":weight" => {
                            saw_weight = true;
                            if parse_u16(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "triple :weight invalid".into(),
                                });
                            }
                        }
                        ":flags" => {
                            saw_flags = true;
                            if parse_u8(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "triple :flags invalid".into(),
                                });
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }

                for (ok, msg) in [
                    (saw_layer, "triple missing :layer"),
                    (saw_plane, "triple missing :plane"),
                    (saw_anchor, "triple missing :anchor"),
                    (saw_weight, "triple missing :weight"),
                    (saw_flags, "triple missing :flags"),
                ] {
                    if !ok {
                        findings.push(Finding {
                            line: line_no,
                            level: "error".into(),
                            message: msg.into(),
                        });
                    }
                }
            }
            "membrane" => {
                counts.membrane += 1;
                let mut saw_state = false;
                let mut saw_flux = false;
                let mut saw_gate = false;
                let mut saw_phase = false;

                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":state" => saw_state = true,
                        ":flux" => {
                            saw_flux = true;
                            if parse_u16(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "membrane :flux invalid".into(),
                                });
                            }
                        }
                        ":gate" => {
                            saw_gate = true;
                            if parse_u8(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "membrane :gate invalid".into(),
                                });
                            }
                        }
                        ":phase" => {
                            saw_phase = true;
                            if parse_u8(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "membrane :phase invalid".into(),
                                });
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }

                for (ok, msg) in [
                    (saw_state, "membrane missing :state"),
                    (saw_flux, "membrane missing :flux"),
                    (saw_gate, "membrane missing :gate"),
                    (saw_phase, "membrane missing :phase"),
                ] {
                    if !ok {
                        findings.push(Finding {
                            line: line_no,
                            level: "error".into(),
                            message: msg.into(),
                        });
                    }
                }
            }
            "calibrate" => {
                counts.calibrate += 1;
                let mut saw_basis = false;
                let mut saw_gain = false;
                let mut saw_window = false;
                let mut saw_phase = false;

                let mut i = 2usize;
                while i + 1 < toks.len() {
                    match toks[i] {
                        ":basis" => saw_basis = true,
                        ":gain" => {
                            saw_gain = true;
                            if parse_u16(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "calibrate :gain invalid".into(),
                                });
                            }
                        }
                        ":window" => {
                            saw_window = true;
                            if parse_u16(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "calibrate :window invalid".into(),
                                });
                            }
                        }
                        ":phase" => {
                            saw_phase = true;
                            if parse_u8(toks[i + 1]).is_none() {
                                findings.push(Finding {
                                    line: line_no,
                                    level: "error".into(),
                                    message: "calibrate :phase invalid".into(),
                                });
                            }
                        }
                        _ => {}
                    }
                    i += 2;
                }

                for (ok, msg) in [
                    (saw_basis, "calibrate missing :basis"),
                    (saw_gain, "calibrate missing :gain"),
                    (saw_window, "calibrate missing :window"),
                    (saw_phase, "calibrate missing :phase"),
                ] {
                    if !ok {
                        findings.push(Finding {
                            line: line_no,
                            level: "error".into(),
                            message: msg.into(),
                        });
                    }
                }
            }
            other => {
                findings.push(Finding {
                    line: line_no,
                    level: "error".into(),
                    message: format!("unknown record kind: {other}"),
                });
            }
        }
    }

    let ok = !findings.iter().any(|f| f.level == "error");
    let report = Report {
        file: path.clone(),
        ok,
        findings,
        counts,
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
    if !report.ok {
        exit(1);
    }
}
