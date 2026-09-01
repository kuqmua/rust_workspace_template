#[cfg(test)]
mod tests {
    #[test]
    fn test_try_maybe_write_token_stream_into_file_skips_when_flag_is_false() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_SKIP,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_SKIPWRITE
            .parse()
            .expect(constants_str::DIAGNOSTIC_5994E7E2);
        crate::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::False,
            &base,
            crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&ts),
            &crate::format_with_cargofmt::FormatWithCargofmt::False,
        )
        .expect(constants_str::DIAGNOSTIC_5ECC3880);
        let _error = std::fs::metadata(&path).expect_err(constants_str::VALUE_7BE5F201);
    }
    #[test]
    fn test_try_maybe_write_token_stream_into_file_writes_tokens_when_flag_is_true() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_DIDWRITE
            .parse()
            .expect(constants_str::DIAGNOSTIC_6C20F49A);
        let expected = ts.to_string();
        crate::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True,
            &base,
            crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&ts),
            &crate::format_with_cargofmt::FormatWithCargofmt::False,
        )
        .expect(constants_str::DIAGNOSTIC_04F83DC1);
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path.as_ref()),
            crate::expected_file_content::ExpectedFileContent::new(&expected),
        );
        crate::cleanup_test_file::cleanup_test_file(path);
    }
    #[test]
    fn test_try_maybe_write_token_stream_into_file_writes_tokens_when_enabled() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_TRY_WRITE,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_TRYDIDWRITE
            .parse()
            .expect(constants_str::DIAGNOSTIC_F771AC2D);
        let expected = ts.to_string();
        crate::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True,
            &base,
            crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&ts),
            &crate::format_with_cargofmt::FormatWithCargofmt::False,
        )
        .expect(constants_str::DIAGNOSTIC_6FEE9F6F);
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path.as_ref()),
            crate::expected_file_content::ExpectedFileContent::new(&expected),
        );
        crate::cleanup_test_file::cleanup_test_file(path);
    }
    #[test]
    fn test_try_maybe_write_token_stream_into_file_accepts_path_input() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_TRY_WRITE_PATH,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_PATHINPUT
            .parse()
            .expect(constants_str::DIAGNOSTIC_F9B0CD83);
        let expected = ts.to_string();
        crate::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True,
            &base,
            crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&ts),
            &crate::format_with_cargofmt::FormatWithCargofmt::False,
        )
        .expect(constants_str::DIAGNOSTIC_F341CDE7);
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path.as_ref()),
            crate::expected_file_content::ExpectedFileContent::new(&expected),
        );
        crate::cleanup_test_file::cleanup_test_file(path);
    }
    #[test]
    #[cfg_attr(miri, ignore = "Miri does not support spawning the rustfmt subprocess")]
    fn test_try_maybe_write_token_stream_into_file_formats_when_rustfmt_enabled() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_TRY_RUN_RUSTFMT,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::STRUCT_B).expect(constants_str::DIAGNOSTIC_7091840D);
        let ts: proc_macro2::TokenStream = constants_str::STRUCT_A
            .parse()
            .expect(constants_str::DIAGNOSTIC_0F30CA53);
        crate::try_maybe_write_token_stream_into_file::try_maybe_write_token_stream_into_file(
            crate::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True,
            &base,
            crate::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&ts),
            &crate::format_with_cargofmt::FormatWithCargofmt::True,
        )
        .expect(constants_str::DIAGNOSTIC_00A995A4);
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path.as_ref()),
            crate::expected_file_content::ExpectedFileContent::new(constants_str::STRUCT_A_NEWLINE),
        );
        crate::cleanup_test_file::cleanup_test_file(path);
    }
}
