#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr("clippy-driver"),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr("8895ca50"),
        super::types::StaticStrSliceRef::from(super::CLIPPY_LINT_EXCEPTIONS.as_slice()),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let exceptions = [
        "fuzzy_provenance_casts",
        "implicit_provenance_casts",
        "lossy_provenance_casts",
        "multiple_supertrait_upcastable",
        "must_not_suspend",
        "non_exhaustive_omitted_patterns",
        "supertrait_item_shadowing_definition",
        "supertrait_item_shadowing_usage",
        "aarch_64_softfloat_neon",
        "default_overrides_default_fields",
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
    ];
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        super::types::StaticStr("rustc"),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr("3c20b457"),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
