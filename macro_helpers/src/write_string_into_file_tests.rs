#[cfg(test)]
#[cfg(test)]
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
    fn try_write_string_into_path_writes_exact_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_PATH);
        let result_path = crate::try_write_string_into_path_tests::try_write_string_into_path(
            &path,
            file_content(constants_str::ABC_ALT_3),
        )
        .expect("dcb22948 try_write_string_into_path_writes_exact_content invariant must hold");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn try_write_string_into_file_adds_rs_extension() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let _path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::XYZ),
        )
        .expect("4f3094e1 try_write_string_into_file_adds_rs_extension invariant must hold");
        assert_content_and_cleanup(path.as_ref(), constants_str::XYZ);
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_TRY_WRITE_FILE,
        ));
        let path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::QWE),
        )
        .expect("6676e082 try_write_string_into_file_returns_path invariant must hold");
        assert_content_and_cleanup(path.as_ref(), constants_str::QWE);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = txt_path(constants_str::MACRO_HELPERS_TRY_WRITE_PATH_PASSTHROUGH);
        let result_path = crate::try_write_string_into_path_tests::try_write_string_into_path(&path, file_content(constants_str::ABC_ALT_3))
            .expect("b6b47a2c try_write_string_into_path_writes_exact_path_without_extension_rewrite invariant must hold");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_missing_file() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_MISSING);
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(constants_str::ABC_ALT_3))
                .expect("f5d2cb68 should_write_string_into_file_returns_true_for_missing_file invariant must hold");
        assert!(bool::from(should_write));
    }
    #[test]
    fn should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_SAME);
        std::fs::write(&path, constants_str::SAME).expect("68e4f52d should_write_string_into_file_returns_false_when_content_is_eq invariant must hold");
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(constants_str::SAME))
                .expect("3e7adf2f should_write_string_into_file_returns_false_when_content_is_eq invariant must hold");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_compares_equal_content_in_chunks() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_LARGE_SAME);
        let content = constants_str::ABCD_ALT.repeat(4097usize);
        std::fs::write(&path, &content).expect("1d706d27 should_write_string_into_file_compares_equal_content_in_chunks invariant must hold");
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(&content)).expect("d6619712 should_write_string_into_file_compares_equal_content_in_chunks invariant must hold");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_finds_diff_after_first_chunk() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_LARGE_DIFF);
        let old_content = constants_str::A_ALT.repeat(16_388usize);
        let mut new_content = old_content.clone();
        new_content.replace_range(16_387usize.., constants_str::B);
        std::fs::write(&path, old_content).expect("abfd8fbc should_write_string_into_file_finds_diff_after_first_chunk invariant must hold");
        let should_write = crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(&new_content))
            .expect("a3040fa0 should_write_string_into_file_finds_diff_after_first_chunk invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_when_content_differs() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_DIFF);
        std::fs::write(&path, constants_str::OLD).expect("a2fd8473 should_write_string_into_file_returns_true_when_content_differs invariant must hold");
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(constants_str::NEW))
                .expect("52c9a1db should_write_string_into_file_returns_true_when_content_differs invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_same_len_diff_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("517fd0c9 should_write_string_into_file_returns_true_for_same_len_diff_content invariant must hold");
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(constants_str::XYZ))
                .expect("a82c48b8 should_write_string_into_file_returns_true_for_same_len_diff_content invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_diff_len_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_DIFF_LEN);
        std::fs::write(&path, constants_str::ABCD_ALT).expect("e2d99b73 should_write_string_into_file_returns_true_for_diff_len_content invariant must hold");
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(path_ref(&path), file_content(constants_str::A_ALT))
                .expect("157e8cad should_write_string_into_file_returns_true_for_diff_len_content invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn write_string_if_needed_returns_false_without_rewrite_for_eq_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_IF_NEEDED_EQ);
        std::fs::write(&path, constants_str::SAME).expect("924bdc58 write_string_if_needed_returns_false_without_rewrite_for_eq_content invariant must hold");
        let wrote = crate::write_string_if_needed_tests::write_string_if_needed(path_ref(&path), file_content(constants_str::SAME))
            .expect("9f27b9cb write_string_if_needed_returns_false_without_rewrite_for_eq_content invariant must hold");
        assert!(!bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::SAME);
    }
    #[test]
    fn write_string_if_needed_returns_true_and_writes_for_diff_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_IF_NEEDED_DIFF);
        std::fs::write(&path, constants_str::OLD).expect("9b4ab8ad write_string_if_needed_returns_true_and_writes_for_diff_content invariant must hold");
        let wrote = crate::write_string_if_needed_tests::write_string_if_needed(path_ref(&path), file_content(constants_str::NEW))
            .expect("4e4ce16d write_string_if_needed_returns_true_and_writes_for_diff_content invariant must hold");
        assert!(bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::NEW);
    }
    #[test]
    fn path_with_rs_extension_accepts_path_input() {
        let path = crate::rs_file_path_tests::rs_file_path(crate::test_path::test_path(
            crate::test_path_stem::TestPathStem::new(constants_str::MACRO_HELPERS_RS_EXT_PATH),
        ));
        assert_eq!(
            path.as_ref().extension().and_then(|v| v.to_str()),
            Some("rs")
        );
    }
    #[test]
    fn try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_IF_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::SAME).expect("0242e1a9 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let metadata_before = std::fs::metadata(&path).expect("974bc327 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let _path =
            crate::try_write_string_into_file::try_write_string_into_file(&base, file_content(constants_str::SAME)).expect("07d9fd90 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let metadata_after = std::fs::metadata(&path).expect("83087942 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_content_and_cleanup(path.as_ref(), constants_str::SAME);
    }
    #[test]
    fn try_write_string_into_file_writes_when_cnt_differs() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_IF_CHANGED_DIFF,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::OLD).expect(
            "d870b82e try_write_string_into_file_writes_when_cnt_differs invariant must hold",
        );
        let _path = crate::try_write_string_into_file::try_write_string_into_file(
            &base,
            file_content(constants_str::NEW),
        )
        .expect("c6fd2bc8 try_write_string_into_file_writes_when_cnt_differs invariant must hold");
        assert_content_and_cleanup(path.as_ref(), constants_str::NEW);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_CHANGED);
        let outcome =
            crate::try_write_string_into_path_with_outcome_tests::try_write_string_into_path_with_outcome(&path, file_content(constants_str::ABC_ALT_3))
                .expect("947faed1 try_write_string_into_path_with_outcome_returns_changed_for_new_content invariant must hold");
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(&path),
            crate::expected_file_content::ExpectedFileContent::new(constants_str::ABC_ALT_3),
        );
        assert_outcome_and_cleanup(path.as_path(), &outcome, true);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = txt_path(constants_str::MACRO_HELPERS_WRITE_OUTCOME_UNCHANGED);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("d293f783 try_write_string_into_path_with_outcome_returns_unchanged_for_same_content invariant must hold");
        let outcome =
            crate::try_write_string_into_path_with_outcome_tests::try_write_string_into_path_with_outcome(&path, file_content(constants_str::ABC_ALT_3))
                .expect("b8f8eaf1 try_write_string_into_path_with_outcome_returns_unchanged_for_same_content invariant must hold");
        assert_outcome_and_cleanup(path.as_path(), &outcome, false);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_changed_and_rs_path() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(constants_str::ABC_ALT_3))
                .expect("57cf209a try_write_string_into_file_with_outcome_returns_changed_and_rs_path invariant must hold");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(bool::from(outcome.is_changed()));
        assert_content_and_cleanup(path.as_ref(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_unchanged_for_same_content() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_UNCHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("2199f0a7 try_write_string_into_file_with_outcome_returns_unchanged_for_same_content invariant must hold");
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(constants_str::ABC_ALT_3))
                .expect("f60721a2 try_write_string_into_file_with_outcome_returns_unchanged_for_same_content invariant must hold");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(!bool::from(outcome.is_changed()));
        cleanup(path.as_ref());
    }
    #[test]
    fn write_path_outcome_into_path_returns_owned_path() {
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
