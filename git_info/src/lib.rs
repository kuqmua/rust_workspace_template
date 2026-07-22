const BASE_GIT_COMMIT_LINK_LEN: usize =
    str_constants::NAMING_GITHUB_URL.len() + str_constants::GIT_INFO_TREE_SEGMENT.len();
const GIT_INFO_STRING_MAX_LEN: usize = 1_048_576;
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    serde_derive::Serialize,
    optml::Optml,
    newtype::AsRefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct GitCommitIdRef<'commit_lt>(&'commit_lt str);
impl PartialEq<&str> for GitCommitIdRef<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, Default, optml::Optml, newtype::AsRefStr, newtype::TryFrom,
)]
#[try_from(
    error = GitInfoStringTryFromStringError,
    validator = |value: &String| if value.len() > GIT_INFO_STRING_MAX_LEN {
        Err(GitInfoStringTryFromStringError::TooLong {
            len: value.len(),
            max: GIT_INFO_STRING_MAX_LEN,
        })
    } else {
        Ok(())
    }
)]
pub struct GitCommitId(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitInfoStringTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for GitInfoStringTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "git info string length {len} exceeds maximum {max}")
            }
        }
    }
}
impl From<GitCommitIdRef<'_>> for GitCommitId {
    fn from(value: GitCommitIdRef<'_>) -> Self {
        Self::try_from(value.0.to_owned()).unwrap_or_else(Self::from)
    }
}
impl From<GitInfoStringTryFromStringError> for GitCommitId {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::AsRefStr)]
pub struct StdGitCommitIdCow<'commit_lt>(std::borrow::Cow<'commit_lt, str>);
impl<'commit_lt> TryFrom<std::borrow::Cow<'commit_lt, str>> for StdGitCommitIdCow<'commit_lt> {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: std::borrow::Cow<'commit_lt, str>) -> Result<Self, Self::Error> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl From<GitInfoStringTryFromStringError> for StdGitCommitIdCow<'_> {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::FromInner)]
pub struct GitCommitIdFallback(Option<GitCommitId>);
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::AsRefStr, newtype::TryFrom)]
#[try_from(
    error = GitInfoStringTryFromStringError,
    validator = |value: &String| if value.len() > GIT_INFO_STRING_MAX_LEN {
        Err(GitInfoStringTryFromStringError::TooLong {
            len: value.len(),
            max: GIT_INFO_STRING_MAX_LEN,
        })
    } else {
        Ok(())
    }
)]
pub struct GitCommitLink(String);
impl From<StdGitCommitLinkCow> for GitCommitLink {
    fn from(value: StdGitCommitLinkCow) -> Self {
        Self::try_from(value.0.into_owned()).unwrap_or_else(Self::from)
    }
}
impl From<GitInfoStringTryFromStringError> for GitCommitLink {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl PartialEq<ProjectGitCommitLinkRef> for GitCommitLink {
    fn eq(&self, other: &ProjectGitCommitLinkRef) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<String> for GitCommitLink {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_derive::Serialize,
    optml::Optml,
    newtype::AsRefStr,
    newtype::Display,
)]
pub struct StdGitCommitLinkCow(std::borrow::Cow<'static, str>);
impl TryFrom<std::borrow::Cow<'static, str>> for StdGitCommitLinkCow {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: std::borrow::Cow<'static, str>) -> Result<Self, Self::Error> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl From<GitInfoStringTryFromStringError> for StdGitCommitLinkCow {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optml::Optml,
    newtype::AsRefInner,
    newtype::Display,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ProjectGitCommitLinkRef(&'static str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner)]
pub struct IsProjectCommit(bool);
impl std::ops::Not for IsProjectCommit {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.0
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::DerefInner, newtype::FromInner,
)]
pub struct GitCommitLinkCapacity(usize);
impl PartialEq<usize> for GitCommitLinkCapacity {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}
#[derive(Debug, optml::Optml, newtype::FromInner)]
struct GitCommitLinkOutputRefMut<'output_lt>(&'output_lt mut String);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optml::Optml,
    newtype::AsRefOwned,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ValidateProjectCommitError(ProjectGitCommitLinkRef);
#[derive(
    Debug, serde_derive::Serialize, Clone, Copy, Hash, PartialEq, Eq, Default, optml::Optml,
)]
pub struct ProjectGitInfo<'commit_lt> {
    commit: GitCommitIdRef<'commit_lt>,
}
impl<'commit_lt> ProjectGitInfo<'commit_lt> {
    #[must_use]
    pub const fn commit(&self) -> GitCommitIdRef<'commit_lt> {
        self.commit
    }
}
impl<'commit_lt> From<GitCommitIdRef<'commit_lt>> for ProjectGitInfo<'commit_lt> {
    fn from(value: GitCommitIdRef<'commit_lt>) -> Self {
        Self { commit: value }
    }
}
impl AsRef<str> for ProjectGitInfo<'_> {
    fn as_ref(&self) -> &str {
        self.commit.0
    }
}
pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> GitCommitLink {
        self.get_git_commit_link_cow().into()
    }
    fn get_git_commit_link_cow(&self) -> StdGitCommitLinkCow;
}
pub trait GetGitCommitId {
    fn get_git_commit_id(&self) -> GitCommitId;
    fn get_git_commit_id_cow(&self) -> StdGitCommitIdCow<'_> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| {
                StdGitCommitIdCow::try_from(std::borrow::Cow::Borrowed(commit_id.0))
                    .unwrap_or_else(StdGitCommitIdCow::from)
            },
            |src| {
                StdGitCommitIdCow::try_from(std::borrow::Cow::Owned(src.get_git_commit_id().0))
                    .unwrap_or_else(StdGitCommitIdCow::from)
            },
        )
    }
    fn get_git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut GitCommitIdFallback,
    ) -> GitCommitIdRef<'commit_id_lt> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| commit_id,
            |src| {
                fallback
                    .0
                    .get_or_insert_with(|| src.get_git_commit_id())
                    .0
                    .as_str()
                    .into()
            },
        )
    }
    fn get_git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        None
    }
    fn with_git_commit_id<R>(&self, f: impl FnOnce(GitCommitIdRef<'_>) -> R) -> R {
        let mut fallback = GitCommitIdFallback::from(None);
        f(self.get_git_commit_id_or_else(&mut fallback))
    }
}
impl<T: ?Sized + AsRef<str>> GetGitCommitId for T {
    fn get_git_commit_id(&self) -> GitCommitId {
        GitCommitId::try_from(self.as_ref().to_owned()).unwrap_or_else(GitCommitId::from)
    }
    fn get_git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        Some(GitCommitIdRef::from(self.as_ref()))
    }
}
impl<T: ?Sized + GetGitCommitId> GetGitCommitLink for T {
    fn get_git_commit_link_cow(&self) -> StdGitCommitLinkCow {
        self.with_git_commit_id(|commit_id| git_commit_link_cow(commit_id))
    }
}
fn with_git_commit_id_ref_or<'src, T, R>(
    src: &'src T,
    on_ref: impl FnOnce(GitCommitIdRef<'src>) -> R,
    on_owned: impl FnOnce(&'src T) -> R,
) -> R
where
    T: ?Sized + GetGitCommitId,
{
    src.get_git_commit_id_ref()
        .map_or_else(|| on_owned(src), on_ref)
}
#[must_use]
pub fn project_git_info() -> ProjectGitInfo<'static> {
    ProjectGitInfo::from(GitCommitIdRef::from(
        str_constants::GIT_INFO_PROJECT_GIT_COMMIT_ID,
    ))
}
#[must_use]
pub fn is_project_commit<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> IsProjectCommit
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    IsProjectCommit::from(commit_id_ref.0 == project_git_info().commit.0)
}
pub fn validate_project_commit<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> Result<(), ValidateProjectCommitError>
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    if is_project_commit(commit_id).0 {
        return Ok(());
    }
    Err(ValidateProjectCommitError::from(
        project_git_commit_link_ref(),
    ))
}
#[must_use]
pub fn project_git_commit_link() -> GitCommitLink {
    GitCommitLink::try_from(project_git_commit_link_ref().0.to_owned())
        .unwrap_or_else(GitCommitLink::from)
}
#[must_use]
pub fn project_git_commit_link_ref() -> ProjectGitCommitLinkRef {
    ProjectGitCommitLinkRef::from(str_constants::GIT_INFO_PROJECT_GIT_COMMIT_LINK)
}
#[must_use]
pub fn git_commit_link<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLink
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    git_commit_link_cow(commit_id).into()
}
#[must_use]
pub fn git_commit_link_cow<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> StdGitCommitLinkCow
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    if commit_id_ref.0.len() > GIT_INFO_STRING_MAX_LEN.saturating_sub(BASE_GIT_COMMIT_LINK_LEN) {
        return StdGitCommitLinkCow::try_from(std::borrow::Cow::Owned(
            GitInfoStringTryFromStringError::TooLong {
                len: BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id_ref.0.len()),
                max: GIT_INFO_STRING_MAX_LEN,
            }
            .to_string(),
        ))
        .unwrap_or_else(StdGitCommitLinkCow::from);
    }
    if is_project_commit(commit_id_ref).0 {
        return StdGitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(
            project_git_commit_link_ref().0,
        ))
        .unwrap_or_else(StdGitCommitLinkCow::from);
    }
    let cap = git_commit_link_capacity(commit_id_ref);
    let mut output = String::with_capacity(cap.0);
    let mut output_ref = GitCommitLinkOutputRefMut::from(&mut output);
    write_git_commit_link(&mut output_ref, commit_id_ref);
    StdGitCommitLinkCow::try_from(std::borrow::Cow::Owned(output))
        .unwrap_or_else(StdGitCommitLinkCow::from)
}
#[allow(clippy::single_call_fn)] // shared writer keeps link assembly consistent across builders and tests
fn write_git_commit_link<'commit_lt, CommitIdTy>(
    output: &mut GitCommitLinkOutputRefMut<'_>,
    commit_id: CommitIdTy,
) where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    output.0.push_str(str_constants::NAMING_GITHUB_URL);
    output.0.push_str(str_constants::GIT_INFO_TREE_SEGMENT);
    output.0.push_str(commit_id_ref.0);
}
#[must_use]
pub fn git_commit_link_capacity<'commit_lt, CommitIdTy>(
    commit_id: CommitIdTy,
) -> GitCommitLinkCapacity
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    GitCommitLinkCapacity::from(BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id_ref.0.len()))
}
#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct TestGitCommit {
        borrow_commit_ref: bool,
        commit: &'static str,
        fallback_calls: std::cell::Cell<usize>,
    }
    impl super::GetGitCommitId for TestGitCommit {
        fn get_git_commit_id(&self) -> super::GitCommitId {
            let calls = self.fallback_calls.get().saturating_add(1);
            self.fallback_calls.set(calls);
            super::GitCommitId::try_from(self.commit.to_owned()).expect("45a9c31d")
        }
        fn get_git_commit_id_ref(&self) -> Option<super::GitCommitIdRef<'_>> {
            self.borrow_commit_ref
                .then_some(super::GitCommitIdRef::from(self.commit))
        }
    }
    fn mk_test_git_commit(commit: &'static str, borrow_commit_ref: bool) -> TestGitCommit {
        TestGitCommit {
            commit,
            borrow_commit_ref,
            fallback_calls: std::cell::Cell::new(0),
        }
    }
    fn mk_owned_test_git_commit(commit: &'static str) -> TestGitCommit {
        mk_test_git_commit(commit, false)
    }
    fn mk_borrowed_test_git_commit(commit: &'static str) -> TestGitCommit {
        mk_test_git_commit(commit, true)
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps fallback-call expectations consistent across owned/borrowed commit-id tests
    fn assert_fallback_calls(v: &TestGitCommit, exp: usize) {
        assert_eq!(v.fallback_calls.get(), exp);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps commit-link equality checks concise across tests
    fn assert_expected_git_commit_link(actual: impl AsRef<str>, exp_commit_id: &str) {
        assert_eq!(actual.as_ref(), expected_git_commit_link(exp_commit_id));
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps borrowed/owned Cow-kind assertions consistent across commit-id tests
    fn assert_commit_id_cow_kind(
        commit_id: &str,
        is_borrowed: bool,
        exp_commit_id: &str,
        exp_is_borrowed: bool,
    ) {
        assert_eq!(commit_id, exp_commit_id);
        assert_eq!(is_borrowed, exp_is_borrowed);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps link-output and fallback-call expectations coupled for test clarity
    fn assert_commit_link_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_id: &str,
        exp_fallback_calls: usize,
    ) {
        let link = super::GetGitCommitLink::get_git_commit_link(v);
        assert_expected_git_commit_link(&link, exp_commit_id);
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps borrowed/owned Cow expectations concise across commit-id tests
    fn assert_commit_id_cow_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_id: &str,
        exp_is_borrowed: bool,
        exp_fallback_calls: usize,
    ) {
        let commit_id = super::GetGitCommitId::get_git_commit_id_cow(v);
        assert_commit_id_cow_kind(
            commit_id.as_ref(),
            matches!(&commit_id.0, std::borrow::Cow::Borrowed(_)),
            exp_commit_id,
            exp_is_borrowed,
        );
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps commit-length and fallback-call checks coupled across borrowed/owned cases
    fn assert_commit_len_and_fallback_calls(
        v: &TestGitCommit,
        exp_commit_len: usize,
        exp_fallback_calls: usize,
    ) {
        let commit_len =
            super::GetGitCommitId::with_git_commit_id(v, |commit_id| commit_id.0.len());
        assert_eq!(commit_len, exp_commit_len);
        assert_fallback_calls(v, exp_fallback_calls);
    }
    #[allow(clippy::single_call_fn)] // shared assertion keeps with_git_commit_id_ref_or branch behavior checks reusable across borrowed/owned test cases
    fn assert_with_git_commit_id_ref_or(
        v: &TestGitCommit,
        exp_commit_len: usize,
        exp_fallback_calls: usize,
    ) {
        let commit_len = super::with_git_commit_id_ref_or(
            v,
            |commit_id| commit_id.0.len(),
            |src| super::GetGitCommitId::get_git_commit_id(src).0.len(),
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
        let oversized = "x".repeat(super::GIT_INFO_STRING_MAX_LEN + 1usize);
        let Err(_commit_id_error) =
            super::StdGitCommitIdCow::try_from(std::borrow::Cow::Owned(oversized.clone()))
        else {
            panic!("8c811508");
        };
        let Err(_commit_link_error) =
            super::StdGitCommitLinkCow::try_from(std::borrow::Cow::Owned(oversized.clone()))
        else {
            panic!("69ee1326");
        };
        let commit = super::GetGitCommitId::get_git_commit_id(oversized.as_str());
        assert!(commit.as_ref().len() <= super::GIT_INFO_STRING_MAX_LEN);
        let link = super::git_commit_link_cow(oversized.as_str());
        assert!(link.as_ref().len() <= super::GIT_INFO_STRING_MAX_LEN);
    }
    #[test]
    fn git_commit_link_builds_expected_url() {
        let link = super::git_commit_link(str_constants::TEST_VALUES_COMMIT);
        assert_expected_git_commit_link(&link, str_constants::TEST_VALUES_COMMIT);
    }
    #[test]
    fn git_commit_link_handles_empty_commit() {
        let link = super::git_commit_link(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX);
        assert_expected_git_commit_link(&link, str_constants::PG_CRUD_EMPTY_SQL_SUFFIX);
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
        let actual = super::git_commit_link_cow(str_constants::TEST_VALUES_WRONG_COMMIT);
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
        let error = super::validate_project_commit(str_constants::TEST_VALUES_WRONG_COMMIT)
            .expect_err(str_constants::VALUE_46BC13A9);
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
            commit: super::GitCommitIdRef::from(str_constants::TEST_VALUES_WRONG_COMMIT),
        };
        let link = super::GetGitCommitLink::get_git_commit_link(&git_info);
        assert_expected_git_commit_link(&link, str_constants::TEST_VALUES_WRONG_COMMIT);
    }
    #[test]
    fn get_git_commit_link_uses_trait_based_commit_id() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::F00DBABE);
        assert_commit_link_and_fallback_calls(&test_git_commit, str_constants::F00DBABE, 1);
    }
    #[test]
    fn get_git_commit_link_calls_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::F00DBABE);
        drop(super::GetGitCommitLink::get_git_commit_link(
            &test_git_commit,
        ));
        assert_fallback_calls(&test_git_commit, 1);
    }
    #[test]
    fn get_git_commit_id_or_else_computes_fallback_once() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::F00DBABE);
        let mut fallback = super::GitCommitIdFallback::from(None);
        let first =
            super::GetGitCommitId::get_git_commit_id_or_else(&test_git_commit, &mut fallback);
        assert_eq!(first, "f00dbabe");
        let second =
            super::GetGitCommitId::get_git_commit_id_or_else(&test_git_commit, &mut fallback);
        assert_eq!(second, "f00dbabe");
        assert_fallback_calls(&test_git_commit, 1);
    }
    #[test]
    fn get_git_commit_id_or_else_prefers_borrowed_ref_without_fallback() {
        let test_git_commit = mk_borrowed_test_git_commit(str_constants::CAFEBABE);
        let mut fallback = super::GitCommitIdFallback::from(None);
        let commit =
            super::GetGitCommitId::get_git_commit_id_or_else(&test_git_commit, &mut fallback);
        assert_eq!(commit, "cafebabe");
        assert_fallback_calls(&test_git_commit, 0);
        assert!(fallback.0.is_none());
    }
    #[test]
    fn get_git_commit_id_cow_returns_owned_without_ref() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::CAFEBABE);
        assert_commit_id_cow_and_fallback_calls(
            &test_git_commit,
            str_constants::CAFEBABE,
            false,
            1,
        );
    }
    #[test]
    fn get_git_commit_link_prefers_borrowed_commit_id() {
        let test_git_commit = mk_borrowed_test_git_commit(str_constants::CAFEBABE);
        assert_commit_link_and_fallback_calls(&test_git_commit, str_constants::CAFEBABE, 0);
    }
    #[test]
    fn get_git_commit_link_cow_borrows_project_link_for_project_commit() {
        let git_info = super::ProjectGitInfo {
            commit: super::project_git_info().commit,
        };
        let link = super::GetGitCommitLink::get_git_commit_link_cow(&git_info);
        assert!(
            matches!(link.0, std::borrow::Cow::Borrowed(v) if std::ptr::eq(v, super::project_git_commit_link_ref().0))
        );
    }
    #[test]
    fn get_git_commit_id_cow_returns_borrowed_when_ref_is_available() {
        let test_git_commit = mk_borrowed_test_git_commit(str_constants::CAFEBABE);
        assert_commit_id_cow_and_fallback_calls(&test_git_commit, str_constants::CAFEBABE, true, 0);
    }
    #[test]
    fn with_git_commit_id_uses_allocating_fallback_once_without_ref() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::CAFEBABE);
        assert_commit_len_and_fallback_calls(&test_git_commit, str_constants::CAFEBABE.len(), 1);
    }
    #[test]
    fn with_git_commit_id_prefers_borrowed_ref_when_available() {
        let test_git_commit = mk_borrowed_test_git_commit(str_constants::CAFEBABE);
        assert_commit_len_and_fallback_calls(&test_git_commit, str_constants::CAFEBABE.len(), 0);
    }
    #[test]
    fn with_git_commit_id_ref_or_prefers_borrowed_ref_when_available() {
        let test_git_commit = mk_borrowed_test_git_commit(str_constants::CAFEBABE);
        assert_with_git_commit_id_ref_or(&test_git_commit, str_constants::CAFEBABE.len(), 0);
    }
    #[test]
    fn with_git_commit_id_ref_or_uses_fallback_without_ref() {
        let test_git_commit = mk_owned_test_git_commit(str_constants::CAFEBABE);
        assert_with_git_commit_id_ref_or(&test_git_commit, str_constants::CAFEBABE.len(), 1);
    }
    #[test]
    fn base_git_commit_link_len_matches_expected_prefix_len() {
        let commit_id = str_constants::TEST_VALUES_COMMIT;
        let expected = format!("{}/tree/{commit_id}", str_constants::NAMING_GITHUB_URL).len();
        assert_eq!(super::git_commit_link_capacity(commit_id), expected);
    }
    #[test]
    fn get_git_commit_link_works_for_str_and_string() {
        let str_link =
            super::GetGitCommitLink::get_git_commit_link(str_constants::TEST_VALUES_COMMIT);
        assert_expected_git_commit_link(&str_link, str_constants::TEST_VALUES_COMMIT);
        let string = String::from(str_constants::TEST_VALUES_COMMIT);
        let string_link = super::GetGitCommitLink::get_git_commit_link(&string);
        assert_expected_git_commit_link(&string_link, str_constants::TEST_VALUES_COMMIT);
    }
    #[test]
    fn get_git_commit_link_works_for_cow_str() {
        let borrowed = std::borrow::Cow::Borrowed(str_constants::TEST_VALUES_COMMIT);
        let borrowed_link = super::GetGitCommitLink::get_git_commit_link(&borrowed);
        assert_expected_git_commit_link(&borrowed_link, str_constants::TEST_VALUES_COMMIT);
        let owned =
            std::borrow::Cow::<'_, str>::Owned(str_constants::TEST_VALUES_COMMIT.to_owned());
        assert_expected_git_commit_link(
            super::GetGitCommitLink::get_git_commit_link(&owned),
            str_constants::TEST_VALUES_COMMIT,
        );
    }
    #[test]
    fn project_git_info_as_ref_returns_commit() {
        let info = super::ProjectGitInfo {
            commit: super::GitCommitIdRef::from(str_constants::TEST_VALUES_COMMIT),
        };
        assert_eq!(info.as_ref(), "abc123");
    }
    #[test]
    fn git_commit_link_capacity_handles_empty_commit() {
        assert_eq!(
            super::git_commit_link_capacity(""),
            str_constants::NAMING_GITHUB_URL.len() + str_constants::GIT_INFO_TREE_SEGMENT.len()
        );
    }
}
