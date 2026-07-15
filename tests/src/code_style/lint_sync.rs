#[test]
fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
    super::assert_workspace_lints_match(
        super::RustOrClippy::Clippy,
        super::types::StaticStr(str_constants::expr::S_1091),
        super::types::AnalyzerBool::from(true),
        super::types::StaticStr(str_constants::expr::S_0492),
        super::types::StaticStrSliceRef::from(
            str_constants::code_style::CLIPPY_LINT_EXCEPTIONS.as_slice(),
        ),
    );
}
#[test]
fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
    let exceptions = [
        str_constants::expr::S_1342,
        str_constants::expr::S_1414,
        str_constants::expr::S_1471,
        str_constants::expr::S_1529,
        str_constants::expr::S_1530,
        str_constants::expr::S_1554,
        str_constants::expr::S_1780,
        str_constants::expr::S_1781,
        str_constants::expr::S_0902,
        str_constants::expr::S_1169,
        str_constants::expr::S_1810,
        str_constants::expr::S_1667,
        str_constants::expr::S_1737,
        str_constants::expr::S_1860, //need to use some kind of different test flag or something for this
        str_constants::expr::S_1861,
        str_constants::expr::S_1169,
        str_constants::expr::S_1459,
        str_constants::expr::S_1202,
        str_constants::expr::S_1176,
        str_constants::expr::S_1795,
    ];
    super::assert_workspace_lints_match(
        super::RustOrClippy::Rust,
        super::types::StaticStr(str_constants::expr::S_1697),
        super::types::AnalyzerBool::default(),
        super::types::StaticStr(str_constants::expr::S_0292),
        //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
        super::types::StaticStrSliceRef::from(exceptions.as_slice()),
    );
}
