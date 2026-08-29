#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]

mod base_git_commit_link_len;
mod build_git_commit_link;
mod build_git_commit_link_cow;
mod check_is_project_commit;
pub use build_git_commit_link::*;
pub use build_git_commit_link_cow::*;
pub use check_is_project_commit::*;
pub use git_commit_id::*;
pub use git_commit_id_cow::*;
pub use git_commit_id_fallback::*;
pub use git_commit_id_provider::*;
pub use git_commit_id_ref::*;
pub use git_commit_link::*;
pub use git_commit_link_capacity::*;
pub use git_commit_link_capacity_value::*;
pub use git_commit_link_cow::*;
pub use git_commit_link_provider::*;
pub use git_info_string_try_from_string_error::*;
pub use is_project_commit::*;
pub use project_git_commit_link::*;
pub use project_git_commit_link_ref::*;
pub use project_git_commit_link_ref_value::*;
pub use project_git_info::*;
pub use project_git_info_value::*;
pub use validate_project_commit::*;
pub use validate_project_commit_error::*;
mod git_commit_id;
mod git_commit_id_cow;
mod git_commit_id_fallback;
mod git_commit_id_provider;
mod git_commit_id_ref;
mod git_commit_link;
mod git_commit_link_capacity;
mod git_commit_link_capacity_value;
mod git_commit_link_cow;
mod git_commit_link_provider;
mod git_info_string_max_len;
mod git_info_string_try_from_string_error;
mod is_project_commit;
mod project_git_commit_link;
mod project_git_commit_link_ref;
mod project_git_commit_link_ref_value;
mod project_git_info;
mod project_git_info_value;
#[cfg(test)]
mod tests;
mod validate_project_commit;
mod validate_project_commit_error;
mod with_git_commit_id_ref_or;
