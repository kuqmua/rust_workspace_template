use super::super::{GitCommitLink, project_git_commit_link_ref};

#[must_use]
pub fn project_git_commit_link() -> GitCommitLink {
    GitCommitLink::try_from(project_git_commit_link_ref().0.to_owned())
        .unwrap_or_else(GitCommitLink::from)
}
