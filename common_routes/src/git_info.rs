#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct GitInfo {
    #[schema(value_type = String)]
    commit: git_info::git_commit_link_cow::GitCommitLinkCow,
}

impl GitInfo {
    #[allow(
        clippy::single_call_fn,
        reason = "the contract owner constructs its private serialized field for the route adapter"
    )]
    pub(super) const fn from_commit(
        git_commit_link_cow: git_info::git_commit_link_cow::GitCommitLinkCow,
    ) -> Self {
        Self {
            commit: git_commit_link_cow,
        }
    }

    #[cfg(test)]
    pub(crate) fn commit_matches(&self, str: &str) -> bool {
        self.commit.as_ref() == str
    }
}
