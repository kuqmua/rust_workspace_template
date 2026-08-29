#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]

pub mod base_git_commit_link_len;
pub mod build_git_commit_link;
pub mod build_git_commit_link_cow;
pub mod check_is_project_commit;
pub mod git_commit_id;
pub mod git_commit_id_cow;
pub mod git_commit_id_fallback;
pub mod git_commit_id_provider;
pub mod git_commit_id_ref;
pub mod git_commit_link;
pub mod git_commit_link_capacity;
pub mod git_commit_link_capacity_value;
pub mod git_commit_link_cow;
pub mod git_commit_link_provider;
pub mod git_info_string_max_len;
pub mod git_info_string_try_from_string_error;
pub mod is_project_commit;
pub mod project_git_commit_link;
pub mod project_git_commit_link_ref;
pub mod project_git_commit_link_ref_value;
pub mod project_git_info;
pub mod project_git_info_value;
#[cfg(test)]
pub mod tests;
pub mod validate_project_commit;
pub mod validate_project_commit_error;
pub mod with_git_commit_id_ref_or;
