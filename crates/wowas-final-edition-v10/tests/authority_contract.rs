use std::path::Path;

#[test]
fn crate_points_to_active_wowas_canon_control() {
    let authority = wowas_final_edition_v10::wowas_authority_file();
    assert!(
        authority.ends_with("/canon/wowas_canon_v1.md"),
        "wowas_authority_file must point to active canon, got {authority}"
    );

    assert!(
        !authority.ends_with("/canon/CURRENT_APPLY_ORDER_v10.md"),
        "crate authority must not enter through legacy v10 apply order"
    );
}

#[test]
fn required_wowas_control_surfaces_exist() {
    for path in wowas_final_edition_v10::wowas_required_control_files() {
        assert!(
            Path::new(path).exists(),
            "missing required WoWaS surface: {path}"
        );
    }
}
