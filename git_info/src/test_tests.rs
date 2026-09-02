#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
struct TestGitCommit {
    commit: &'static str,
    fallback_calls: std::cell::Cell<usize>,
    borrow_commit_ref: bool,
}
impl crate::git_commit_id_provider::GitCommitIdProvider for TestGitCommit {
    fn git_commit_id(&self) -> crate::git_commit_id::GitCommitId {
        let calls = self.fallback_calls.get().saturating_add(1);
        self.fallback_calls.set(calls);
        crate::git_commit_id::GitCommitId::try_from(self.commit.to_owned())
            .expect(constants_str::DIAGNOSTIC_45A9C31D)
    }
    fn git_commit_id_ref(&self) -> Option<crate::git_commit_id_ref::GitCommitIdRef<'_>> {
        self.borrow_commit_ref
            .then_some(crate::git_commit_id_ref::GitCommitIdRef::from(self.commit))
    }
}
fn make_test_git_commit(commit: &'static str, borrow_commit_ref: bool) -> TestGitCommit {
    TestGitCommit {
        commit,
        borrow_commit_ref,
        fallback_calls: std::cell::Cell::new(0),
    }
}
fn make_owned_test_git_commit(commit: &'static str) -> TestGitCommit {
    make_test_git_commit(commit, false)
}
fn make_borrowed_test_git_commit(commit: &'static str) -> TestGitCommit {
    make_test_git_commit(commit, true)
}
fn assert_fallback_calls(v: &TestGitCommit, exp: usize) {
    assert_eq!(v.fallback_calls.get(), exp);
}
fn assert_expected_git_commit_link(actual: impl AsRef<str>, exp_commit_id: &str) {
    assert_eq!(actual.as_ref(), expected_git_commit_link(exp_commit_id));
}
fn assert_commit_link_and_fallback_calls(
    v: &TestGitCommit,
    exp_commit_id: &str,
    exp_fallback_calls: usize,
) {
    let link = crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(v);
    assert_expected_git_commit_link(&link, exp_commit_id);
    assert_fallback_calls(v, exp_fallback_calls);
}
fn assert_commit_id_cow_and_fallback_calls(
    v: &TestGitCommit,
    exp_commit_id: &str,
    exp_is_borrowed: bool,
    exp_fallback_calls: usize,
) {
    let commit_id = crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_cow(v);
    assert_eq!(commit_id.as_ref(), exp_commit_id);
    assert_eq!(
        matches!(
            std::borrow::Cow::from(commit_id),
            std::borrow::Cow::Borrowed(_)
        ),
        exp_is_borrowed,
    );
    assert_fallback_calls(v, exp_fallback_calls);
}
fn assert_commit_len_and_fallback_calls(
    v: &TestGitCommit,
    exp_commit_len: usize,
    exp_fallback_calls: usize,
) {
    let commit_len =
        crate::git_commit_id_provider::GitCommitIdProvider::with_git_commit_id(v, |commit_id| {
            commit_id.as_ref().len()
        });
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(v, exp_fallback_calls);
}
fn assert_with_git_commit_id_ref_or(
    v: &TestGitCommit,
    exp_commit_len: usize,
    exp_fallback_calls: usize,
) {
    let commit_len = crate::with_git_commit_id_ref_or::with_git_commit_id_ref_or(
        v,
        |commit_id| commit_id.as_ref().len(),
        |src| {
            crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id(src)
                .as_ref()
                .len()
        },
    );
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(v, exp_fallback_calls);
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
    let commit =
        crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id(oversized.as_str());
    assert!(commit.as_ref().len() <= crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN);
    let link = crate::build_git_commit_link_cow::build_git_commit_link_cow(oversized.as_str());
    assert!(link.as_ref().len() <= crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN);
}
#[test]
fn test_git_commit_link_builds_expected_url() {
    let link =
        crate::build_git_commit_link::build_git_commit_link(constants_str::TEST_VALUES_COMMIT);
    assert_expected_git_commit_link(&link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_git_commit_link_supports_empty_commit() {
    let link = crate::build_git_commit_link::build_git_commit_link(
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
    );
    assert_expected_git_commit_link(&link, constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
}
#[test]
fn test_git_commit_link_cow_borrows_static_project_link_for_project_commit() {
    let project_commit = crate::project_git_info_value::project_git_info_value().commit();
    let actual = crate::build_git_commit_link_cow::build_git_commit_link_cow(project_commit);
    assert!(
        matches!(std::borrow::Cow::from(actual), std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, <&str>::from(crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value())))
    );
}
#[test]
fn test_git_commit_link_uses_static_project_link_for_project_commit() {
    let project_commit = crate::project_git_info_value::project_git_info_value().commit();
    let actual = crate::build_git_commit_link::build_git_commit_link(project_commit);
    assert_eq!(
        actual,
        crate::project_git_commit_link_ref_value::project_git_commit_link_ref_value()
    );
}
#[test]
fn test_git_commit_link_cow_owns_link_for_non_project_commit() {
    let actual = crate::build_git_commit_link_cow::build_git_commit_link_cow(
        constants_str::TEST_VALUES_WRONG_COMMIT,
    );
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
        crate::project_git_commit_link::project_git_commit_link(),
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
    let link =
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&git_info);
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
    drop(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
            &test_git_commit,
        ),
    );
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn test_git_commit_id_or_else_computes_fallback_once() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
    let first = crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
        &test_git_commit,
        &mut fallback,
    );
    assert_eq!(first, constants_str::F00DBABE);
    let second = crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
        &test_git_commit,
        &mut fallback,
    );
    assert_eq!(second, constants_str::F00DBABE);
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn test_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    let mut fallback = crate::git_commit_id_fallback::GitCommitIdFallback::from(None);
    let commit = crate::git_commit_id_provider::GitCommitIdProvider::git_commit_id_or_else(
        &test_git_commit,
        &mut fallback,
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
    let link = crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
        &git_info,
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
    let str_link = crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(
        constants_str::TEST_VALUES_COMMIT,
    );
    assert_expected_git_commit_link(&str_link, constants_str::TEST_VALUES_COMMIT);
    let string = String::from(constants_str::TEST_VALUES_COMMIT);
    let string_link =
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&string);
    assert_expected_git_commit_link(&string_link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn test_git_commit_link_works_for_cow_str() {
    let borrowed = std::borrow::Cow::Borrowed(constants_str::TEST_VALUES_COMMIT);
    let borrowed_link =
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&borrowed);
    assert_expected_git_commit_link(&borrowed_link, constants_str::TEST_VALUES_COMMIT);
    let owned = std::borrow::Cow::<'_, str>::Owned(constants_str::TEST_VALUES_COMMIT.to_owned());
    assert_expected_git_commit_link(
        crate::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link(&owned),
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
