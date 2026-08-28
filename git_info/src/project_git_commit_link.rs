use crate::domain_types::{GitCommitLink, project_git_commit_link_ref_value};

#[must_use]
pub fn project_git_commit_link() -> GitCommitLink {
    GitCommitLink::try_from(project_git_commit_link_ref_value().0.to_owned())
        .unwrap_or_else(GitCommitLink::from)
}
