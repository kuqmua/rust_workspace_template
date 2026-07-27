#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum FormatWithCargofmt {
    False,
    True,
}
#[derive(Debug, Copy, Clone, serde::Deserialize, optml::Optml)]
pub enum ShouldWriteTokenStreamIntoFile {
    False,
    True,
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct ProcMacro2TokenStreamRef<'ts_lt>(&'ts_lt proc_macro2::TokenStream);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
struct StdRustfmtPath<'path_lt>(&'path_lt std::path::Path);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
struct ShouldWriteTokenStreamFlag(bool);
#[allow(clippy::single_call_fn)] // rustfmt execution is isolated so io/process errors stay localized and easy to test
fn try_run_rustfmt(path: StdRustfmtPath<'_>) -> std::io::Result<()> {
    let mut command = crate::tool_command::ToolCommand::new(
        crate::tool_command::ToolProgramRef::from(str_constants::RUSTFMT),
    );
    let path_text = path.0.to_string_lossy();
    let status = command
        .arg(crate::tool_command::ToolArgRef::from(path_text.as_ref()))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "rustfmt failed for {}",
            path.0.display()
        )))
    }
}
#[allow(clippy::single_call_fn)] // keeps ShouldWriteTokenStreamIntoFile flag interpretation centralized
fn should_write_token_stream_flag(v: ShouldWriteTokenStreamIntoFile) -> ShouldWriteTokenStreamFlag {
    ShouldWriteTokenStreamFlag::from(matches!(v, ShouldWriteTokenStreamIntoFile::True))
}
#[allow(clippy::single_call_fn)] // centralizes token-to-file write mapping and outcome extraction
fn try_write_token_stream_into_file<P>(
    file_name: P,
    ts: ProcMacro2TokenStreamRef<'_>,
) -> std::io::Result<crate::string_writer::WritePathOutcome>
where
    P: AsRef<std::path::Path>,
{
    let string_cnt = ts.as_ref().to_string();
    crate::string_writer::try_write_string_into_file_with_outcome(
        file_name,
        crate::string_writer::StringFileContentRef::from(string_cnt.as_str()),
    )
}
pub fn try_maybe_write_token_stream_into_file<P>(
    should_write_token_stream_into_file: ShouldWriteTokenStreamIntoFile,
    file_name: P,
    ts: ProcMacro2TokenStreamRef<'_>,
    format_with_cargofmt: &FormatWithCargofmt,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    if !should_write_token_stream_flag(should_write_token_stream_into_file).0 {
        return Ok(());
    }
    let wr_outcome = try_write_token_stream_into_file(file_name, ts)?;
    if bool::from(wr_outcome.is_changed())
        && matches!(format_with_cargofmt, FormatWithCargofmt::True)
    {
        try_run_rustfmt(StdRustfmtPath::from(wr_outcome.path().as_ref()))?;
    }
    Ok(())
}
pub fn maybe_write_token_stream_into_file<P>(
    should_write_token_stream_into_file: ShouldWriteTokenStreamIntoFile,
    file_name: P,
    ts: ProcMacro2TokenStreamRef<'_>,
    format_with_cargofmt: &FormatWithCargofmt,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    try_maybe_write_token_stream_into_file(
        should_write_token_stream_into_file,
        file_name,
        ts,
        format_with_cargofmt,
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn maybe_write_token_stream_into_file_skips_when_flag_is_false() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_SKIP,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            str_constants::STRUCT_SKIPWRITE.parse().expect("5994e7e2");
        super::maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::False,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("5ecc3880");
        let _error = std::fs::metadata(&path).expect_err(str_constants::VALUE_7BE5F201);
    }
    #[test]
    fn maybe_write_token_stream_into_file_writes_tokens_when_flag_is_true() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            str_constants::STRUCT_DIDWRITE.parse().expect("6c20f49a");
        let expected = ts.to_string();
        super::maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("04f83dc1");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::test_hlp::cleanup_test_file(path);
    }
    #[test]
    fn should_write_token_stream_flag_maps_true_and_false_flags() {
        assert!(
            !super::should_write_token_stream_flag(super::ShouldWriteTokenStreamIntoFile::False).0
        );
        assert!(
            super::should_write_token_stream_flag(super::ShouldWriteTokenStreamIntoFile::True).0
        );
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_writes_tokens_when_enabled() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_TRY_WRITE,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            str_constants::STRUCT_TRYDIDWRITE.parse().expect("f771ac2d");
        let expected = ts.to_string();
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("6fee9f6f");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::test_hlp::cleanup_test_file(path);
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_accepts_path_input() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_TRY_WRITE_PATH,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            str_constants::STRUCT_PATHINPUT.parse().expect("f9b0cd83");
        let expected = ts.to_string();
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("f341cde7");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::test_hlp::cleanup_test_file(path);
    }
    #[test]
    #[cfg_attr(miri, ignore = "Miri does not support spawning the rustfmt subprocess")]
    fn try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_TRY_RUN_RUSTFMT,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, str_constants::STRUCT_B).expect("7091840d");
        let ts: proc_macro2::TokenStream = str_constants::STRUCT_A.parse().expect("0f30ca53");
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::True,
        )
        .expect("00a995a4");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::test_hlp::ExpectedFileContent::new(str_constants::STRUCT_A_NEWLINE),
        );
        crate::test_hlp::cleanup_test_file(path);
    }
}
