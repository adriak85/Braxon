pub const WOWAS_PACK_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/canon");

/// v14 is the single first-load authority surface for the current WoWaS lane.
pub const WOWAS_ACTIVE_CANON_CONTROL: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/WOWAS_CANON_AUTHORITY_v14.md");

/// Cohesive canon body loaded after the v14 authority surface.
pub const WOWAS_COHESIVE_CANON_V1: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/wowas_canon_v1.md");

/// Source-of-truth registry installed from v14 authority and applied patch law.
pub const WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md"
);

/// Installed prose/tone control. This is the final home; not a patch file.
pub const WOWAS_PROSE_AND_TONE_GUIDE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/prose_and_tone_guide_v14.json"
);

/// Patch ingestion ledger. Patch folders are law while ingested, then history only.
pub const WOWAS_PATCH_INGESTION_LEDGER_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/PATCH_INGESTION_LEDGER_v14.md"
);

/// Magic system final-home control derived from v10 patch law and v14 authority.
pub const WOWAS_MAGIC_SYSTEM_CONTROL_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/magic_system_control_v14.md"
);

/// Character/scene placement control for the active 25-book lattice.
pub const WOWAS_CHARACTER_PLACEMENT_CONTROL_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/control/character_placement_control_v14.md"
);

/// v13 router/registry layer retained for authority routing, audit, and support lookup.
pub const WOWAS_AUTHORITY_MANIFEST_V13: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_final_authority_manifest_v13.json"
);

pub const WOWAS_AUTHORITY_ROUTER_V13: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_final_authority_system_v13.md"
);

/// Legacy registry retained for audit/history only; v14 registry is authoritative.
pub const WOWAS_PATCH_ABSORPTION_REGISTRY_V13: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_patch_absorption_registry_v13.md"
);

/// Actual book/canon tree. Prose generation must resolve through this tree,
/// not old loose generated prose packets.
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

pub const WOWAS_CLEAN_SCENE_INDEX: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_clean_scene_index_v2.tsv"
);

pub const WOWAS_CHARACTER_TIMELINE_LATTICE_V14: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv"
);

pub const WOWAS_MONSTER_SPECIES_REGISTRY_V8: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/wowas_monster_species_registry_v8.tsv"
);

/// Legacy apply-order files are retained for audit/history, not as crate authority.
pub const WOWAS_LEGACY_APPLY_ORDER_V10: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/CURRENT_APPLY_ORDER_v10.md"
);

pub const WOWAS_LEGACY_APPLY_ORDER_V11: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/canon/CURRENT_APPLY_ORDER_v11.md"
);

pub const WOWAS_INSTALL_SUMMARY_V10: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/INSTALL_SUMMARY_v10.txt");

pub const WOWAS_INSTALL_SUMMARY_V11: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/canon/INSTALL_SUMMARY_v11.txt");

pub fn wowas_pack_root() -> &'static str {
    WOWAS_PACK_ROOT
}

/// Backward-compatible function name, now corrected to v14 first-load authority.
pub fn wowas_authority_file() -> &'static str {
    WOWAS_ACTIVE_CANON_CONTROL
}

pub fn wowas_active_canon_control() -> &'static str {
    WOWAS_ACTIVE_CANON_CONTROL
}

pub fn wowas_cohesive_canon_v1() -> &'static str {
    WOWAS_COHESIVE_CANON_V1
}

<<<<<<< Updated upstream
pub fn wowas_source_of_truth_registry_v14() -> &'static str {
    WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14
}

=======
>>>>>>> Stashed changes
pub fn wowas_authority_manifest_v13() -> &'static str {
    WOWAS_AUTHORITY_MANIFEST_V13
}

pub fn wowas_authority_router_v13() -> &'static str {
    WOWAS_AUTHORITY_ROUTER_V13
}

pub fn wowas_patch_absorption_registry_v13() -> &'static str {
    WOWAS_PATCH_ABSORPTION_REGISTRY_V13
}

pub fn wowas_patch_ingestion_ledger_v14() -> &'static str {
    WOWAS_PATCH_INGESTION_LEDGER_V14
}

pub fn wowas_magic_system_control_v14() -> &'static str {
    WOWAS_MAGIC_SYSTEM_CONTROL_V14
}

pub fn wowas_character_placement_control_v14() -> &'static str {
    WOWAS_CHARACTER_PLACEMENT_CONTROL_V14
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

pub fn wowas_character_timeline_lattice_v14() -> &'static str {
    WOWAS_CHARACTER_TIMELINE_LATTICE_V14
}

pub fn wowas_monster_species_registry_v8() -> &'static str {
    WOWAS_MONSTER_SPECIES_REGISTRY_V8
}

pub fn wowas_prose_and_tone_guide() -> &'static str {
    WOWAS_PROSE_AND_TONE_GUIDE
}

pub fn wowas_install_summary() -> &'static str {
    WOWAS_INSTALL_SUMMARY_V11
}

pub fn wowas_required_control_files() -> &'static [&'static str] {
    &[
        WOWAS_ACTIVE_CANON_CONTROL,
<<<<<<< Updated upstream
        WOWAS_SOURCE_OF_TRUTH_REGISTRY_V14,
        WOWAS_PROSE_AND_TONE_GUIDE,
        WOWAS_PATCH_INGESTION_LEDGER_V14,
        WOWAS_MAGIC_SYSTEM_CONTROL_V14,
        WOWAS_CHARACTER_PLACEMENT_CONTROL_V14,
=======
>>>>>>> Stashed changes
        WOWAS_COHESIVE_CANON_V1,
        WOWAS_AUTHORITY_MANIFEST_V13,
        WOWAS_AUTHORITY_ROUTER_V13,
        WOWAS_CANONICAL_STORY_TREE,
        WOWAS_BOOKS_ROOT,
        WOWAS_BOOK_01_ROOT,
        WOWAS_SCENE_HEADING_INDEX,
        WOWAS_CLEAN_SCENE_INDEX,
        WOWAS_CHARACTER_TIMELINE_LATTICE_V14,
        WOWAS_MONSTER_SPECIES_REGISTRY_V8,
    ]
}

pub fn wowas_generation_source_order() -> &'static [&'static str] {
    &[
        "canon/WOWAS_CANON_AUTHORITY_v14.md",
<<<<<<< Updated upstream
        "canon/control/WOWAS_SOURCE_OF_TRUTH_REGISTRY_v14.md",
        "canon/control/prose_and_tone_guide_v14.json",
        "canon/control/PATCH_INGESTION_LEDGER_v14.md",
        "canon/control/magic_system_control_v14.md",
        "canon/control/character_placement_control_v14.md",
=======
>>>>>>> Stashed changes
        "canon/wowas_canon_v1.md",
        "canon/wowas_character_timeline_lattice_UNIFIED_v14.tsv",
        "canon/wowas_clean_scene_index_v2.tsv",
        "canon/wowas_monster_species_registry_v8.tsv",
        "canon/canonical_story_tree",
        "canon/canonical_story_tree/_scene_heading_index.tsv",
        "canon/canonical_story_tree/books",
        "canon/canonical_story_tree/characters",
        "canon/canonical_story_tree/world",
        "canon/wowas_final_authority_system_v13.md",
    ]
}
