#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum FormatWithCargofmt {
    False,
    True,
}
#[derive(Debug, Copy, Clone, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub enum ShouldWriteTokenStreamIntoFile {
    False,
    True,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ProcMacro2TokenStreamRef<'ts_lt>(&'ts_lt proc_macro2::TokenStream);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct ShouldWriteTokenStreamFlag(bool);
#[allow(clippy::single_call_fn)] // production call plus direct unit-test calls make this a multi-call helper across the repository
fn should_write_token_stream_flag(v: ShouldWriteTokenStreamIntoFile) -> ShouldWriteTokenStreamFlag {
    ShouldWriteTokenStreamFlag::from(matches!(v, ShouldWriteTokenStreamIntoFile::True))
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
    let string_cnt = ts.as_ref().to_string();
    let wr_outcome = crate::domain_types::string_writer::try_write_string_into_file_with_outcome(
        file_name,
        crate::domain_types::string_writer::StringFileContentRef::from(string_cnt.as_str()),
    )?;
    if bool::from(wr_outcome.is_changed())
        && matches!(format_with_cargofmt, FormatWithCargofmt::True)
    {
        let path = wr_outcome.path();
        let mut command = crate::domain_types::tool_command::ToolCommand::new(
            crate::domain_types::tool_command::ToolProgramRef::from(constants_str::RUSTFMT),
        );
        let path_text = path.as_ref().to_string_lossy();
        let status = command
            .arg(crate::domain_types::tool_command::ToolArgRef::from(
                path_text.as_ref(),
            ))
            .status()?;
        if !status.success() {
            return Err(std::io::Error::other(format!(
                "rustfmt failed for {}",
                path.as_ref().display()
            )));
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn try_maybe_write_token_stream_into_file_skips_when_flag_is_false() {
        let base = crate::domain_types::test_hlp::test_path(
            crate::domain_types::test_hlp::TestPathStem::new(constants_str::MACRO_HELPERS_SKIP),
        );
        let path = crate::domain_types::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            constants_str::STRUCT_SKIPWRITE.parse().expect("5994e7e2 try_maybe_write_token_stream_into_file_skips_when_flag_is_false invariant must hold");
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::False,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("5ecc3880 try_maybe_write_token_stream_into_file_skips_when_flag_is_false invariant must hold");
        let _error = std::fs::metadata(&path).expect_err(constants_str::VALUE_7BE5F201);
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_writes_tokens_when_flag_is_true() {
        let base = crate::domain_types::test_hlp::test_path(
            crate::domain_types::test_hlp::TestPathStem::new(constants_str::MACRO_HELPERS_WRITE),
        );
        let path = crate::domain_types::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            constants_str::STRUCT_DIDWRITE.parse().expect("6c20f49a try_maybe_write_token_stream_into_file_writes_tokens_when_flag_is_true invariant must hold");
        let expected = ts.to_string();
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("04f83dc1 try_maybe_write_token_stream_into_file_writes_tokens_when_flag_is_true invariant must hold");
        crate::domain_types::test_hlp::assert_file_content(
            crate::domain_types::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_hlp::cleanup_test_file(path);
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
        let base = crate::domain_types::test_hlp::test_path(
            crate::domain_types::test_hlp::TestPathStem::new(
                constants_str::MACRO_HELPERS_TRY_WRITE,
            ),
        );
        let path = crate::domain_types::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            constants_str::STRUCT_TRYDIDWRITE.parse().expect("f771ac2d try_maybe_write_token_stream_into_file_writes_tokens_when_enabled invariant must hold");
        let expected = ts.to_string();
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("6fee9f6f try_maybe_write_token_stream_into_file_writes_tokens_when_enabled invariant must hold");
        crate::domain_types::test_hlp::assert_file_content(
            crate::domain_types::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_hlp::cleanup_test_file(path);
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_accepts_path_input() {
        let base = crate::domain_types::test_hlp::test_path(
            crate::domain_types::test_hlp::TestPathStem::new(
                constants_str::MACRO_HELPERS_TRY_WRITE_PATH,
            ),
        );
        let path = crate::domain_types::rs_file_path::rs_file_path(&base);
        let ts: proc_macro2::TokenStream =
            constants_str::STRUCT_PATHINPUT.parse().expect("f9b0cd83 try_maybe_write_token_stream_into_file_accepts_path_input invariant must hold");
        let expected = ts.to_string();
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::False,
        )
        .expect("f341cde7 try_maybe_write_token_stream_into_file_accepts_path_input invariant must hold");
        crate::domain_types::test_hlp::assert_file_content(
            crate::domain_types::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_hlp::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_hlp::cleanup_test_file(path);
    }
    #[test]
    #[cfg_attr(miri, ignore = "Miri does not support spawning the rustfmt subprocess")]
    fn try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled() {
        let base = crate::domain_types::test_hlp::test_path(
            crate::domain_types::test_hlp::TestPathStem::new(
                constants_str::MACRO_HELPERS_TRY_RUN_RUSTFMT,
            ),
        );
        let path = crate::domain_types::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, constants_str::STRUCT_B).expect("7091840d try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled invariant must hold");
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_A.parse().expect("0f30ca53 try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled invariant must hold");
        super::try_maybe_write_token_stream_into_file(
            super::ShouldWriteTokenStreamIntoFile::True,
            &base,
            super::ProcMacro2TokenStreamRef::from(&ts),
            &super::FormatWithCargofmt::True,
        )
        .expect("00a995a4 try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled invariant must hold");
        crate::domain_types::test_hlp::assert_file_content(
            crate::domain_types::test_hlp::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_hlp::ExpectedFileContent::new(
                constants_str::STRUCT_A_NEWLINE,
            ),
        );
        crate::domain_types::test_hlp::cleanup_test_file(path);
    }
}
