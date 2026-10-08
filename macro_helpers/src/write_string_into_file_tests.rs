#[cfg(test)]
mod tests {
    fn file_content(str: &str) -> crate::string_file_content_ref::StringFileContentRef<'_> {
        crate::string_file_content_ref::StringFileContentRef::from(str)
    }
    fn path_ref(path: &std::path::Path) -> crate::written_file_path_ref::WrittenFilePathRef<'_> {
        crate::written_file_path_ref::WrittenFilePathRef::from(path)
    }
    fn written_path(
        path_buf: std::path::PathBuf,
    ) -> crate::written_file_path_buf::WrittenFilePathBuf {
        crate::written_file_path_buf::WrittenFilePathBuf::from(path_buf)
    }
    fn txt_path(str: &str) -> std::path::PathBuf {
        crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(str))
            .as_ref()
            .with_extension(constants_str::TXT)
    }
    fn cleanup(path: &std::path::Path) {
        crate::cleanup_test_file::cleanup_test_file(path);
    }
    fn assert_content_and_cleanup(path: &std::path::Path, str: &str) {
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path),
            crate::expected_file_content::ExpectedFileContent::new(str),
        );
        cleanup(path);
    }
    fn assert_outcome_and_cleanup(
        path: &std::path::Path,
        write_path_outcome: &crate::write_path_outcome::WritePathOutcome,
        bool: bool,
    ) {
        assert_eq!(write_path_outcome.path().as_ref(), path);
        assert_eq!(bool::from(write_path_outcome.is_changed()), bool);
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
    fn test_should_write_string_into_file_accepts_invalid_utf8_of_same_length()
    -> std::io::Result<()> {
        let path = txt_path(constants_str::MACRO_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF);
        std::fs::write(&path, [u8::MAX; 3])?;
        let should_write =
            crate::should_write_string_into_file_tests::should_write_string_into_file(
                path_ref(&path),
                file_content(constants_str::XYZ),
            )?;
        cleanup(path.as_path());
        if bool::from(should_write) {
            Ok(())
        } else {
            Err(std::io::ErrorKind::InvalidData.into())
        }
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
            Some(constants_str::RS)
        );
    }
    #[test]
    fn test_try_write_string_into_file_skips_rewrite_when_content_is_unchanged() {
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
    fn test_try_write_string_into_file_writes_when_content_differs() {
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
    fn test_try_write_string_into_file_replaces_invalid_utf8_of_same_length() -> std::io::Result<()>
    {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        std::fs::write(&path, [u8::MAX; 3])?;
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
                &base,
                file_content(constants_str::ABC_ALT_3),
            )
            ?;
        crate::assert_file_content::assert_file_content(
            crate::std_assert_file_path::StdAssertFilePath::new(path.as_ref()),
            crate::expected_file_content::ExpectedFileContent::new(constants_str::ABC_ALT_3),
        );
        assert_outcome_and_cleanup(path.as_ref(), &outcome, true);
        Ok(())
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
    #[test]
    fn test_atomic_file_writer_compares_complete_content_across_chunk_boundaries() {
        [0usize, 8192usize, 8193usize, 16385usize].into_iter().fold((), |(), length| {
            let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED));
            let path = crate::rs_file_path_tests::rs_file_path(&base);
            let initial = constants_str::X.repeat(length);
            assert!(matches!(std::fs::write(&path, initial.as_bytes()), Ok(())));
            assert!(crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(initial.as_str())).is_ok_and(|outcome| !bool::from(outcome.is_changed()) && outcome.path().as_ref() == path.as_ref()));
            let changed = format!("{}{}", constants_str::X.repeat(length.saturating_sub(1usize)), constants_str::B);
            assert_eq!(changed.len(), length.max(1usize));
            assert!(crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(&base, file_content(changed.as_str())).is_ok_and(|outcome| bool::from(outcome.is_changed()) && outcome.path().as_ref() == path.as_ref()));
            assert_content_and_cleanup(path.as_ref(), changed.as_str());
        });
    }

    #[test]
    fn test_atomic_file_writer_preserves_parent_file_on_metadata_failure() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        assert!(matches!(
            std::fs::write(&base, constants_str::ABC_ALT_3),
            Ok(())
        ));
        let destination = base.as_ref().join(constants_str::X);
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
                &destination,
                file_content(constants_str::ABC_ALT_3),
            );
        assert!(outcome.is_err_and(|error| error.kind() == std::io::ErrorKind::NotADirectory));
        assert_content_and_cleanup(base.as_ref(), constants_str::ABC_ALT_3);
    }

    #[test]
    fn test_cleanup_reports_directory_removal_failure_and_preserves_directory() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        assert!(matches!(std::fs::create_dir_all(&base), Ok(())));
        let expected_error = std::fs::remove_file(&base).err();
        assert!(expected_error.is_some());
        let Some(error) = expected_error else {
            return;
        };
        let expected = constants_str::PANIC_33EA4EA2.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            &error.to_string(),
            1usize,
        );
        let observed = std::panic::catch_unwind(|| cleanup(base.as_ref()));
        assert!(std::fs::metadata(&base).is_ok_and(|metadata| metadata.is_dir()));
        assert!(matches!(std::fs::remove_dir(&base), Ok(())));
        assert!(observed.as_ref().is_err_and(|payload| {
            payload
                .downcast_ref::<String>()
                .is_some_and(|message| message == &expected)
        }));
        assert!(matches!(
            std::panic::catch_unwind(|| cleanup(base.as_ref())),
            Ok(())
        ));
        assert!(std::fs::metadata(&base).is_err_and(
            |missing_path_error| missing_path_error.kind() == std::io::ErrorKind::NotFound
        ));
    }
    #[test]
    fn test_atomic_file_writer_propagates_missing_parent_error_without_creating_directory() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let destination = base.as_ref().join(constants_str::X);
        let outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
                &destination,
                file_content(constants_str::ABC_ALT_3),
            );
        assert!(outcome.is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound));
        assert!(
            std::fs::metadata(&base)
                .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        );
    }

    #[test]
    fn test_atomic_file_writer_preserves_directory_on_read_and_commit_errors() {
        let base = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(
            constants_str::MACRO_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path_tests::rs_file_path(&base);
        assert!(matches!(std::fs::create_dir_all(&path), Ok(())));
        let marker = path.as_ref().join(constants_str::X);
        assert!(matches!(
            std::fs::write(&marker, constants_str::ABC_ALT_3),
            Ok(())
        ));
        let metadata_result = std::fs::metadata(&path);
        assert!(
            metadata_result
                .as_ref()
                .is_ok_and(std::fs::Metadata::is_dir)
        );
        let Ok(metadata) = metadata_result else {
            return;
        };
        let length_result = usize::try_from(metadata.len());
        assert!(
            length_result
                .as_ref()
                .is_ok_and(|length| *length <= 16384usize)
        );
        let Ok(length) = length_result else {
            return;
        };
        let content = constants_str::X.repeat(length);
        let read_outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
                &base,
                file_content(content.as_str()),
            );
        assert!(read_outcome.is_err_and(|error| error.kind() == std::io::ErrorKind::IsADirectory));
        let commit_outcome =
            crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
                &base,
                file_content(constants_str::EMPTY),
            );
        assert!(
            commit_outcome.is_err_and(|error| error.kind() == std::io::ErrorKind::IsADirectory)
        );
        assert_content_and_cleanup(marker.as_path(), constants_str::ABC_ALT_3);
        assert!(matches!(std::fs::remove_dir(&path), Ok(())));
    }
    #[test]
    fn test_file_content_assertion_rejects_read_failures_and_content_mismatch() {
        let path = txt_path(stringify!(
            test_file_content_assertion_rejects_read_failures_and_content_mismatch
        ));
        let invalid_utf8 = [u8::MAX];
        [
            (None, constants_str::X, true),
            (Some(constants_str::XYZ.as_bytes()), constants_str::X, true),
            (Some(invalid_utf8.as_slice()), constants_str::X, true),
            (Some(constants_str::X.as_bytes()), constants_str::A, false),
        ]
        .into_iter()
        .fold((), |(), (content, expected, read_failure)| {
            cleanup(path.as_path());
            if let Some(bytes) = content {
                assert!(std::fs::write(&path, bytes).is_ok_and(|()| true));
            }
            let observed = std::panic::catch_unwind(|| {
                crate::assert_file_content::assert_file_content(
                    crate::std_assert_file_path::StdAssertFilePath::new(path.as_path()),
                    crate::expected_file_content::ExpectedFileContent::new(expected),
                );
            });
            assert!(observed.is_err_and(|payload| {
                !read_failure
                    || payload.downcast_ref::<String>().is_some_and(|message| {
                        message.starts_with(constants_str::DIAGNOSTIC_D5EC6712)
                    })
            }));
        });
        assert!(std::fs::write(&path, constants_str::ABC_ALT_3).is_ok_and(|()| true));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
}
