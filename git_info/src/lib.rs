pub const PROJECT_GIT_INFO: ProjectGitInfo = ProjectGitInfo {
    commit: ProjectGitCommitId("workspace-template"),
};

const GIT_TREE_SEGMENT: GitTreeSegment = GitTreeSegment("/tree/");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitCommitLinkCapacity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitCommitLinkText {
    commit_id: ProjectGitCommitId,
    repository_url: GitRepositoryUrl,
    tree_segment: GitTreeSegment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitRepositoryUrl(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GitTreeSegment(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectGitCommitId(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectGitInfo {
    commit: ProjectGitCommitId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsProjectCommit {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidateProjectCommitOutcome {
    Invalid { expected_link: GitCommitLinkText },
    Valid,
}

pub trait GetGitCommitId {
    fn get_git_commit_id(&self) -> ProjectGitCommitId;
}

pub trait GetGitCommitLink {
    fn get_git_commit_link(&self) -> GitCommitLinkText;
}

impl AsRef<str> for ProjectGitCommitId {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for GitRepositoryUrl {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for GitTreeSegment {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl core::fmt::Display for GitCommitLinkText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(
            f,
            "{}{}{}",
            self.repository_url.as_ref(),
            self.tree_segment.as_ref(),
            self.commit_id.as_ref()
        )
    }
}

impl GetGitCommitId for ProjectGitInfo {
    fn get_git_commit_id(&self) -> ProjectGitCommitId {
        self.commit
    }
}

impl GetGitCommitLink for ProjectGitInfo {
    fn get_git_commit_link(&self) -> GitCommitLinkText {
        project_git_commit_link()
    }
}

#[must_use]
pub fn is_project_commit(commit_id: &ProjectGitCommitId) -> IsProjectCommit {
    if commit_id.as_ref() == PROJECT_GIT_INFO.commit.as_ref() {
        return IsProjectCommit::True;
    }
    IsProjectCommit::False
}

#[must_use]
pub fn validate_project_commit(commit_id: &ProjectGitCommitId) -> ValidateProjectCommitOutcome {
    match is_project_commit(commit_id) {
        IsProjectCommit::False => ValidateProjectCommitOutcome::Invalid {
            expected_link: project_git_commit_link(),
        },
        IsProjectCommit::True => ValidateProjectCommitOutcome::Valid,
    }
}

#[must_use]
pub const fn project_git_commit_id() -> ProjectGitCommitId {
    PROJECT_GIT_INFO.commit
}

#[must_use]
pub const fn project_git_commit_link() -> GitCommitLinkText {
    GitCommitLinkText {
        commit_id: PROJECT_GIT_INFO.commit,
        repository_url: GitRepositoryUrl(naming_constants::GITHUB_URL),
        tree_segment: GIT_TREE_SEGMENT,
    }
}

#[must_use]
pub const fn git_repository_url() -> GitRepositoryUrl {
    GitRepositoryUrl(naming_constants::GITHUB_URL)
}

#[must_use]
pub const fn git_commit_link_capacity(_commit_id: &ProjectGitCommitId) -> GitCommitLinkCapacity {
    GitCommitLinkCapacity
}
