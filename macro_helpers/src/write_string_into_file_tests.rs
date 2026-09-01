#[cfg(test)]
mod tests {
    fn file_content(v: &str) -> crate::string_file_content_ref::StringFileContentRef<'_> {
        crate::string_file_content_ref::StringFileContentRef::from(v)
    }
    fn path_ref(v: &std::path::Path) -> crate::written_file_path_ref::WrittenFilePathRef<'_> {
        crate::written_file_path_ref::WrittenFilePathRef::from(v)
    }
    fn written_path(v: std::path::PathBuf) -> crate::written_file_path_buf::WrittenFilePathBuf {
        crate::written_file_path_buf::WrittenFilePathBuf::from(v)
    }
    fn txt_path(name: &str) -> std::path::PathBuf {
        crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(name))
            .as_ref()
            .with_extension(constants_str::TXT)
    }
    fn cleanup(path: &std::path::Path) {
        crate::cleanup_test_file::cleanup_test_file(path);
    }
    fn assert_content_and_cleanup(path: &std::path::Path, expected: &str) {
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path),
            crate::expected_file_content::ExpectedFileContent::new(expected),
        );
        cleanup(path);
    }
    fn assert_outcome_and_cleanup(
        path: &std::path::Path,
        outcome: &crate::write_path_outcome::WritePathOutcome,
        expected_changed: bool,
    ) {
        assert_eq!(outcome.path().as_ref(), path);
        assert_eq!(bool::from(outcome.is_changed()), expected_changed);
        cleanup(path);
    }
    #[test]
    fn test_try_write_string_into_path_writes_exact_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_PATH);
        let result_path = crate::try_write_string_into_path_tests::try_write_string_into_path(
            &path,
            file_content(constants_str::ABC_ALT_3),
        )
        .expect(constants_str::DIAGNOSTIC_DCB22948);
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn test_try_write_string_into_file_adds_rs_extension() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let _path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::XYZ),
        )
        .expect(constants_str::DIAGNOSTIC_4F3094E1);
        assert_content_and_cleanup(path.as_ref(), constants_str::XYZ);
    }
    #[test]
    fn test_try_write_string_into_file_returns_path() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_TRY_WRITE_FILE,
        ));
        let path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::QWE),
        )
        .expect(constants_str::DIAGNOSTIC_6676E082);
        assert_content_and_cleanup(path.as_ref(), constants_str::QWE);
    }
    #[test]
    fn test_try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = txt_path(constants_str::MACRO_HELPERS_TRY_WRITE_PATH_PASSTHROUGH);
        let result_path = crate::try_write_string_into_path_tests::try_write_string_into_path(
            &path,
            file_content(constants_str::ABC_ALT_3),
        )
        .expect(constants_str::DIAGNOSTIC_B6B47A2C);
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn test_should_write_string_into_file_returns_true_for_missing_file() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_MISSING);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::ABC_ALT_3),
            )
            .expect(constants_str::DIAGNOSTIC_F5D2CB68);
        assert!(bool::from(should_write));
    }
    #[test]
    fn test_should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_SAME);
        std::fs::write(&path, constants_str::SAME).expect(constants_str::DIAGNOSTIC_68E4F52D);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::SAME),
            )
            .expect(constants_str::DIAGNOSTIC_3E7ADF2F);
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_should_write_string_into_file_compares_equal_content_in_chunks() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_LARGE_SAME);
        let content = constants_str::ABCD_ALT.repeat(4097usize);
        std::fs::write(&path, &content).expect(constants_str::DIAGNOSTIC_1D706D27);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(&content),
            )
            .expect(constants_str::DIAGNOSTIC_D6619712);
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_should_write_string_into_file_finds_diff_after_first_chunk() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_LARGE_DIFF);
        let old_content = constants_str::A_ALT.repeat(16_388usize);
        let mut new_content = old_content.clone();
        new_content.replace_range(16_387usize.., constants_str::B);
        std::fs::write(&path, old_content).expect(constants_str::DIAGNOSTIC_ABFD8FBC);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(&new_content),
            )
            .expect(constants_str::DIAGNOSTIC_A3040FA0);
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_should_write_string_into_file_returns_true_when_content_differs() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_DIFF);
        std::fs::write(&path, constants_str::OLD).expect(constants_str::DIAGNOSTIC_A2FD8473);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::NEW),
            )
            .expect(constants_str::DIAGNOSTIC_52C9A1DB);
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_should_write_string_into_file_returns_true_for_same_len_diff_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect(constants_str::DIAGNOSTIC_517FD0C9);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::XYZ),
            )
            .expect(constants_str::DIAGNOSTIC_A82C48B8);
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_should_write_string_into_file_returns_true_for_diff_len_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_DIFF_LEN);
        std::fs::write(&path, constants_str::ABCD_ALT).expect(constants_str::DIAGNOSTIC_E2D99B73);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::A_ALT),
            )
            .expect(constants_str::DIAGNOSTIC_157E8CAD);
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn test_write_string_if_needed_returns_false_without_rewrite_for_eq_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_IF_NEEDED_EQ);
        std::fs::write(&path, constants_str::SAME).expect(constants_str::DIAGNOSTIC_924BDC58);
        let wrote = crate::write_string_if_needed_tests::write_string_if_needed(
            path_ref(&path),
            file_content(constants_str::SAME),
        )
        .expect(constants_str::DIAGNOSTIC_9F27B9CB);
        assert!(!bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::SAME);
    }
    #[test]
    fn test_write_string_if_needed_returns_true_and_writes_for_diff_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_IF_NEEDED_DIFF);
        std::fs::write(&path, constants_str::OLD).expect(constants_str::DIAGNOSTIC_9B4AB8AD);
        let wrote = crate::write_string_if_needed_tests::write_string_if_needed(
            path_ref(&path),
            file_content(constants_str::NEW),
        )
        .expect(constants_str::DIAGNOSTIC_4E4CE16D);
        assert!(bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::NEW);
    }
    #[test]
    fn test_path_with_rs_extension_accepts_path_input() {
        let path = crate::rs_file_path_tests::rs_file_path(crate::test_path::test_path(
            crate::test_path_stem::TestPathStem::new(constants_str::MACRO_HELPERS_RS_EXT_PATH),
        ));
        assert_eq!(
            path.as_ref().extension().and_then(|v| v.to_str()),
            Some("rs")
        );
    }
    #[test]
    fn test_try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_IF_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::SAME).expect(constants_str::DIAGNOSTIC_0242E1A9);
        let metadata_before = std::fs::metadata(&path).expect(constants_str::DIAGNOSTIC_974BC327);
        let _path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::SAME),
        )
        .expect(constants_str::DIAGNOSTIC_07D9FD90);
        let metadata_after = std::fs::metadata(&path).expect(constants_str::DIAGNOSTIC_83087942);
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_content_and_cleanup(path.as_ref(), constants_str::SAME);
    }
    #[test]
    fn test_try_write_string_into_file_writes_when_cnt_differs() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_IF_CHANGED_DIFF,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::OLD).expect(constants_str::DIAGNOSTIC_D870B82E);
        let _path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::NEW),
        )
        .expect(constants_str::DIAGNOSTIC_C6FD2BC8);
        assert_content_and_cleanup(path.as_ref(), constants_str::NEW);
    }
    #[test]
    fn test_try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_CHANGED);
        let outcome =
            crate::try_write_string_into_path_with_outcome_tests::try_write_string_into_path_with_outcome(&path, file_content(constants_str::ABC_ALT_3))
                .expect(constants_str::DIAGNOSTIC_947FAED1);
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(&path),
            crate::expected_file_content::ExpectedFileContent::new(constants_str::ABC_ALT_3),
        );
        assert_outcome_and_cleanup(path.as_path(), &outcome, true);
    }
    #[test]
    fn test_try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_UNCHANGED);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect(constants_str::DIAGNOSTIC_D293F783);
        let outcome =
            crate::try_write_string_into_path_with_outcome_tests::try_write_string_into_path_with_outcome(&path, file_content(constants_str::ABC_ALT_3))
                .expect(constants_str::DIAGNOSTIC_B8F8EAF1);
        assert_outcome_and_cleanup(path.as_path(), &outcome, false);
    }
    #[test]
    fn test_try_write_string_into_file_with_outcome_returns_changed_and_rs_path() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(constants_str::ABC_ALT_3))
                .expect(constants_str::DIAGNOSTIC_57CF209A);
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(bool::from(outcome.is_changed()));
        assert_content_and_cleanup(path.as_ref(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn test_try_write_string_into_file_with_outcome_returns_unchanged_for_same_content() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_UNCHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect(constants_str::DIAGNOSTIC_2199F0A7);
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(constants_str::ABC_ALT_3))
                .expect(constants_str::DIAGNOSTIC_F60721A2);
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(!bool::from(outcome.is_changed()));
        cleanup(path.as_ref());
    }
    #[test]
    fn test_write_path_outcome_into_path_returns_owned_path() {
        let changed_path = txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_INTO_PATH_CHANGED);
        let changed = crate::write_path_outcome::WritePathOutcome::Changed(written_path(
            changed_path.clone(),
        ));
        assert_eq!(changed.into_path(), written_path(changed_path));
        let unchanged_path =
            txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_INTO_PATH_UNCHANGED);
        let unchanged = crate::write_path_outcome::WritePathOutcome::Unchanged(written_path(
            unchanged_path.clone(),
        ));
        assert_eq!(unchanged.into_path(), written_path(unchanged_path));
    }
}
