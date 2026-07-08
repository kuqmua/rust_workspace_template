#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        "clippy-driver",
        true,
        "8895ca50",
        &super::CLIPPY_LINT_EXCEPTIONS,
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        "rustc",
        false,
        "3c20b457",
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        &[
            "fuzzy_provenance_casts",
            "lossy_provenance_casts",
            "multiple_supertrait_upcastable",
            "must_not_suspend",
            "non_exhaustive_omitted_patterns",
            "supertrait_item_shadowing_definition",
            "supertrait_item_shadowing_usage",
            "aarch_64_softfloat_neon",
            "dflt_overrides_dflt_fields",
            "test_unstable_lint",
            "resolving_to_items_shadowing_supertrait_items",
            "shadowing_supertrait_items",
            "unqualified_local_imports", //need to use some kind of different test flag or something for this
            "unreachable_cfg_select_predicates",
            "default_overrides_default_fields",
            "linker_info",
            "duplicate_features",
            "deprecated_llvm_intrinsic",
            "tail_call_track_caller",
        ],
    );
}
