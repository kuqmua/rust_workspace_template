#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(Debug, serde::Serialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NotFoundPayload {
    pub(super) commit: git_info::git_commit_link_cow::GitCommitLinkCow,
    pub(super) message: to_err_string::error_text::ErrorText,
    pub(super) open_api_specification: crate::open_api_specification_path::OpenApiSpecificationPath,
}
