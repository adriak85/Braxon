use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{create_dir_all, read_dir, read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const CORE_NAMES: &[&str] = &[
    "Pip", "Indalwin", "Mack", "Xethrolund", "Rylos", "Boojay", "Rolzen",
    "Daisy May", "Majiskii", "Dervish", "Ursula", "Neith",
];

const CHARACTER_EVENTS: &[&str] = &[
    "argument that turns into protection",
    "small ridiculous task that reveals serious loyalty",
    "rescue attempt that goes socially wrong before it goes right",
    "market pressure scene with a hidden consequence",
    "quiet care scene interrupted by external danger",
    "comic misunderstanding that exposes a true wound",
    "training failure that becomes a better method",
    "shared meal that turns into a strategic decision",
    "route negotiation where the weak detail saves everyone",
    "unexpected kindness that creates a later obligation",
    "public embarrassment that becomes witness",
    "dangerous question asked at the wrong time but for the right reason",
];

const WILDLIFE_EVENTS: &[&str] = &[
    "animal blocks the obvious path and saves the party from a worse route",
    "creature comedy turns into ecological warning",
    "small species reacts to magic before people understand the threat",
    "pack behavior mirrors the cast relationship under pressure",
    "predator encounter reveals a hidden wounded character",
    "symbiotic creature creates a temporary shelter or bridge",
    "migration pattern exposes political or magical disruption",
    "wildlife panic signals a concealed ripper or Drawn pressure",
];

const DESERT_PRESSURES: &[&str] = &[
    "heat shimmer memory distortion",
    "salt road caravan etiquette",
    "buried glass river crossing",
    "predator shadow with no visible body",
    "desert market debt ritual",
    "dry well that still sings underground",
    "ashstorm shelter negotiation",
    "night-cold campfire truth scene",
    "oasis mirage that is partly real",
    "bone orchard boundary custom",
];

#[derive(Debug, Clone, Default)]
struct SourceProfile {
    id: String,
    name: String,
    origin: String,
    function: String,
    light: String,
    shadow: String,
    relationship: String,
    power: String,
    reimagine: String,
    system: String,
    anchor: String,
    dark_rule: String,
    obligation: String,
}

#[derive(Debug, Clone, Default)]
struct CharacterCandidate {
    name: String,
    book: usize,
    score: i32,
    tier: String,
    role: String,
    source_anchor: String,
    reason: String,
    is_core: bool,
    is_dark_transient: bool,
}

#[derive(Debug, Clone, Default)]
struct BookPlan {
    num: usize,
    title: String,
    active_cast: Vec<String>,
    introduced: Vec<String>,
    key_text: String,
}

#[derive(Debug, Clone, Default)]
struct Creature {
    id: String,
    name: String,
    form: String,
    primary: String,
    secondary: String,
    biome: String,
    danger: String,
    ecology: String,
    score: i32,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let want_all = args.is_empty() || args.iter().any(|a| a == "--all");
    let want_characters = want_all || args.iter().any(|a| a == "--characters");
    let want_wildlife = want_all || args.iter().any(|a| a == "--wildlife");
    let want_desert = want_all || args.iter().any(|a| a == "--desert");
    let only_book = parse_book_filter(&args);

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let canon = root.join("canon");
    let out_dir = canon.join("active/generated");
    create_dir_all(&out_dir)?;

    let source_profiles = read_source_profiles(&root.join(".non_wowas_source_inspiration_lattice.file"));
    let books = read_book_plans(&canon.join("canonical_story_tree/characters/02_CHARACTER_PLACEMENT_BY_BOOK.md"));
    let candidates = review_all_characters(&canon, &books, &source_profiles);
    let creatures = read_creatures(&canon.join("active/creature_registry_target_5000.tsv"));

    if want_characters {
        write_character_encounters(&out_dir, &books, &candidates, &source_profiles, only_book)?;
    }
    if want_wildlife {
        write_wildlife_encounters(&out_dir, &books, &candidates, &creatures, only_book)?;
    }
    if want_desert {
        write_desert_population(&out_dir, &books, &candidates, &creatures, &source_profiles, only_book)?;
    }
    write_review_summary(&out_dir, &source_profiles, &candidates, &creatures)?;

    println!("wrote encounter system outputs under {}", out_dir.display());
    Ok(())
}

fn parse_book_filter(args: &[String]) -> Option<usize> {
    for i in 0..args.len() {
        if args[i] == "--book" {
            return args.get(i + 1).and_then(|s| s.trim_start_matches('B').parse::<usize>().ok());
        }
        if let Some(rest) = args[i].strip_prefix("--book=") {
            return rest.trim_start_matches('B').parse::<usize>().ok();
        }
    }
    None
}

fn read_source_profiles(path: &Path) -> Vec<SourceProfile> {
    let Ok(text) = read_to_string(path) else { return Vec::new(); };
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    let idx = index_map(header);
    let mut out = Vec::new();
    for line in lines {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 8 { continue; }
        let profile = SourceProfile {
            id: col(&cols, &idx, "source_id"),
            name: col(&cols, &idx, "source_name"),
            origin: col(&cols, &idx, "franchise_or_origin"),
            function: col(&cols, &idx, "function_harvested"),
            light: col(&cols, &idx, "light_side"),
            shadow: col(&cols, &idx, "shadow_side"),
            relationship: col(&cols, &idx, "relationship_style"),
            power: col(&cols, &idx, "power_style"),
            reimagine: col(&cols, &idx, "what_must_be_reimagined"),
            system: col(&cols, &idx, "reimagination_system"),
            anchor: col(&cols, &idx, "primary_wowas_anchor"),
            dark_rule: col(&cols, &idx, "transient_dark_plot_rule"),
            obligation: col(&cols, &idx, "plot_thread_obligation"),
        };
        if !profile.name.is_empty() { out.push(profile); }
    }
    out
}

fn read_book_plans(path: &Path) -> Vec<BookPlan> {
    let Ok(text) = read_to_string(path) else { return Vec::new(); };
    let mut books = Vec::new();
    let mut current: Option<BookPlan> = None;
    let mut in_key = false;
    for raw in text.lines() {
        let line = raw.trim();
        if let Some(rest) = line.strip_prefix("## Book ") {
            if let Some(b) = current.take() { books.push(b); }
            let num = rest.get(0..2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            let title = rest.split('—').nth(1).unwrap_or(rest).trim().to_string();
            current = Some(BookPlan { num, title, ..Default::default() });
            in_key = false;
            continue;
        }
        let Some(book) = current.as_mut() else { continue; };
        if let Some(rest) = line.strip_prefix("**Active cast**:") {
            book.active_cast = split_names(rest);
            in_key = false;
        } else if let Some(rest) = line.strip_prefix("**Introduced this book**:") {
            book.introduced = split_names(rest);
            in_key = false;
        } else if line.starts_with("**Key character beats**") {
            in_key = true;
        } else if line.starts_with("## ") {
            in_key = false;
        } else if in_key {
            book.key_text.push_str(line);
            book.key_text.push('\n');
        }
    }
    if let Some(b) = current.take() { books.push(b); }
    books.sort_by_key(|b| b.num);
    books
}

fn review_all_characters(canon: &Path, books: &[BookPlan], sources: &[SourceProfile]) -> Vec<CharacterCandidate> {
    let mut by_key: BTreeMap<(usize, String), CharacterCandidate> = BTreeMap::new();

    for book in books {
        for name in &book.active_cast {
            add_candidate(&mut by_key, book.num, name, 140, "active_cast", "placement_by_book", sources, &book.key_text);
        }
        for name in &book.introduced {
            add_candidate(&mut by_key, book.num, name, 105, "introduced", "placement_by_book", sources, &book.key_text);
        }
        for name in extract_known_names(&book.key_text) {
            add_candidate(&mut by_key, book.num, &name, 70, "key_beat_mention", "placement_by_book", sources, &book.key_text);
        }
    }

    absorb_named_cast(canon, &mut by_key, sources);
    absorb_generated_lattice(canon, &mut by_key, sources);
    absorb_character_review(canon, &mut by_key, sources);
    absorb_registry_names(canon, &mut by_key, sources);

    let mut out: Vec<CharacterCandidate> = by_key.into_values().collect();
    for c in &mut out {
        if c.is_core { c.score += 45; }
        if !c.source_anchor.is_empty() { c.score += 15; }
        if c.role.contains("story") || c.role.contains("support") { c.score += 10; }
        if c.name.starts_with("WC") || c.name.starts_with("auto::") { c.score -= 20; }
        if c.name.contains("Placeholder") || c.name.contains("TBD") { c.score -= 100; }
        c.is_dark_transient = is_dark_transient(&c.name, &c.role, &c.reason, &c.source_anchor);
    }
    out.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
    out
}

fn add_candidate(
    map: &mut BTreeMap<(usize, String), CharacterCandidate>,
    book: usize,
    name: &str,
    score: i32,
    tier: &str,
    reason: &str,
    sources: &[SourceProfile],
    context: &str,
) {
    let clean = clean_name(name);
    if clean.len() < 2 { return; }
    let key = (book, clean.to_lowercase());
    let is_core = is_core_name(&clean);
    let source_anchor = best_source_anchor(&clean, context, sources);
    let entry = map.entry(key).or_insert_with(|| CharacterCandidate {
        name: clean.clone(), book, tier: tier.to_string(), role: String::new(), source_anchor, reason: reason.to_string(), is_core, ..Default::default()
    });
    entry.score += score;
    if entry.source_anchor.is_empty() { entry.source_anchor = best_source_anchor(&entry.name, context, sources); }
    if !entry.reason.contains(reason) {
        entry.reason.push('|');
        entry.reason.push_str(reason);
    }
    entry.is_core |= is_core;
}

fn absorb_named_cast(canon: &Path, map: &mut BTreeMap<(usize, String), CharacterCandidate>, sources: &[SourceProfile]) {
    let path = canon.join("canonical_story_tree/characters/01_NAMED_CAST_TOP300.md");
    let Ok(text) = read_to_string(path) else { return; };
    let names = extract_markdown_names(&text);
    for book in 1..=33 {
        for name in names.iter().take(120) {
            add_candidate(map, book, name, 12, "named_cast", "named_cast_top300", sources, &text);
        }
    }
}

fn absorb_registry_names(canon: &Path, map: &mut BTreeMap<(usize, String), CharacterCandidate>, sources: &[SourceProfile]) {
    let path = canon.join("canonical_story_tree/characters/06_CHARACTER_REGISTRY.json");
    let Ok(text) = read_to_string(path) else { return; };
    for name in extract_jsonish_names(&text).into_iter().take(300) {
        for book in 1..=33 {
            add_candidate(map, book, &name, 8, "registry", "character_registry", sources, &text);
        }
    }
}

fn absorb_character_review(canon: &Path, map: &mut BTreeMap<(usize, String), CharacterCandidate>, sources: &[SourceProfile]) {
    let path = canon.join("control/character_generation_review_v14.tsv");
    let Ok(text) = read_to_string(path) else { return; };
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split('\t').collect();
        let joined = cols.join(" ");
        for name in extract_known_names(&joined) {
            for book in 1..=33 {
                add_candidate(map, book, &name, 18, "reviewed", "character_generation_review", sources, &joined);
            }
        }
    }
}

fn absorb_generated_lattice(canon: &Path, map: &mut BTreeMap<(usize, String), CharacterCandidate>, sources: &[SourceProfile]) {
    let path = canon.join("active/character_timeline_lattice_v14_33.tsv");
    let Ok(text) = read_to_string(path) else { return; };
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    let idx = index_map(header);
    for line in lines {
        let cols: Vec<&str> = line.split('\t').collect();
        let name = col(&cols, &idx, "name");
        if name.is_empty() { continue; }
        let book_text = col(&cols, &idx, "book_anchor");
        let book = book_text.trim_start_matches('B').parse::<usize>().unwrap_or(0);
        if book == 0 { continue; }
        let tier = col(&cols, &idx, "tier");
        let role = col(&cols, &idx, "role");
        let score = col(&cols, &idx, "selection_score").parse::<i32>().unwrap_or(0);
        add_candidate(map, book, &name, 25 + score / 10, &tier, "generated_lattice_reviewed", sources, &format!("{role} {tier}"));
        if let Some(c) = map.get_mut(&(book, clean_name(&name).to_lowercase())) {
            c.role = role;
        }
    }
}

fn read_creatures(path: &Path) -> Vec<Creature> {
    let Ok(text) = read_to_string(path) else { return Vec::new(); };
    let mut lines = text.lines();
    let idx = index_map(lines.next().unwrap_or(""));
    let mut out = Vec::new();
    for line in lines {
        let cols: Vec<&str> = line.split('\t').collect();
        let name = col(&cols, &idx, "species_name");
        if name.is_empty() { continue; }
        out.push(Creature {
            id: col(&cols, &idx, "creature_id"),
            name,
            form: col(&cols, &idx, "base_form"),
            primary: col(&cols, &idx, "primary_trait"),
            secondary: col(&cols, &idx, "secondary_trait"),
            biome: col(&cols, &idx, "biome"),
            danger: col(&cols, &idx, "danger_band"),
            ecology: col(&cols, &idx, "ecology_role"),
            score: col(&cols, &idx, "selection_score").parse::<i32>().unwrap_or(0),
        });
    }
    out.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
    out
}

fn write_character_encounters(out_dir: &Path, books: &[BookPlan], candidates: &[CharacterCandidate], sources: &[SourceProfile], only_book: Option<usize>) -> std::io::Result<()> {
    let path = out_dir.join("wowas_character_encounters.tsv");
    let mut out = BufWriter::new(File::create(path)?);
    writeln!(out, "book_num\tbook_title\tencounter_id\tcore_character\tsatellite_character\tsatellite_score\tsource_profiles\tsource_traits\tevent_shape\tstakes\tmemorable_hook\tplot_incorporation\tcoverage_rule")?;

    let mut coverage: BTreeMap<(usize, String), usize> = BTreeMap::new();
    let mut dark_rows: Vec<String> = Vec::new();
    let mut n = 0usize;
    for book in books.iter().filter(|b| only_book.map(|x| x == b.num).unwrap_or(true)) {
        let selected = select_best_for_book(book.num, candidates, 18);
        let mut core: Vec<CharacterCandidate> = selected.iter().filter(|c| c.is_core).cloned().collect();
        if core.is_empty() {
            core = selected.iter().take(3).cloned().collect();
        }
        let satellites: Vec<CharacterCandidate> = selected.iter().filter(|c| !c.is_core).cloned().collect();
        let fallback_satellites = if satellites.is_empty() { selected.clone() } else { satellites.clone() };

        for core_char in &core {
            for i in 0..3 {
                let sat = pick(&fallback_satellites, n + i + book.num);
                n += 1;
                write_character_row(&mut out, book, n, core_char, sat, sources, &mut coverage, &mut dark_rows, "core_minimum_three")?;
            }
        }
        for sat in &satellites {
            if coverage.get(&(book.num, sat.name.clone())).copied().unwrap_or(0) == 0 {
                let core_char = pick(&core, n + book.num);
                n += 1;
                write_character_row(&mut out, book, n, core_char, sat, sources, &mut coverage, &mut dark_rows, "satellite_minimum_one")?;
            }
        }
    }

    let mut cov = BufWriter::new(File::create(out_dir.join("wowas_encounter_coverage.tsv"))?);
    writeln!(cov, "book_num\tcharacter\tencounter_count")?;
    for ((book, name), count) in coverage {
        writeln!(cov, "{}\t{}\t{}", book, tsv(&name), count)?;
    }
    let mut dark = BufWriter::new(File::create(out_dir.join("wowas_dark_transient_plot_threads.tsv"))?);
    writeln!(dark, "book_num\tencounter_id\tcharacter\tplot_thread_obligation")?;
    for row in dark_rows { writeln!(dark, "{row}")?; }
    Ok(())
}

fn write_character_row<W: Write>(out: &mut W, book: &BookPlan, id_num: usize, core: &CharacterCandidate, sat: &CharacterCandidate, sources: &[SourceProfile], coverage: &mut BTreeMap<(usize, String), usize>, dark_rows: &mut Vec<String>, rule: &str) -> std::io::Result<()> {
    let profiles = profiles_for(&format!("{} {} {} {}", core.name, sat.name, core.source_anchor, sat.source_anchor), sources);
    let profile_ids = profiles.iter().map(|p| p.id.as_str()).collect::<Vec<_>>().join("+");
    let traits = profiles.iter().take(3).map(|p| format!("{} / {} / {}", p.function, p.relationship, p.power)).collect::<Vec<_>>().join(" || ");
    let event = CHARACTER_EVENTS[id_num % CHARACTER_EVENTS.len()];
    let stakes = if sat.is_dark_transient { "dark transient match must enter plot continuity" } else { "relationship, pressure, humor, and future obligation" };
    let memorable = if sat.score > 150 { "high-score satellite gets recurring callback seed" } else { "small event must leave a visible mark" };
    let incorporation = if sat.is_dark_transient { plot_obligation(&profiles, sat) } else { "encounter may remain local unless later scene pressure calls it back".to_string() };
    let eid = format!("WE{:02}_{:05}", book.num, id_num);
    writeln!(out, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        book.num, tsv(&book.title), eid, tsv(&core.name), tsv(&sat.name), sat.score, tsv(&profile_ids), tsv(&traits), tsv(event), tsv(stakes), tsv(memorable), tsv(&incorporation), rule)?;
    *coverage.entry((book.num, core.name.clone())).or_insert(0) += 1;
    *coverage.entry((book.num, sat.name.clone())).or_insert(0) += 1;
    if sat.is_dark_transient {
        dark_rows.push(format!("{}\t{}\t{}\t{}", book.num, eid, tsv(&sat.name), tsv(&incorporation)));
    }
    Ok(())
}

fn write_wildlife_encounters(out_dir: &Path, books: &[BookPlan], candidates: &[CharacterCandidate], creatures: &[Creature], only_book: Option<usize>) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(out_dir.join("wowas_wildlife_encounters.tsv"))?);
    writeln!(out, "book_num\tbook_title\tencounter_id\tcharacter_anchor\tcreature_id\tspecies_name\tbiome\tdanger\tecology_role\tevent_shape\tplot_use")?;
    let mut n = 0usize;
    for book in books.iter().filter(|b| only_book.map(|x| x == b.num).unwrap_or(true)) {
        let selected = select_best_for_book(book.num, candidates, 8);
        for i in 0..4 {
            if creatures.is_empty() || selected.is_empty() { continue; }
            let c = pick(creatures, book.num * 17 + i * 13);
            let ch = pick(&selected, book.num + i);
            n += 1;
            let event = WILDLIFE_EVENTS[(book.num + i) % WILDLIFE_EVENTS.len()];
            let plot = if c.danger.contains("predatory") || c.danger.contains("sacred") {
                "wildlife pressure should expose hidden route, social cost, or magical imbalance"
            } else {
                "wildlife encounter may carry humor, ecology, or local culture"
            };
            writeln!(out, "{}\t{}\tWW{:02}_{:05}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                book.num, tsv(&book.title), book.num, n, tsv(&ch.name), tsv(&c.id), tsv(&c.name), tsv(&c.biome), tsv(&c.danger), tsv(&c.ecology), tsv(event), tsv(plot))?;
        }
    }
    Ok(())
}

fn write_desert_population(out_dir: &Path, books: &[BookPlan], candidates: &[CharacterCandidate], creatures: &[Creature], sources: &[SourceProfile], only_book: Option<usize>) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(out_dir.join("wowas_desert_population.tsv"))?);
    writeln!(out, "book_num\tbook_title\tdesert_cell_id\tpopulation_function\tcharacter_anchor\twildlife_anchor\tsource_inspiration\tpressure\troute_or_place\tmemorable_event\tuse_individually")?;
    let routes = ["salt-glass road", "bone orchard edge", "dry river underpass", "ash dune market", "heat-hum shrine", "night well camp", "red sand switchback", "buried gate trail"];
    let functions = ["trader", "guide", "water witness", "wildlife handler", "desert healer", "lost-child finder", "storm listener", "route judge", "camp cook", "border singer"];
    let mut n = 0usize;
    for book in books.iter().filter(|b| only_book.map(|x| x == b.num).unwrap_or(true)) {
        let selected = select_best_for_book(book.num, candidates, 10);
        for i in 0..6 {
            n += 1;
            let ch = selected.get((book.num + i) % selected.len().max(1));
            let cr = creatures.get((book.num * 29 + i) % creatures.len().max(1));
            let sp = sources.get((book.num * 7 + i) % sources.len().max(1));
            let pressure = DESERT_PRESSURES[(book.num + i) % DESERT_PRESSURES.len()];
            let route = routes[(book.num * 3 + i) % routes.len()];
            let function = functions[(book.num + i * 2) % functions.len()];
            let memorable = format!("{} at {} forces {} to matter", pressure, route, function);
            writeln!(out, "{}\t{}\tWD{:02}_{:05}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                book.num, tsv(&book.title), book.num, n, tsv(function), tsv(ch.map(|x| x.name.as_str()).unwrap_or("local desert satellite")), tsv(cr.map(|x| x.name.as_str()).unwrap_or("desert wildlife pending")), tsv(sp.map(|x| x.name.as_str()).unwrap_or("source pending")), tsv(pressure), tsv(route), tsv(&memorable), "yes")?;
        }
    }
    Ok(())
}

fn write_review_summary(out_dir: &Path, sources: &[SourceProfile], candidates: &[CharacterCandidate], creatures: &[Creature]) -> std::io::Result<()> {
    let mut out = BufWriter::new(File::create(out_dir.join("wowas_encounter_source_review.tsv"))?);
    writeln!(out, "kind\tcount\tnote")?;
    writeln!(out, "source_inspiration_profiles\t{}\tnon-WoWaS hidden source lattice profiles loaded", sources.len())?;
    writeln!(out, "character_candidates_reviewed\t{}\tplacement, named cast, registry, generated lattice, and review inputs scored", candidates.len())?;
    writeln!(out, "creature_candidates_reviewed\t{}\tactive creature registry reviewed for wildlife/desert use", creatures.len())?;
    let mut top = BufWriter::new(File::create(out_dir.join("wowas_best_character_picks.tsv"))?);
    writeln!(top, "book_num\trank\tname\tscore\ttier\treason\tsource_anchor\tis_core\tis_dark_transient")?;
    let mut by_book: BTreeMap<usize, Vec<&CharacterCandidate>> = BTreeMap::new();
    for c in candidates { by_book.entry(c.book).or_default().push(c); }
    for (book, mut chars) in by_book {
        chars.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
        for (rank, c) in chars.into_iter().take(24).enumerate() {
            writeln!(top, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", book, rank + 1, tsv(&c.name), c.score, tsv(&c.tier), tsv(&c.reason), tsv(&c.source_anchor), c.is_core, c.is_dark_transient)?;
        }
    }
    Ok(())
}

fn select_best_for_book(book: usize, candidates: &[CharacterCandidate], limit: usize) -> Vec<CharacterCandidate> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for c in candidates.iter().filter(|c| c.book == book) {
        if seen.insert(c.name.to_lowercase()) {
            out.push(c.clone());
        }
        if out.len() >= limit { break; }
    }
    if out.is_empty() {
        for c in candidates.iter().take(limit) { out.push(c.clone()); }
    }
    out
}

fn profiles_for<'a>(text: &str, profiles: &'a [SourceProfile]) -> Vec<&'a SourceProfile> {
    let lower = text.to_lowercase();
    let mut out = Vec::new();
    for p in profiles {
        let anchor = p.anchor.to_lowercase();
        if lower.contains(&p.name.to_lowercase()) || (!anchor.is_empty() && lower.contains(&anchor)) {
            out.push(p);
        }
    }
    if out.is_empty() {
        out.extend(profiles.iter().take(3));
    }
    out
}

fn plot_obligation(profiles: &[&SourceProfile], sat: &CharacterCandidate) -> String {
    let mut bits = Vec::new();
    bits.push("plot incorporation required".to_string());
    bits.push(format!("{} cannot remain disposable", sat.name));
    for p in profiles.iter().take(2) {
        if !p.dark_rule.is_empty() { bits.push(p.dark_rule.clone()); }
        if !p.obligation.is_empty() { bits.push(p.obligation.clone()); }
    }
    bits.join("; ")
}

fn best_source_anchor(name: &str, context: &str, profiles: &[SourceProfile]) -> String {
    let hay = format!("{} {}", name, context).to_lowercase();
    let mut matches = Vec::new();
    for p in profiles {
        let anchor = p.anchor.to_lowercase();
        if hay.contains(&p.name.to_lowercase()) || (!anchor.is_empty() && hay.contains(&anchor)) {
            matches.push(p.id.clone());
        }
    }
    matches.join("+")
}

fn is_core_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    CORE_NAMES.iter().any(|c| lower.contains(&c.to_lowercase()))
}

fn is_dark_transient(name: &str, role: &str, reason: &str, anchor: &str) -> bool {
    let s = format!("{} {} {} {}", name, role, reason, anchor).to_lowercase();
    let dark = ["dark", "villain", "antagonist", "shadow", "ripper", "wraith", "neith", "boojay", "corveth", "solvaenkyr", "drawn"];
    let transient = s.contains("transient") || s.contains("temporary") || s.contains("encounter") || s.contains("background");
    transient && dark.iter().any(|d| s.contains(d))
}

fn split_names(text: &str) -> Vec<String> {
    text.split(',')
        .flat_map(|p| p.split(" and "))
        .map(clean_name)
        .filter(|s| s.len() > 1 && !s.contains("...") && !s.eq_ignore_ascii_case("all core cast"))
        .collect()
}

fn clean_name(s: &str) -> String {
    let mut x = s.replace("**", "").replace('[', "").replace(']', "");
    if let Some(pos) = x.find('(') { x.truncate(pos); }
    if let Some(pos) = x.find(':') { x.truncate(pos); }
    x = x.replace("Rylos/Boojay", "Rylos Vayne Johnson / Boojay");
    x = x.replace("Vaynie", "Vayne");
    x.trim_matches(|c: char| c == '-' || c == '*' || c == '.' || c.is_whitespace()).trim().to_string()
}

fn extract_known_names(text: &str) -> Vec<String> {
    let names = ["Pip", "Mack", "Xethrolund", "Rylos Vayne Johnson", "Boojay", "Rolzen", "Daisy May", "Majiskii", "Dervish", "Ursula", "Neith", "Solvaenkyr", "Wrenath", "Corveth", "Sorra", "Oswyn", "Calla", "Pethran", "Feritha", "Kyreal", "Kyrel", "Daevyn", "Elven", "Verath", "Loreth", "Seth", "Ithren", "Tavan", "Renath", "Selwyn", "Reval", "Thorn", "Drest", "Peln", "Maev", "Sova", "Alex", "Derris", "Lassith", "Glint"];
    let lower = text.to_lowercase();
    let mut out = Vec::new();
    for n in names {
        if lower.contains(&n.to_lowercase()) { out.push(n.to_string()); }
    }
    out
}

fn extract_markdown_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("### ") || t.starts_with("## ") || t.starts_with("- ") {
            let mut name = t.trim_start_matches('#').trim_start_matches('-').trim();
            if let Some(pos) = name.find('—') { name = &name[..pos]; }
            if let Some(pos) = name.find(':') { name = &name[..pos]; }
            let cleaned = clean_name(name);
            if cleaned.split_whitespace().count() <= 5 && cleaned.len() > 1 { out.push(cleaned); }
        }
    }
    out
}

fn extract_jsonish_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.contains("\"name\"") || t.contains("\"display_name\"") {
            if let Some(pos) = t.find(':') {
                let v = t[pos + 1..].trim().trim_matches(',').trim_matches('"');
                let cleaned = clean_name(v);
                if cleaned.len() > 1 { out.push(cleaned); }
            }
        }
    }
    out
}

fn index_map(header: &str) -> BTreeMap<String, usize> {
    header.split('\t').enumerate().map(|(i, s)| (s.trim().to_string(), i)).collect()
}

fn col(cols: &[&str], idx: &BTreeMap<String, usize>, key: &str) -> String {
    idx.get(key).and_then(|i| cols.get(*i)).map(|s| s.trim().to_string()).unwrap_or_default()
}

fn pick<T>(items: &[T], seed: usize) -> &T {
    &items[seed % items.len()]
}

fn tsv(s: &str) -> String {
    s.replace('\t', " ").replace('\n', " ").replace('\r', " ").trim().to_string()
}
