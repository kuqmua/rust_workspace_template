#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "test tests keeps declaration order aligned with generated layout or processing flow"
)]
struct TestGitCommit {
    commit: &'static str,
    fallback_calls: std::cell::Cell<usize>,
    borrow_commit_ref: bool,
}
impl crate::git_commit_id_provider::GitCommitIdProvider for TestGitCommit {
    fn git_commit_id(
        &self,
    ) -> Result<
        crate::git_commit_id::GitCommitId,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        let calls = self.fallback_calls.get().saturating_add(1);
        self.fallback_calls.set(calls);
        crate::git_commit_id::GitCommitId::try_from(self.commit.to_owned())
    }
    fn git_commit_id_ref(&self) -> Option<crate::git_commit_id_ref::GitCommitIdRef<'_>> {
        self.borrow_commit_ref
            .then_some(crate::git_commit_id_ref::GitCommitIdRef::from(self.commit))
    }
}
fn make_test_git_commit(str: &'static str, bool: bool) -> TestGitCommit {
    TestGitCommit {
        commit: str,
        borrow_commit_ref: bool,
        fallback_calls: std::cell::Cell::new(0),
    }
}
fn make_owned_test_git_commit(str: &'static str) -> TestGitCommit {
    make_test_git_commit(str, false)
}
fn make_borrowed_test_git_commit(str: &'static str) -> TestGitCommit {
    make_test_git_commit(str, true)
}
fn assert_fallback_calls(test_git_commit: &TestGitCommit, usize: usize) {
    assert_eq!(test_git_commit.fallback_calls.get(), usize);
}
fn assert_expected_git_commit_link(actual: impl AsRef<str>, str: &str) {
    assert_eq!(actual.as_ref(), expected_git_commit_link(str));
}
fn assert_commit_link_and_fallback_calls(test_git_commit: &TestGitCommit, str: &str, usize: usize) {
    let link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
            test_git_commit,
        ),
    );
    assert_expected_git_commit_link(&link, str);
    assert_fallback_calls(test_git_commit, usize);
}
fn assert_commit_id_cow_and_fallback_calls(
    test_git_commit: &TestGitCommit,
    str: &str,
    bool: bool,
    usize: usize,
) {
    let commit_id =
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_cow(test_git_commit);
    assert!(matches!(&commit_id, Ok(value) if value.as_ref() == str));
    assert_eq!(
        matches!(
            commit_id.map(std::borrow::Cow::from),
            Ok(std::borrow::Cow::Borrowed(_))
        ),
        bool,
    );
    assert_fallback_calls(test_git_commit, usize);
}
fn assert_commit_len_and_fallback_calls(
    test_git_commit: &TestGitCommit,
    exp_commit_len: usize,
    exp_fallback_calls: usize,
) {
    let commit_len = test_git_value(
        crate::git_commit_id_provider::GitCommitIdProvider::with_git_commit_id(
            test_git_commit,
            |commit_id| commit_id.as_ref().len(),
        ),
    );
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(test_git_commit, exp_fallback_calls);
}
fn assert_with_git_commit_id_ref_or(
    test_git_commit: &TestGitCommit,
    exp_commit_len: usize,
    exp_fallback_calls: usize,
) {
    let commit_len = crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
        test_git_commit,
        |commit_id| commit_id.as_ref().len(),
        |src| {
            test_git_value(crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id(src))
                .as_ref()
                .len()
        },
    );
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(test_git_commit, exp_fallback_calls);
}
fn expected_git_commit_link(commit_id_src: impl AsRef<str>) -> String {
    format!(
        "{}{}{}",
        constants_str::NAMING_GITHUB_URL,
        constants_str::GIT_INFO_TREE_SEGMENT,
        commit_id_src.as_ref()
    )
}
#[test]
fn test_owned_git_values_and_generated_links_enforce_length_limit() {
    let oversized = constants_str::X
        .repeat(crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE);
    let expected =
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: oversized.len(),
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        };
    assert_eq!(
        crate::git_commit_id::GitCommitId::try_from(oversized.clone()),
        Err(expected)
    );
    assert_eq!(
        crate::git_commit_link::GitCommitLink::try_from(oversized.clone()),
        Err(expected)
    );
    let Err(_commit_id_error) = crate::git_commit_id_cow::GitCommitIdCow::try_from(
        std::borrow::Cow::Owned(oversized.clone()),
    ) else {
        std::panic::panic_any(constants_str::PANIC_8C811508);
    };
    let Err(_commit_link_error) = crate::git_commit_link_cow::GitCommitLinkCow::try_from(
        std::borrow::Cow::Owned(oversized.clone()),
    ) else {
        std::panic::panic_any(constants_str::PANIC_69EE1326);
    };
    assert_eq!(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id(oversized.as_str()),
        Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: oversized.len(), max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        }),
    );
    assert_eq!(
        crate::build_git_commit_link_cow::build_git_commit_link_cow(oversized.as_str()),
        Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: oversized.len() + crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN,
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        }),
    );
}
#[test]
fn test_git_commit_link_builds_expected_url() {
    let link = test_git_value(crate::build_git_commit_link::build_git_commit_link(
        constants_str::TEST_VALUES_COMMIT,
    ));
    assert_expected_git_commit_link(&link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_git_commit_link_supports_empty_commit() {
    let link = test_git_value(crate::build_git_commit_link::build_git_commit_link(
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
    ));
    assert_expected_git_commit_link(&link, constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
}
#[test]
fn test_git_commit_link_cow_borrows_static_project_link_for_project_commit() {
    let project_commit = crate::project_git_info_value::project_git_info_value().commit();
    let actual = test_git_value(crate::build_git_commit_link_cow::build_git_commit_link_cow(
        project_commit,
    ));
    assert!(
        matches!(std::borrow::Cow::from(actual), std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, <&str>::from(crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value())))
    );
}
#[test]
fn test_git_commit_link_uses_static_project_link_for_project_commit() {
    let project_commit = crate::project_git_info_value::project_git_info_value().commit();
    let actual = test_git_value(crate::build_git_commit_link::build_git_commit_link(
        project_commit,
    ));
    assert_eq!(
        actual,
        crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value()
    );
}
#[test]
fn test_git_commit_link_cow_owns_link_for_non_project_commit() {
    let actual = test_git_value(crate::build_git_commit_link_cow::build_git_commit_link_cow(
        constants_str::TEST_VALUES_WRONG_COMMIT,
    ));
    assert!(
        matches!(std::borrow::Cow::from(actual), std::borrow::Cow::Owned(v) if v == expected_git_commit_link(constants_str::TEST_VALUES_WRONG_COMMIT))
    );
}
#[test]
fn test_is_project_commit_returns_true_for_project_commit() {
    assert!(crate::check_is_project_commit::check_is_project_commit(
        crate::project_git_info_value::project_git_info_value().commit()
    ));
}
#[test]
fn test_is_project_commit_returns_false_for_other_commit() {
    assert!(!crate::check_is_project_commit::check_is_project_commit(
        constants_str::TEST_VALUES_WRONG_COMMIT
    ));
}
#[test]
fn test_validate_project_commit_returns_ok_for_project_commit() {
    assert_eq!(
        crate::validate_project_commit::validate_project_commit(
            crate::project_git_info_value::project_git_info_value().commit()
        ),
        Ok(())
    );
}
#[test]
fn test_validate_project_commit_returns_project_link_for_non_project_commit() {
    assert_eq!(
        crate::validate_project_commit::validate_project_commit(
            constants_str::TEST_VALUES_WRONG_COMMIT
        ),
        Err(
            crate::validate_project_commit_error::ValidateProjectCommitError::from(
                crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value()
            )
        )
    );
}
#[test]
fn test_validate_project_commit_reuses_static_project_link_ref() {
    let error = crate::validate_project_commit::validate_project_commit(
        constants_str::TEST_VALUES_WRONG_COMMIT,
    )
    .expect_err(constants_str::VALUE_46BC13A9);
    let project_link =
        crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value();
    assert!(std::ptr::eq(
        <&str>::from(crate::project_git_commit_link_ref::ProjectGitCommitLinkRef::from(error)),
        <&str>::from(project_link)
    ));
}
#[test]
fn test_project_git_commit_link_matches_project_commit() {
    assert_eq!(
        crate::project_git_commit_link::project_git_commit_link().as_ref(),
        expected_git_commit_link(crate::project_git_info_value::project_git_info_value().commit())
    );
}
#[test]
fn test_project_git_commit_link_ref_is_static_and_stable() {
    let first = crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value();
    let second = crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value();
    assert_eq!(first, second);
    assert!(std::ptr::eq(<&str>::from(first), <&str>::from(second)));
}
#[test]
fn test_project_git_info_returns_commit_link() {
    let git_info = crate::project_git_info::ProjectGitInfo::from(
        crate::git_commit_id_ref::GitCommitIdRef::from(constants_str::TEST_VALUES_WRONG_COMMIT),
    );
    let link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&git_info),
    );
    assert_expected_git_commit_link(&link, constants_str::TEST_VALUES_WRONG_COMMIT);
}
#[test]
fn test_git_commit_link_uses_trait_based_commit_id() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    assert_commit_link_and_fallback_calls(&test_git_commit, constants_str::F00DBABE, 1);
}
#[test]
fn test_git_commit_link_calls_allocating_fallback_once_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    drop(test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
            &test_git_commit,
        ),
    ));
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn test_git_commit_id_or_else_computes_fallback_once() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
    let first = test_git_value(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
            &test_git_commit,
            &mut fallback,
        ),
    );
    assert_eq!(first, constants_str::F00DBABE);
    let second = test_git_value(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
            &test_git_commit,
            &mut fallback,
        ),
    );
    assert_eq!(second, constants_str::F00DBABE);
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn test_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
    let commit = test_git_value(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
            &test_git_commit,
            &mut fallback,
        ),
    );
    assert_eq!(commit, constants_str::CAFEBABE);
    assert_fallback_calls(&test_git_commit, 0);
    assert!(fallback.is_none());
}
#[test]
fn test_git_commit_id_cow_returns_owned_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_commit_id_cow_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, false, 1);
}
#[test]
fn test_git_commit_link_prefers_borrowed_commit_id() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_link_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, 0);
}
#[test]
fn test_git_commit_link_cow_borrows_project_link_for_project_commit() {
    let git_info = crate::project_git_info::ProjectGitInfo::from(
        crate::project_git_info_value::project_git_info_value().commit(),
    );
    let link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            &git_info,
        ),
    );
    assert!(
        matches!(std::borrow::Cow::from(link), std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, <&str>::from(crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value())))
    );
}
#[test]
fn test_git_commit_id_cow_returns_borrowed_when_ref_is_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_id_cow_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, true, 0);
}
#[test]
fn test_with_git_commit_id_uses_allocating_fallback_once_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_commit_len_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE.len(), 1);
}
#[test]
fn test_with_git_commit_id_prefers_borrowed_ref_when_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_len_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE.len(), 0);
}
#[test]
fn test_with_git_commit_id_ref_or_prefers_borrowed_ref_when_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_with_git_commit_id_ref_or(&test_git_commit, constants_str::CAFEBABE.len(), 0);
}
#[test]
fn test_with_git_commit_id_ref_or_uses_fallback_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_with_git_commit_id_ref_or(&test_git_commit, constants_str::CAFEBABE.len(), 1);
}
#[test]
fn test_base_git_commit_link_len_matches_expected_prefix_len() {
    let commit_id = constants_str::TEST_VALUES_COMMIT;
    let expected = format!("{}/tree/{commit_id}", constants_str::NAMING_GITHUB_URL).len();
    assert_eq!(
        crate::git_commit_link_capacity_value::git_commit_link_capacity_value(commit_id),
        expected
    );
}
#[test]
fn test_git_commit_link_works_for_str_and_string() {
    let str_link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
            constants_str::TEST_VALUES_COMMIT,
        ),
    );
    assert_expected_git_commit_link(&str_link, constants_str::TEST_VALUES_COMMIT);
    let string = String::from(constants_str::TEST_VALUES_COMMIT);
    let string_link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&string),
    );
    assert_expected_git_commit_link(&string_link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_git_commit_link_works_for_cow_str() {
    let borrowed = std::borrow::Cow::Borrowed(constants_str::TEST_VALUES_COMMIT);
    let borrowed_link = test_git_value(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&borrowed),
    );
    assert_expected_git_commit_link(&borrowed_link, constants_str::TEST_VALUES_COMMIT);
    let owned = std::borrow::Cow::<'_, str>::Owned(constants_str::TEST_VALUES_COMMIT.to_owned());
    assert_expected_git_commit_link(
        test_git_value(
            crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&owned),
        ),
        constants_str::TEST_VALUES_COMMIT,
    );
}
#[test]
fn test_project_git_info_as_ref_returns_commit() {
    let info = crate::project_git_info::ProjectGitInfo::from(
        crate::git_commit_id_ref::GitCommitIdRef::from(constants_str::TEST_VALUES_COMMIT),
    );
    assert_eq!(info.as_ref(), constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_git_commit_link_capacity_supports_empty_commit() {
    assert_eq!(
        crate::git_commit_link_capacity_value::git_commit_link_capacity_value(constants_str::EMPTY),
        constants_str::NAMING_GITHUB_URL.len() + constants_str::GIT_INFO_TREE_SEGMENT.len()
    );
}

#[test]
fn test_borrowed_commit_id_conversion_preserves_text_and_rejects_overflow() {
    assert!([
        constants_usize::ZERO,
        constants_usize::ONE,
        crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE,
    ].into_iter().all(|length| {
        let text = constants_str::X.repeat(length);
        let result = crate::git_commit_id::GitCommitId::try_from(
            crate::git_commit_id_ref::GitCommitIdRef::from(text.as_str()),
        );
        match result {
            Ok(commit) => length <= crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN
                && commit.as_ref() == text.as_str(),
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong { len, max }) =>
                length > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN
                && len == length && max == crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        }
    }));
}

#[test]
fn test_cow_commit_provider_rejects_overflow_and_preserves_borrowing() {
    assert!([
        constants_usize::ZERO,
        crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE,
    ].into_iter().all(|length| {
        let text = constants_str::X.repeat(length);
        match crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_cow(&text) {
            Ok(commit) => matches!(std::borrow::Cow::from(commit), std::borrow::Cow::Borrowed(value)
                if value == text && std::ptr::eq(value.as_ptr(), text.as_ptr())
                    && length <= crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN),
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong { len, max }) =>
                length > crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN
                    && len == length && max == crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        }
    }));
}

fn test_git_value<Value>(
    result: Result<
        Value,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    >,
) -> Value {
    result.expect(constants_str::DIAGNOSTIC_45A9C31D)
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
enum TestRejectedGitCommit {
    Invalid,
}
impl crate::git_commit_id_provider::GitCommitIdProvider for TestRejectedGitCommit {
    fn git_commit_id(
        &self,
    ) -> Result<
        crate::git_commit_id::GitCommitId,
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError,
    > {
        Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE,
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        })
    }
}

#[test]
fn test_failed_commit_fallback_preserves_error_and_does_not_invoke_callback() {
    let provider = TestRejectedGitCommit::Invalid;
    let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
    let expected =
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE,
            max: crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        };
    assert_eq!(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
            &provider,
            &mut fallback
        ),
        Err(expected)
    );
    assert!(fallback.is_none());
    let invoked = std::cell::Cell::new(false);
    assert_eq!(
        crate::git_commit_id_provider::GitCommitIdProvider::with_git_commit_id(
            &provider,
            |_commit| invoked.set(true)
        ),
        Err(expected)
    );
    assert!(!invoked.get());
    assert_eq!(
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_cow(&provider),
        Err(expected)
    );
    assert_eq!(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&provider),
        Err(expected)
    );
    assert_eq!(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
            &provider
        ),
        Err(expected)
    );
}

#[test]
fn test_commit_link_length_boundary_returns_original_length_error() {
    let maximum_commit = crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN
        - crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN;
    assert!([maximum_commit, maximum_commit + constants_usize::ONE].into_iter().all(|length| {
        let text = constants_str::X.repeat(length);
        let result = crate::build_git_commit_link_cow::build_git_commit_link_cow(text.as_str());
        match result {
            Ok(link) => length == maximum_commit && link.as_ref() == expected_git_commit_link(text.as_str()),
            Err(crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {len, max}) => length > maximum_commit && len == length + crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN && max == crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN,
        }
    }));
}

#[test]
fn test_git_length_error_fallbacks_preserve_diagnostic_text_and_owned_storage() {
    let error =
        crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
            len: 7usize,
            max: 3usize,
        };
    assert_eq!(
        error.to_string(),
        constants_str::TEST_GIT_STRING_LENGTH_ERROR
    );
    let commit = crate::git_commit_id::GitCommitId::from(error);
    let link = crate::git_commit_link::GitCommitLink::from(error);
    assert_eq!(commit.as_ref(), constants_str::TEST_GIT_STRING_LENGTH_ERROR);
    assert_eq!(link.as_ref(), constants_str::TEST_GIT_STRING_LENGTH_ERROR);
    [
        std::borrow::Cow::from(crate::git_commit_id_cow::GitCommitIdCow::from(error)),
        std::borrow::Cow::from(crate::git_commit_link_cow::GitCommitLinkCow::from(error)),
    ]
    .into_iter()
    .fold((), |(), fallback| {
        assert!(matches!(fallback, std::borrow::Cow::Owned(text)
            if text == constants_str::TEST_GIT_STRING_LENGTH_ERROR));
    });
}

#[test]
fn test_owned_commit_link_and_deserialization_enforce_byte_limit() {
    let maximum = crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN;
    [
        constants_str::EMPTY.to_owned(),
        constants_str::X.repeat(maximum),
        constants_str::X.repeat(maximum + constants_usize::ONE),
        '\u{00e9}'.to_string().repeat(maximum),
    ]
    .into_iter()
    .fold((), |(), text| {
        let parsed = crate::git_commit_link_cow::GitCommitLinkCow::try_from(text.clone());
        let deserializer = serde::de::value::StringDeserializer::<serde::de::value::Error>::new(text.clone());
        let decoded = <crate::git_commit_link_cow::GitCommitLinkCow as serde::Deserialize>::deserialize(deserializer);
        if text.len() <= maximum {
            assert!(parsed.is_ok_and(|link| link.as_ref() == text
                && matches!(std::borrow::Cow::from(link), std::borrow::Cow::Owned(value) if value == text)));
            assert!(decoded.is_ok_and(|link| link.as_ref() == text
                && matches!(std::borrow::Cow::from(link), std::borrow::Cow::Owned(value) if value == text)));
        } else {
            let expected = crate::git_info_string_try_from_string_error::GitInfoStringTryFromStringError::TooLong {
                len: text.len(),
                max: maximum,
            };
            assert_eq!(parsed, Err(expected));
            assert!(decoded.is_err_and(|error| error.to_string().contains(expected.to_string().as_str())));
        }
    });
}

#[test]
fn test_git_link_capacity_counts_unicode_bytes_and_preserves_link_text() {
    let text = char::from(233u8).to_string().repeat(3usize);
    assert_eq!(text.chars().count(), 3usize);
    assert_eq!(text.len(), 6usize);
    let expected = expected_git_commit_link(text.as_str());
    assert_eq!(
        crate::git_commit_link_capacity_value::git_commit_link_capacity_value(text.as_str()),
        expected.len(),
    );
    let actual = test_git_value(crate::build_git_commit_link::build_git_commit_link(
        text.as_str(),
    ));
    assert_eq!(actual.as_ref(), expected.as_str());
}

#[test]
fn test_default_git_references_preserve_empty_commit_text() {
    let commit = crate::git_commit_id_ref::GitCommitIdRef::default();
    let info = crate::project_git_info::ProjectGitInfo::default();
    assert_eq!(commit.as_ref(), constants_str::EMPTY);
    assert_eq!(info.commit(), commit);
    assert_eq!(info.as_ref(), constants_str::EMPTY);
    assert_eq!(commit.to_string(), constants_str::EMPTY);
}

#[test]
fn test_project_commit_comparison_preserves_exact_case_and_whitespace() {
    let info = crate::project_git_info_value::project_git_info_value();
    let commit = info.commit();
    [
        format!("{}{commit}", constants_str::SPACE),
        format!("{commit}{}", constants_str::SPACE),
        commit.as_ref().to_ascii_uppercase(),
        commit.as_ref().to_ascii_lowercase(),
    ]
    .into_iter()
    .fold((), |(), text| {
        let expected = text.as_str() == commit.as_ref();
        assert_eq!(
            bool::from(crate::check_is_project_commit::check_is_project_commit(
                text.as_str()
            )),
            expected,
        );
        assert_eq!(
            crate::validate_project_commit::validate_project_commit(text.as_str()),
            if expected {
                Ok(())
            } else {
                Err(
                    crate::validate_project_commit_error::ValidateProjectCommitError::from(
                        crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value(
                        ),
                    ),
                )
            },
        );
    });
}
