#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
struct TestGitCommit {
    commit: &'static str,
    fallback_calls: std::cell::Cell<usize>,
    borrow_commit_ref: bool,
}
impl super::GitCommitIdProvider for TestGitCommit {
    fn git_commit_id(&self) -> super::GitCommitId {
        let calls = self.fallback_calls.get().saturating_add(1);
        self.fallback_calls.set(calls);
        super::GitCommitId::try_from(self.commit.to_owned())
            .expect("45a9c31d git_commit_id invariant must hold")
    }
    fn git_commit_id_ref(&self) -> Option<super::GitCommitIdRef<'_>> {
        self.borrow_commit_ref
            .then_some(super::GitCommitIdRef::from(self.commit))
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
    let link = super::GitCommitLinkProvider::git_commit_link(v);
    assert_expected_git_commit_link(&link, exp_commit_id);
    assert_fallback_calls(v, exp_fallback_calls);
}
fn assert_commit_id_cow_and_fallback_calls(
    v: &TestGitCommit,
    exp_commit_id: &str,
    exp_is_borrowed: bool,
    exp_fallback_calls: usize,
) {
    let commit_id = super::GitCommitIdProvider::git_commit_id_cow(v);
    assert_eq!(commit_id.as_ref(), exp_commit_id);
    assert_eq!(
        matches!(&commit_id.0, std::borrow::Cow::Borrowed(_)),
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
        super::GitCommitIdProvider::with_git_commit_id(v, |commit_id| commit_id.0.len());
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(v, exp_fallback_calls);
}
fn assert_with_git_commit_id_ref_or(
    v: &TestGitCommit,
    exp_commit_len: usize,
    exp_fallback_calls: usize,
) {
    let commit_len = super::with_git_commit_id_ref_or(
        v,
        |commit_id| commit_id.0.len(),
        |src| super::GitCommitIdProvider::git_commit_id(src).0.len(),
    );
    assert_eq!(commit_len, exp_commit_len);
    assert_fallback_calls(v, exp_fallback_calls);
}
fn expected_git_commit_link(commit_id_src: impl AsRef<str>) -> String {
    let commit_id = super::GitCommitIdRef::from(commit_id_src.as_ref());
    let mut output = String::with_capacity(super::git_commit_link_capacity(commit_id).0);
    let mut output_ref = super::GitCommitLinkOutputRefMut::from(&mut output);
    super::write_git_commit_link(&mut output_ref, commit_id);
    output
}
#[test]
fn owned_git_values_and_generated_links_enforce_length_limit() {
    let oversized = constants_str::X.repeat(super::GIT_INFO_STRING_MAX_LEN + constants_usize::ONE);
    let Err(_commit_id_error) =
        super::GitCommitIdCow::try_from(std::borrow::Cow::Owned(oversized.clone()))
    else {
        panic!("8c811508");
    };
    let Err(_commit_link_error) =
        super::GitCommitLinkCow::try_from(std::borrow::Cow::Owned(oversized.clone()))
    else {
        panic!("69ee1326");
    };
    let commit = super::GitCommitIdProvider::git_commit_id(oversized.as_str());
    assert!(commit.as_ref().len() <= super::GIT_INFO_STRING_MAX_LEN);
    let link = super::git_commit_link_cow(oversized.as_str());
    assert!(link.as_ref().len() <= super::GIT_INFO_STRING_MAX_LEN);
}
#[test]
fn git_commit_link_builds_expected_url() {
    let link = super::git_commit_link(constants_str::TEST_VALUES_COMMIT);
    assert_expected_git_commit_link(&link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn git_commit_link_supports_empty_commit() {
    let link = super::git_commit_link(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
    assert_expected_git_commit_link(&link, constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
}
#[test]
fn git_commit_link_cow_borrows_static_project_link_for_project_commit() {
    let project_commit = super::project_git_info().commit;
    let actual = super::git_commit_link_cow(project_commit);
    assert!(
        matches!(actual.0, std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, super::project_git_commit_link_ref().0))
    );
}
#[test]
fn git_commit_link_uses_static_project_link_for_project_commit() {
    let project_commit = super::project_git_info().commit;
    let actual = super::git_commit_link(project_commit);
    assert_eq!(actual, super::project_git_commit_link_ref());
}
#[test]
fn git_commit_link_cow_owns_link_for_non_project_commit() {
    let actual = super::git_commit_link_cow(constants_str::TEST_VALUES_WRONG_COMMIT);
    assert!(
        matches!(actual.0, std::borrow::Cow::Owned(v) if v == expected_git_commit_link("deadbeef"))
    );
}
#[test]
fn is_project_commit_returns_true_for_project_commit() {
    assert!(super::is_project_commit(super::project_git_info().commit));
}
#[test]
fn is_project_commit_returns_false_for_other_commit() {
    assert!(!super::is_project_commit("deadbeef"));
}
#[test]
fn validate_project_commit_returns_ok_for_project_commit() {
    assert_eq!(
        super::validate_project_commit(super::project_git_info().commit),
        Ok(())
    );
}
#[test]
fn validate_project_commit_returns_project_link_for_non_project_commit() {
    assert_eq!(
        super::validate_project_commit("deadbeef"),
        Err(super::ValidateProjectCommitError(
            super::project_git_commit_link_ref()
        ))
    );
}
#[test]
fn validate_project_commit_reuses_static_project_link_ref() {
    let error = super::validate_project_commit(constants_str::TEST_VALUES_WRONG_COMMIT)
        .expect_err(constants_str::VALUE_46BC13A9);
    let project_link = super::project_git_commit_link_ref();
    assert!(std::ptr::eq(error.0.0, project_link.0));
}
#[test]
fn project_git_commit_link_matches_project_commit() {
    assert_eq!(
        super::project_git_commit_link(),
        expected_git_commit_link(super::project_git_info().commit)
    );
}
#[test]
fn project_git_commit_link_ref_is_static_and_stable() {
    let first = super::project_git_commit_link_ref();
    let second = super::project_git_commit_link_ref();
    assert_eq!(first, second);
    assert!(std::ptr::eq(first.0, second.0));
}
#[test]
fn project_git_info_returns_commit_link() {
    let git_info = super::ProjectGitInfo {
        commit: super::GitCommitIdRef::from(constants_str::TEST_VALUES_WRONG_COMMIT),
    };
    let link = super::GitCommitLinkProvider::git_commit_link(&git_info);
    assert_expected_git_commit_link(&link, constants_str::TEST_VALUES_WRONG_COMMIT);
}
#[test]
fn git_commit_link_uses_trait_based_commit_id() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    assert_commit_link_and_fallback_calls(&test_git_commit, constants_str::F00DBABE, 1);
}
#[test]
fn git_commit_link_calls_allocating_fallback_once_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    drop(super::GitCommitLinkProvider::git_commit_link(
        &test_git_commit,
    ));
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn git_commit_id_or_else_computes_fallback_once() {
    let test_git_commit = make_owned_test_git_commit(constants_str::F00DBABE);
    let mut fallback = super::GitCommitIdFallback::from(None);
    let first = super::GitCommitIdProvider::git_commit_id_or_else(&test_git_commit, &mut fallback);
    assert_eq!(first, "f00dbabe");
    let second = super::GitCommitIdProvider::git_commit_id_or_else(&test_git_commit, &mut fallback);
    assert_eq!(second, "f00dbabe");
    assert_fallback_calls(&test_git_commit, 1);
}
#[test]
fn git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    let mut fallback = super::GitCommitIdFallback::from(None);
    let commit = super::GitCommitIdProvider::git_commit_id_or_else(&test_git_commit, &mut fallback);
    assert_eq!(commit, "cafebabe");
    assert_fallback_calls(&test_git_commit, 0);
    assert!(fallback.0.is_none());
}
#[test]
fn git_commit_id_cow_returns_owned_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_commit_id_cow_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, false, 1);
}
#[test]
fn git_commit_link_prefers_borrowed_commit_id() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_link_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, 0);
}
#[test]
fn git_commit_link_cow_borrows_project_link_for_project_commit() {
    let git_info = super::ProjectGitInfo {
        commit: super::project_git_info().commit,
    };
    let link = super::GitCommitLinkProvider::git_commit_link_cow(&git_info);
    assert!(
        matches!(link.0, std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, super::project_git_commit_link_ref().0))
    );
}
#[test]
fn git_commit_id_cow_returns_borrowed_when_ref_is_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_id_cow_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE, true, 0);
}
#[test]
fn with_git_commit_id_uses_allocating_fallback_once_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_commit_len_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE.len(), 1);
}
#[test]
fn with_git_commit_id_prefers_borrowed_ref_when_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_commit_len_and_fallback_calls(&test_git_commit, constants_str::CAFEBABE.len(), 0);
}
#[test]
fn with_git_commit_id_ref_or_prefers_borrowed_ref_when_available() {
    let test_git_commit = make_borrowed_test_git_commit(constants_str::CAFEBABE);
    assert_with_git_commit_id_ref_or(&test_git_commit, constants_str::CAFEBABE.len(), 0);
}
#[test]
fn with_git_commit_id_ref_or_uses_fallback_without_ref() {
    let test_git_commit = make_owned_test_git_commit(constants_str::CAFEBABE);
    assert_with_git_commit_id_ref_or(&test_git_commit, constants_str::CAFEBABE.len(), 1);
}
#[test]
fn base_git_commit_link_len_matches_expected_prefix_len() {
    let commit_id = constants_str::TEST_VALUES_COMMIT;
    let expected = format!("{}/tree/{commit_id}", constants_str::NAMING_GITHUB_URL).len();
    assert_eq!(super::git_commit_link_capacity(commit_id), expected);
}
#[test]
fn git_commit_link_works_for_str_and_string() {
    let str_link = super::GitCommitLinkProvider::git_commit_link(constants_str::TEST_VALUES_COMMIT);
    assert_expected_git_commit_link(&str_link, constants_str::TEST_VALUES_COMMIT);
    let string = String::from(constants_str::TEST_VALUES_COMMIT);
    let string_link = super::GitCommitLinkProvider::git_commit_link(&string);
    assert_expected_git_commit_link(&string_link, constants_str::TEST_VALUES_COMMIT);
}
#[test]
fn git_commit_link_works_for_cow_str() {
    let borrowed = std::borrow::Cow::Borrowed(constants_str::TEST_VALUES_COMMIT);
    let borrowed_link = super::GitCommitLinkProvider::git_commit_link(&borrowed);
    assert_expected_git_commit_link(&borrowed_link, constants_str::TEST_VALUES_COMMIT);
    let owned = std::borrow::Cow::<'_, str>::Owned(constants_str::TEST_VALUES_COMMIT.to_owned());
    assert_expected_git_commit_link(
        super::GitCommitLinkProvider::git_commit_link(&owned),
        constants_str::TEST_VALUES_COMMIT,
    );
}
#[test]
fn project_git_info_as_ref_returns_commit() {
    let info = super::ProjectGitInfo {
        commit: super::GitCommitIdRef::from(constants_str::TEST_VALUES_COMMIT),
    };
    assert_eq!(info.as_ref(), "abc123");
}
#[test]
fn git_commit_link_capacity_supports_empty_commit() {
    assert_eq!(
        super::git_commit_link_capacity(""),
        constants_str::NAMING_GITHUB_URL.len() + constants_str::GIT_INFO_TREE_SEGMENT.len()
    );
}
