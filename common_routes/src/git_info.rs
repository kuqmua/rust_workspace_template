#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct GitInfo {
    #[schema(value_type = String)]
    pub(super) commit: git_info::git_commit_link_cow::GitCommitLinkCow,
}
