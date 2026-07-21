#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr::from(str_constants::CLIPPY_DRIVER),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr::from(str_constants::VALUE_8895CA50),
        super::types::StaticStrSliceRef::from(
            str_constants::CODE_STYLE_CLIPPY_LINT_EXCEPTIONS.as_slice(),
        ),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let exceptions = [
        str_constants::FUZZY_PROVENANCE_CASTS,
        str_constants::IMPLICIT_PROVENANCE_CASTS,
        str_constants::LOSSY_PROVENANCE_CASTS,
        str_constants::MULTIPLE_SUPERTRAIT_UPCASTABLE,
        str_constants::MUST_NOT_SUSPEND,
        str_constants::NON_EXHAUSTIVE_OMITTED_PATTERNS,
        str_constants::SUPERTRAIT_ITEM_SHADOWING_DEFINITION,
        str_constants::SUPERTRAIT_ITEM_SHADOWING_USAGE,
        str_constants::AARCH_64_SOFTFLOAT_NEON,
        str_constants::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
        str_constants::TEST_UNSTABLE_LINT,
        str_constants::RESOLVING_TO_ITEMS_SHADOWING_SUPERTRAIT_ITEMS,
        str_constants::SHADOWING_SUPERTRAIT_ITEMS,
        str_constants::UNQUALIFIED_LOCAL_IMPORTS, //need to use some kind of different test flag or something for this
        str_constants::UNREACHABLE_CFG_SELECT_PREDICATES,
        str_constants::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
        str_constants::LINKER_INFO,
        str_constants::DUPLICATE_FEATURES,
        str_constants::DEPRECATED_LLVM_INTRINSIC,
        str_constants::TAIL_CALL_TRACK_CALLER,
    ];
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        super::types::StaticStr::from(str_constants::RUSTC),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr::from(str_constants::VALUE_3C20B457),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
