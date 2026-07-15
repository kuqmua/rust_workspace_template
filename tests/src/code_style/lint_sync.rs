#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr(str_constants::text::CLIPPY_DRIVER),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr(str_constants::text::VALUE_8895CA50),
        super::types::StaticStrSliceRef::from(
            str_constants::code_style::CLIPPY_LINT_EXCEPTIONS.as_slice(),
        ),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let exceptions = [
        str_constants::text::FUZZY_PROVENANCE_CASTS,
        str_constants::text::IMPLICIT_PROVENANCE_CASTS,
        str_constants::text::LOSSY_PROVENANCE_CASTS,
        str_constants::text::MULTIPLE_SUPERTRAIT_UPCASTABLE,
        str_constants::text::MUST_NOT_SUSPEND,
        str_constants::text::NON_EXHAUSTIVE_OMITTED_PATTERNS,
        str_constants::text::SUPERTRAIT_ITEM_SHADOWING_DEFINITION,
        str_constants::text::SUPERTRAIT_ITEM_SHADOWING_USAGE,
        str_constants::text::AARCH_64_SOFTFLOAT_NEON,
        str_constants::text::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
        str_constants::text::TEST_UNSTABLE_LINT,
        str_constants::text::RESOLVING_TO_ITEMS_SHADOWING_SUPERTRAIT_ITEMS,
        str_constants::text::SHADOWING_SUPERTRAIT_ITEMS,
        str_constants::text::UNQUALIFIED_LOCAL_IMPORTS, //need to use some kind of different test flag or something for this
        str_constants::text::UNREACHABLE_CFG_SELECT_PREDICATES,
        str_constants::text::DEFAULT_OVERRIDES_DEFAULT_FIELDS,
        str_constants::text::LINKER_INFO,
        str_constants::text::DUPLICATE_FEATURES,
        str_constants::text::DEPRECATED_LLVM_INTRINSIC,
        str_constants::text::TAIL_CALL_TRACK_CALLER,
    ];
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        super::types::StaticStr(str_constants::text::RUSTC),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr(str_constants::text::VALUE_3C20B457),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
