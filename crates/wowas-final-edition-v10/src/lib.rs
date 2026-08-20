pub const WOWAS_PACK_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/canon");

/// Active canon control for the reconstruction branch.
pub const WOWAS_ACTIVE_CANON_CONTROL: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/wowas_canon_v1.md");

/// Cohesive canon body retained as a directly readable supporting surface.
pub const WOWAS_COHESIVE_CANON_V1: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/wowas_canon_v1.md");

/// v14 source-of-truth registry; routing/audit support, not a patch chain.
pub const WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md"
);

/// The active prose/tone contract is resolved from the installed control surface.
pub const WOWAS_PROSE_AND_TONE_GUIDE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/prose_and_tone_guide_v14.json"
);

/// Active canonical scene inventory.
pub const WOWAS_CLEAN_SCENE_INDEX: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_clean_scene_index_v2.tsv"
);

/// Active 33-book canonical order.
pub const WOWAS_BOOK_SPINE_33: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/active/book_spine_33.tsv"
);

/// Active 33-book character/timeline lattice.
pub const WOWAS_CHARACTER_TIMELINE_LATTICE_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/active/character_timeline_lattice_v14_33.tsv"
);

/// Active inclusion/exclusion law surfaces.
pub const WOWAS_CANON_LAWS: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/active/canon_laws.tsv");
pub const WOWAS_CANON_BLOCKLIST: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/active/canon_blocklist.tsv"
);

/// Active character generation/reconciliation control.
pub const WOWAS_CHARACTER_GENERATION_REVIEW_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/character_generation_review_v14.tsv"
);

/// Actual book/canon tree. Prose generation resolves through this tree.
pub const WOWAS_CANONICAL_STORY_TREE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/canonical_story_tree");
pub const WOWAS_BOOKS_ROOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/canonical_story_tree/books"
);
pub const WOWAS_BOOK_01_ROOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/canonical_story_tree/books/Book_01_Choices_Make_World"
);
pub const WOWAS_SCENE_HEADING_INDEX: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/canonical_story_tree/_scene_heading_index.tsv"
);

/// Legacy authority files remain callable only for audit/history compatibility.
pub const WOWAS_AUTHORITY_MANIFEST_V13: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_final_authority_manifest_v13.json"
);
pub const WOWAS_AUTHORITY_ROUTER_V13: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_final_authority_system_v13.md"
);

pub fn wowas_pack_root() -> &'static str {
    WOWAS_PACK_ROOT
}
pub fn wowas_authority_file() -> &'static str {
    WOWAS_ACTIVE_CANON_CONTROL
}
pub fn wowas_active_canon_control() -> &'static str {
    WOWAS_ACTIVE_CANON_CONTROL
}
pub fn wowas_cohesive_canon_v1() -> &'static str {
    WOWAS_COHESIVE_CANON_V1
}
pub fn wowas_source_of_truth_registry_v14() -> &'static str {
    WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14
}
pub fn wowas_authority_manifest_v13() -> &'static str {
    WOWAS_AUTHORITY_MANIFEST_V13
}
pub fn wowas_authority_router_v13() -> &'static str {
    WOWAS_AUTHORITY_ROUTER_V13
}
pub fn wowas_canonical_story_tree() -> &'static str {
    WOWAS_CANONICAL_STORY_TREE
}
pub fn wowas_books_root() -> &'static str {
    WOWAS_BOOKS_ROOT
}
pub fn wowas_book_01_root() -> &'static str {
    WOWAS_BOOK_01_ROOT
}
pub fn wowas_scene_heading_index() -> &'static str {
    WOWAS_SCENE_HEADING_INDEX
}
pub fn wowas_clean_scene_index() -> &'static str {
    WOWAS_CLEAN_SCENE_INDEX
}
pub fn wowas_book_spine_33() -> &'static str {
    WOWAS_BOOK_SPINE_33
}
pub fn wowas_character_timeline_lattice_v14() -> &'static str {
    WOWAS_CHARACTER_TIMELINE_LATTICE_V14
}
pub fn wowas_canon_laws() -> &'static str {
    WOWAS_CANON_LAWS
}
pub fn wowas_canon_blocklist() -> &'static str {
    WOWAS_CANON_BLOCKLIST
}
pub fn wowas_character_generation_review_v14() -> &'static str {
    WOWAS_CHARACTER_GENERATION_REVIEW_V14
}

pub fn wowas_prose_and_tone_guide() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/canon/control/prose_and_tone_guide_v14.json"
    ))
}

pub fn wowas_install_summary() -> &'static str {
    "WoWaS active canon: 33-book spine, active scene index, 33-book character lattice, canon laws/blocklist, canonical story tree. Legacy patch/version surfaces are audit-only."
}

pub fn wowas_required_control_files() -> &'static [&'static str] {
    &[
        WOWAS_ACTIVE_CANON_CONTROL,
        WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14,
        WOWAS_PROSE_AND_TONE_GUIDE,
        WOWAS_COHESIVE_CANON_V1,
        WOWAS_BOOK_SPINE_33,
        WOWAS_CLEAN_SCENE_INDEX,
        WOWAS_CHARACTER_TIMELINE_LATTICE_V14,
        WOWAS_CANON_LAWS,
        WOWAS_CANON_BLOCKLIST,
        WOWAS_CHARACTER_GENERATION_REVIEW_V14,
        WOWAS_CANONICAL_STORY_TREE,
        WOWAS_SCENE_HEADING_INDEX,
    ]
}

pub fn wowas_generation_source_order() -> &'static [&'static str] {
    &[
        "canon/active/CANON_AUTHORITY.md",
        "canon/active/book_spine_33.tsv",
        "canon/active/canon_laws.tsv",
        "canon/active/canon_blocklist.tsv",
        "canon/active/character_timeline_lattice_v14_33.tsv",
        "canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md",
        "canon/control/prose_and_tone_guide_v14.json",
        "canon/control/character_generation_review_v14.tsv",
        "canon/wowas_canon_v1.md",
        "canon/wowas_clean_scene_index_v2.tsv",
        "canon/canonical_story_tree",
        "canon/canonical_story_tree/_scene_heading_index.tsv",
        "canon/canonical_story_tree/books",
        "canon/canonical_story_tree/characters",
        "canon/canonical_story_tree/world",
    ]
}
