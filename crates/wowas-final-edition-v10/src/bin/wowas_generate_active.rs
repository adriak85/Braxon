#[path = "../namegen.rs"]
mod namegen;

use namegen::{best_reviewed_name, NameGenRequest};
use std::collections::BTreeSet;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const CHARACTER_TARGET: usize = 5000;
const CREATURE_TARGET: usize = 5000;

const ROLES: &[&str] = &[
    "field healer",
    "map seller",
    "tunnel witness",
    "corridor child",
    "ripper survivor",
    "sonomancy listener",
    "glyph apprentice",
    "glass forge worker",
    "orchard guard",
    "wolf-road scout",
    "market singer",
    "stone archivist",
    "faction runner",
    "bridge keeper",
    "garden watcher",
    "triage helper",
    "bookbinder",
    "cavern farmer",
    "cellar cook",
    "threshold mason",
];

const REGIONS: &[&str] = &[
    "willow-stone county",
    "glass orchard",
    "stone fen",
    "blue-light road",
    "ripper tunnel",
    "morrow market",
    "black-heart ridge",
    "last open gate",
    "neith edge",
    "pocket sanctuary",
    "xeth glyph yard",
    "cello field",
    "root vale",
    "ash river",
    "diamond breakland",
];

const HOUSE_PRESSURES: &[&str] = &[
    "Vayne",
    "Null-Heart",
    "Iron-Song",
    "Wren-Ash",
    "Thorn-Vayne",
    "Marso-len",
    "Rolzen-line",
    "Kyreal-pack",
    "Xethrolund-glyph",
    "Pip-no-path",
    "Mack-ground",
    "Dervish-glass",
];

const SOURCES: &[&str] = &[
    "Pip source orbit",
    "Rolzen source orbit",
    "Xethrolund source orbit",
    "Rylos source orbit",
    "Kyreal source orbit",
    "Thessa source orbit",
    "Kael source orbit",
    "Pael source orbit",
    "Soth source orbit",
    "Vellin source orbit",
    "Corrath source orbit",
    "Dervish source orbit",
];

const TIERS: &[&str] = &[
    "story",
    "support",
    "recurring-background",
    "background",
    "deep-background",
];

const CREATURE_FORMS: &[&str] = &[
    "moth-deer",
    "root hound",
    "glass heron",
    "basalt hare",
    "orchard eel",
    "lantern fox",
    "cinder elk",
    "moss bear",
    "willow crab",
    "stonefin bird",
    "thorn whale",
    "ash otter",
    "ribbon serpent",
    "mirror lynx",
    "fern mantis",
    "marrow crane",
    "quartz ram",
    "song beetle",
];

const TRAITS: &[&str] = &[
    "weird",
    "gross",
    "beautiful",
    "horrific",
    "elegant",
    "chaotic",
    "sanitized",
    "verdant",
    "unibreccianated",
    "unbrictionable",
    "resonant",
    "glyph-touched",
    "diamond-scarred",
    "cello-woken",
];

const ECOLOGY: &[&str] = &[
    "pollinator",
    "carrion recycler",
    "warning species",
    "healing symbiote",
    "apex grazer",
    "burrow architect",
    "memory mimic",
    "root spreader",
    "glass cleaner",
    "river filter",
    "gate sentinel",
    "song amplifier",
    "soil restorer",
    "weather listener",
    "pack companion",
];

fn item<'a>(items: &'a [&str], idx: usize, salt: usize) -> &'a str {
    items[(idx * 37 + salt * 11) % items.len()]
}

fn reviewed_name(
    seed: &str,
    pressure: &str,
    source: &str,
    role: &str,
    region: &str,
) -> (String, u16) {
    let req = NameGenRequest {
        seed,
        house_pressure: pressure,
        source_anchor: source,
        story_role: role,
        region,
        minimum_candidates: 90,
    };
    match best_reviewed_name(&req) {
        Some(candidate) => (candidate.name, candidate.score),
        None => (format!("Fallback{}Name", seed.replace('-', "")), 0),
    }
}

fn main() -> std::io::Result<()> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let active = root.join("canon/active");
    let generated = active.join("generated");
    create_dir_all(&generated)?;

    let char_active_path = active.join("character_timeline_lattice_v14_33.tsv");
    let char_generated_path = generated.join("wowas_generated_characters_5000.tsv");
    let creature_active_path = active.join("creature_registry_target_5000.tsv");
    let creature_generated_path = generated.join("wowas_generated_creatures_5000.tsv");
    let summary_path = generated.join("generation_summary.tsv");

    let mut char_lines: Vec<String> = Vec::with_capacity(CHARACTER_TARGET + 1);
    char_lines.push("character_id\tname\ttier\tvolume\tbook_anchor\trole\thouse_pressure\tregion\tsource_anchor\tcandidate_draws\tselection_score\tstory_background_law\tvisual_status".to_string());
    let mut used_names = BTreeSet::new();
    for i in 1..=CHARACTER_TARGET {
        let role = item(ROLES, i, 1);
        let region = item(REGIONS, i, 2);
        let pressure = item(HOUSE_PRESSURES, i, 3);
        let source = item(SOURCES, i, 4);
        let tier = item(TIERS, i, 5);
        let volume = 1 + ((i - 1) / 1700).min(2);
        let book = 1 + ((i - 1) % 33);
        let mut seed_count = 0;
        let (mut name, mut score) = loop {
            let seed = format!("wowas-character-{i:05}-{seed_count}-{role}-{region}");
            let candidate = reviewed_name(&seed, pressure, source, role, region);
            if used_names.insert(candidate.0.clone()) || seed_count > 20 {
                break candidate;
            }
            seed_count += 1;
        };
        if !used_names.insert(name.clone()) {
            name = format!("{}{}", name, i);
            score = score.saturating_sub(1);
        }
        char_lines.push(format!(
            "WC{i:05}\t{name}\t{tier}\tV{volume}\tB{book:02}\t{role}\t{pressure}\t{region}\t{source}\t90\t{score}\taveraged_background_story_orbit_aesthetic_origin_distance\tpending_art_reference"
        ));
    }

    let mut creature_lines: Vec<String> = Vec::with_capacity(CREATURE_TARGET + 1);
    creature_lines.push("creature_id\tspecies_name\tbase_form\tprimary_trait\tsecondary_trait\tbiome\tdanger_band\tecology_role\tcandidate_draws\tselection_score\tgeneration_law\tvisual_status".to_string());
    let mut used_creatures = BTreeSet::new();
    for i in 1..=CREATURE_TARGET {
        let form = item(CREATURE_FORMS, i, 1);
        let primary = item(TRAITS, i, 2);
        let secondary = item(TRAITS, i, 7);
        let biome = item(REGIONS, i, 4);
        let ecology = item(ECOLOGY, i, 5);
        let danger = match i % 5 {
            0 => "gentle",
            1 => "watchful",
            2 => "hazardous",
            3 => "predatory",
            _ => "sacred-dangerous",
        };
        let pressure = format!("{primary}-{form}");
        let source = format!("{secondary}-{ecology}");
        let role = format!("{ecology} creature species");
        let mut seed_count = 0;
        let (mut name, mut score) = loop {
            let seed = format!("wowas-creature-{i:05}-{seed_count}-{form}-{biome}");
            let candidate = reviewed_name(&seed, &pressure, &source, &role, biome);
            if used_creatures.insert(candidate.0.clone()) || seed_count > 20 {
                break candidate;
            }
            seed_count += 1;
        };
        if !used_creatures.insert(name.clone()) {
            name = format!("{}{}", name, i);
            score = score.saturating_sub(1);
        }
        creature_lines.push(format!(
            "WR{i:05}\t{name}\t{form}\t{primary}\t{secondary}\t{biome}\t{danger}\t{ecology}\t90\t{score}\tmutative_post_diamond_5000_creature_registry\tpending_art_reference"
        ));
    }

    write_lines(&char_active_path, &char_lines)?;
    write_lines(&char_generated_path, &char_lines)?;
    write_lines(&creature_active_path, &creature_lines)?;
    write_lines(&creature_generated_path, &creature_lines)?;
    write_lines(
        &summary_path,
        &[
            "key\tvalue".to_string(),
            format!("characters\t{CHARACTER_TARGET}"),
            format!("creatures\t{CREATURE_TARGET}"),
            "candidate_draws_per_entry\t90".to_string(),
            "selection_law\taveraged_background_story_orbit_aesthetic_origin_distance".to_string(),
            "picture_status\tpending_art_reference".to_string(),
        ],
    )?;

    println!("wrote {}", char_active_path.display());
    println!("wrote {}", creature_active_path.display());
    Ok(())
}

fn write_lines(path: &PathBuf, lines: &[String]) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(path)?);
    for line in lines {
        writeln!(out, "{line}")?;
    }
    Ok(())
}
