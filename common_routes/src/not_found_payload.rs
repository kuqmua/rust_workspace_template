#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::OpenApiSpecificationPath;

#[derive(Debug, serde::Serialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NotFoundPayload {
    pub(super) commit: git_info::GitCommitLinkCow,
    pub(super) message: to_err_string::domain_types::ErrorText,
    pub(super) open_api_specification: OpenApiSpecificationPath,
}
