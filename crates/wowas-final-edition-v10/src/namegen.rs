#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameCandidate {
    pub name: String,
    pub score: u16,
    pub rejected: bool,
    pub reason: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameGenRequest<'a> {
    pub seed: &'a str,
    pub house_pressure: &'a str,
    pub source_anchor: &'a str,
    pub story_role: &'a str,
    pub region: &'a str,
    pub minimum_candidates: usize,
}

const MINIMUM_REVIEW_DRAWS: usize = 30;
const OPENERS: &[&str] = &[
    "Aer", "Asha", "Bren", "Cael", "Daro", "Eren", "Falon", "Ghara", "Halen", "Ivara", "Joren",
    "Kaelis", "Liora", "Maeron", "Nivara", "Orren", "Paelin", "Quorin", "Ravena", "Sorelan",
];
const MID: &[&str] = &[
    "ael", "amar", "anor", "avel", "brin", "coria", "dalen", "elion", "endar", "evara", "fira",
    "galen", "halor", "ian", "indra", "jora", "kelan", "luth", "maelor", "nara",
];
const TAILS: &[&str] = &[
    "ael", "aen", "ara", "arel", "aria", "avel", "dion", "dria", "el", "ella", "elle", "en", "eth",
    "ian", "iel", "ienne", "ion", "ira", "ith", "iven",
];
const ECHO: &[&str] = &[
    "ash", "bell", "brook", "cairn", "cinder", "crown", "ember", "fen", "forge", "glass", "grove",
    "hollow", "ivy", "lark", "marrow", "mire", "moss", "orchard", "quartz", "river",
];

fn hash64(text: &str) -> u64 {
    let mut h = 14695981039346656037_u64;
    for b in text.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(1099511628211);
    }
    h
}

fn roll(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

fn pick<'a>(state: &mut u64, items: &'a [&'a str]) -> &'a str {
    items[(roll(state) as usize) % items.len()]
}

fn clean(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn titlecase(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for c in s.chars() {
        if up {
            for x in c.to_uppercase() {
                out.push(x);
            }
            up = false;
        } else {
            for x in c.to_lowercase() {
                out.push(x);
            }
        }
    }
    out
}

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

fn vowel_ok(name: &str) -> bool {
    let vowels = name
        .chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y'))
        .count();
    let letters = name.chars().filter(|c| c.is_alphabetic()).count();
    letters >= 7 && vowels >= 3 && vowels * 4 >= letters
}

fn too_close(name: &str, source: &str, pressure: &str) -> bool {
    let n = clean(name);
    let s = clean(source);
    let p = clean(pressure);
    (!s.is_empty() && (n == s || n.contains(&s) || s.contains(&n) || common_prefix(&n, &s) >= 5))
        || (!p.is_empty()
            && (n.starts_with(&p.chars().take(4).collect::<String>())
                || common_prefix(&n, &p) >= 5))
}

fn build(req: &NameGenRequest<'_>, state: &mut u64, draw: usize) -> String {
    let mut parts = vec![pick(state, OPENERS).to_string()];
    for _ in 0..(1 + ((roll(state) as usize + draw) % 3)) {
        parts.push(pick(state, MID).to_string());
    }
    if roll(state) % 10 < 3 {
        let echo = ECHO[((hash64(req.house_pressure) ^ roll(state)) as usize) % ECHO.len()];
        parts.push(echo.to_string());
    }
    let tail =
        TAILS[((roll(state) ^ hash64(req.story_role) ^ hash64(req.region)) as usize) % TAILS.len()];
    parts.push(tail.to_string());
    titlecase(&parts.join(""))
}

fn score(name: &str, req: &NameGenRequest<'_>) -> NameCandidate {
    if name.len() < 8 {
        return NameCandidate {
            name: name.into(),
            score: 0,
            rejected: true,
            reason: "too_clipped",
        };
    }
    if name.len() > 25 {
        return NameCandidate {
            name: name.into(),
            score: 0,
            rejected: true,
            reason: "too_long",
        };
    }
    if !vowel_ok(name) {
        return NameCandidate {
            name: name.into(),
            score: 0,
            rejected: true,
            reason: "weak_phonetic_balance",
        };
    }
    if too_close(name, req.source_anchor, req.house_pressure) {
        return NameCandidate {
            name: name.into(),
            score: 0,
            rejected: true,
            reason: "too_close",
        };
    }
    let mut s = 50;
    if (10..=19).contains(&name.len()) {
        s += 18;
    }
    if ["el", "en", "ia", "ion", "elle", "ael"]
        .iter()
        .any(|x| name.ends_with(x))
    {
        s += 10;
    }
    NameCandidate {
        name: name.into(),
        score: s,
        rejected: false,
        reason: "accepted",
    }
}

pub fn reviewed_name_candidates(req: &NameGenRequest<'_>) -> Vec<NameCandidate> {
    let target = req.minimum_candidates.max(MINIMUM_REVIEW_DRAWS);
    let mut state = hash64(req.seed)
        ^ hash64(req.house_pressure)
        ^ hash64(req.source_anchor)
        ^ hash64(req.story_role)
        ^ hash64(req.region);
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::with_capacity(target);
    let mut draw = 0;
    while out.len() < target && draw < target * 12 {
        let name = build(req, &mut state, draw);
        draw += 1;
        if seen.insert(clean(&name)) {
            out.push(score(&name, req));
        }
    }
    out.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));
    out
}

pub fn best_reviewed_name(req: &NameGenRequest<'_>) -> Option<NameCandidate> {
    reviewed_name_candidates(req)
        .into_iter()
        .find(|c| !c.rejected)
}
