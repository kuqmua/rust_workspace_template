const BASE_GIT_COMMIT_LINK_LEN: usize =
    constants_str::NAMING_GITHUB_URL.len() + constants_str::GIT_INFO_TREE_SEGMENT.len();
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Default,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(
    error = GitInfoStringTryFromStringError,
    validator = GitCommitId::validate
)]
pub struct GitCommitId(String);
impl GitCommitId {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), GitInfoStringTryFromStringError> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(())
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefStr,
)]
pub struct GitCommitIdCow<'commit_lt>(std::borrow::Cow<'commit_lt, str>);
impl<'commit_lt> TryFrom<std::borrow::Cow<'commit_lt, str>> for GitCommitIdCow<'commit_lt> {
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
impl From<GitInfoStringTryFromStringError> for GitCommitIdCow<'_> {
    fn from(value: GitInfoStringTryFromStringError) -> Self {
        Self(std::borrow::Cow::Owned(value.to_string()))
    }
}
#[derive(
    Debug, Clone, PartialEq, Eq, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner,
)]
pub struct GitCommitIdFallback(Option<GitCommitId>);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::PartialEqInner,
    newtype::TryFrom,
)]
#[try_from(
    error = GitInfoStringTryFromStringError,
    validator = GitCommitLink::validate
)]
pub struct GitCommitLink(String);
impl GitCommitLink {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), GitInfoStringTryFromStringError> {
        if value.len() > GIT_INFO_STRING_MAX_LEN {
            Err(GitInfoStringTryFromStringError::TooLong {
                len: value.len(),
                max: GIT_INFO_STRING_MAX_LEN,
            })
        } else {
            Ok(())
        }
    }
}
impl From<GitCommitLinkCow> for GitCommitLink {
    fn from(value: GitCommitLinkCow) -> Self {
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
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_derive::Deserialize,
    serde_derive::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefStr,
    newtype::Display,
)]
#[serde(try_from = "String")]
pub struct GitCommitLinkCow(std::borrow::Cow<'static, str>);
impl TryFrom<String> for GitCommitLinkCow {
    type Error = GitInfoStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(std::borrow::Cow::Owned(value))
    }
}
impl TryFrom<std::borrow::Cow<'static, str>> for GitCommitLinkCow {
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
impl From<GitInfoStringTryFromStringError> for GitCommitLinkCow {
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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefInner,
    newtype::Display,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ProjectGitCommitLinkRef(&'static str);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::NotInner,
)]
pub struct IsProjectCommit(bool);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::PartialEqInner,
)]
pub struct GitCommitLinkCapacity(usize);
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct GitCommitLinkOutputRefMut<'output_lt>(&'output_lt mut String);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ValidateProjectCommitError(ProjectGitCommitLinkRef);
#[derive(
    Debug,
    serde_derive::Serialize,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    optimal_memory_layout::OptimalMemoryLayout,
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
pub trait GitCommitLinkProvider {
    fn git_commit_link(&self) -> GitCommitLink {
        self.git_commit_link_cow().into()
    }
    fn git_commit_link_cow(&self) -> GitCommitLinkCow;
}
pub trait GitCommitIdProvider {
    fn git_commit_id(&self) -> GitCommitId;
    fn git_commit_id_cow(&self) -> GitCommitIdCow<'_> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| {
                GitCommitIdCow::try_from(std::borrow::Cow::Borrowed(commit_id.0))
                    .unwrap_or_else(GitCommitIdCow::from)
            },
            |src| {
                GitCommitIdCow::try_from(std::borrow::Cow::Owned(src.git_commit_id().0))
                    .unwrap_or_else(GitCommitIdCow::from)
            },
        )
    }
    fn git_commit_id_or_else<'commit_id_lt>(
        &'commit_id_lt self,
        fallback: &'commit_id_lt mut GitCommitIdFallback,
    ) -> GitCommitIdRef<'commit_id_lt> {
        with_git_commit_id_ref_or(
            self,
            |commit_id| commit_id,
            |src| {
                fallback
                    .0
                    .get_or_insert_with(|| src.git_commit_id())
                    .0
                    .as_str()
                    .into()
            },
        )
    }
    fn git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        None
    }
    fn with_git_commit_id<R>(&self, f: impl FnOnce(GitCommitIdRef<'_>) -> R) -> R {
        let mut fallback = GitCommitIdFallback::from(None);
        f(self.git_commit_id_or_else(&mut fallback))
    }
}
impl<T: ?Sized + AsRef<str>> GitCommitIdProvider for T {
    fn git_commit_id(&self) -> GitCommitId {
        GitCommitId::try_from(self.as_ref().to_owned()).unwrap_or_else(GitCommitId::from)
    }
    fn git_commit_id_ref(&self) -> Option<GitCommitIdRef<'_>> {
        Some(GitCommitIdRef::from(self.as_ref()))
    }
}
impl<T: ?Sized + GitCommitIdProvider> GitCommitLinkProvider for T {
    fn git_commit_link_cow(&self) -> GitCommitLinkCow {
        self.with_git_commit_id(|commit_id| git_commit_link_cow(commit_id))
    }
}
fn with_git_commit_id_ref_or<'src, T, R>(
    src: &'src T,
    on_ref: impl FnOnce(GitCommitIdRef<'src>) -> R,
    on_owned: impl FnOnce(&'src T) -> R,
) -> R
where
    T: ?Sized + GitCommitIdProvider,
{
    src.git_commit_id_ref()
        .map_or_else(|| on_owned(src), on_ref)
}
#[must_use]
pub fn project_git_info() -> ProjectGitInfo<'static> {
    ProjectGitInfo::from(GitCommitIdRef::from(
        constants_str::GIT_INFO_PROJECT_GIT_COMMIT_ID,
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
    ProjectGitCommitLinkRef::from(constants_str::GIT_INFO_PROJECT_GIT_COMMIT_LINK)
}
#[must_use]
pub fn git_commit_link<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLink
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    git_commit_link_cow(commit_id).into()
}
#[must_use]
pub fn git_commit_link_cow<'commit_lt, CommitIdTy>(commit_id: CommitIdTy) -> GitCommitLinkCow
where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    if commit_id_ref.0.len() > GIT_INFO_STRING_MAX_LEN.saturating_sub(BASE_GIT_COMMIT_LINK_LEN) {
        return GitCommitLinkCow::try_from(std::borrow::Cow::Owned(
            GitInfoStringTryFromStringError::TooLong {
                len: BASE_GIT_COMMIT_LINK_LEN.saturating_add(commit_id_ref.0.len()),
                max: GIT_INFO_STRING_MAX_LEN,
            }
            .to_string(),
        ))
        .unwrap_or_else(GitCommitLinkCow::from);
    }
    if is_project_commit(commit_id_ref).0 {
        return GitCommitLinkCow::try_from(std::borrow::Cow::Borrowed(
            project_git_commit_link_ref().0,
        ))
        .unwrap_or_else(GitCommitLinkCow::from);
    }
    let cap = git_commit_link_capacity(commit_id_ref);
    let mut output = String::with_capacity(cap.0);
    let mut output_ref = GitCommitLinkOutputRefMut::from(&mut output);
    write_git_commit_link(&mut output_ref, commit_id_ref);
    GitCommitLinkCow::try_from(std::borrow::Cow::Owned(output))
        .unwrap_or_else(GitCommitLinkCow::from)
}
#[allow(clippy::single_call_fn)] // shared writer keeps link assembly consistent across builders and tests
fn write_git_commit_link<'commit_lt, CommitIdTy>(
    output: &mut GitCommitLinkOutputRefMut<'_>,
    commit_id: CommitIdTy,
) where
    CommitIdTy: Into<GitCommitIdRef<'commit_lt>>,
{
    let commit_id_ref = commit_id.into();
    output.0.push_str(constants_str::NAMING_GITHUB_URL);
    output.0.push_str(constants_str::GIT_INFO_TREE_SEGMENT);
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
#[path = "tests.rs"]
mod tests;
