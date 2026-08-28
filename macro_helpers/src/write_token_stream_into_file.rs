#[path = "format_with_cargofmt.rs"]
mod format_with_cargofmt;
#[path = "proc_macro2_token_stream_ref.rs"]
mod proc_macro2_token_stream_ref;
#[path = "should_write_token_stream_into_file.rs"]
mod should_write_token_stream_into_file;
#[path = "try_maybe_write_token_stream_into_file.rs"]
mod try_maybe_write_token_stream_into_file;

pub use format_with_cargofmt::FormatWithCargofmt;
pub use proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef;
pub use should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile;
pub use try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file;
#[cfg(test)]
mod tests {
    #[test]
    fn try_maybe_write_token_stream_into_file_skips_when_flag_is_false() {
        let base = crate::domain_types::test_helper::test_path(
            crate::domain_types::test_helper::TestPathStem::new(constants_str::MACRO_HELPERS_SKIP),
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
        let base = crate::domain_types::test_helper::test_path(
            crate::domain_types::test_helper::TestPathStem::new(constants_str::MACRO_HELPERS_WRITE),
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
        crate::domain_types::test_helper::assert_file_content(
            crate::domain_types::test_helper::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_helper::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_helper::cleanup_test_file(path);
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_writes_tokens_when_enabled() {
        let base = crate::domain_types::test_helper::test_path(
            crate::domain_types::test_helper::TestPathStem::new(
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
        crate::domain_types::test_helper::assert_file_content(
            crate::domain_types::test_helper::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_helper::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_helper::cleanup_test_file(path);
    }
    #[test]
    fn try_maybe_write_token_stream_into_file_accepts_path_input() {
        let base = crate::domain_types::test_helper::test_path(
            crate::domain_types::test_helper::TestPathStem::new(
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
        crate::domain_types::test_helper::assert_file_content(
            crate::domain_types::test_helper::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_helper::ExpectedFileContent::new(&expected),
        );
        crate::domain_types::test_helper::cleanup_test_file(path);
    }
    #[test]
    #[cfg_attr(miri, ignore = "Miri does not support spawning the rustfmt subprocess")]
    fn try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled() {
        let base = crate::domain_types::test_helper::test_path(
            crate::domain_types::test_helper::TestPathStem::new(
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
        crate::domain_types::test_helper::assert_file_content(
            crate::domain_types::test_helper::StdAssertFilePath::new(path.as_ref()),
            crate::domain_types::test_helper::ExpectedFileContent::new(
                constants_str::STRUCT_A_NEWLINE,
            ),
        );
        crate::domain_types::test_helper::cleanup_test_file(path);
    }
}
