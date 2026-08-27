#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]
#[path = "domain_types/base_git_commit_link_len.rs"]
mod base_git_commit_link_len;
use base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN;
#[path = "domain_types/git_info_string_max_len.rs"]
mod git_info_string_max_len;
use git_info_string_max_len::GIT_INFO_STRING_MAX_LEN;
#[path = "domain_types/git_commit_id_ref.rs"]
mod git_commit_id_ref;
pub use git_commit_id_ref::*;
#[path = "domain_types/git_commit_id.rs"]
mod git_commit_id;
pub use git_commit_id::*;
#[path = "domain_types/git_info_string_try_from_string_error.rs"]
mod git_info_string_try_from_string_error;
pub use git_info_string_try_from_string_error::*;
#[path = "domain_types/git_commit_id_cow.rs"]
mod git_commit_id_cow;
pub use git_commit_id_cow::*;
#[path = "domain_types/git_commit_id_fallback.rs"]
mod git_commit_id_fallback;
pub use git_commit_id_fallback::*;
#[path = "domain_types/git_commit_link.rs"]
mod git_commit_link;
pub use git_commit_link::*;
#[path = "domain_types/git_commit_link_cow.rs"]
mod git_commit_link_cow;
pub use git_commit_link_cow::*;
#[path = "domain_types/project_git_commit_link_ref.rs"]
mod project_git_commit_link_ref;
pub use project_git_commit_link_ref::*;
#[path = "domain_types/is_project_commit.rs"]
mod is_project_commit;
pub use is_project_commit::*;
#[path = "domain_types/git_commit_link_capacity.rs"]
mod git_commit_link_capacity;
pub use git_commit_link_capacity::*;
#[path = "domain_types/git_commit_link_output_ref_mut.rs"]
mod git_commit_link_output_ref_mut;
use git_commit_link_output_ref_mut::GitCommitLinkOutputRefMut;
#[path = "domain_types/validate_project_commit_error.rs"]
mod validate_project_commit_error;
pub use validate_project_commit_error::*;
#[path = "domain_types/project_git_info.rs"]
mod project_git_info;
pub use project_git_info::*;
#[path = "domain_types/git_commit_link_provider.rs"]
mod git_commit_link_provider;
pub use git_commit_link_provider::*;
#[path = "domain_types/git_commit_id_provider.rs"]
mod git_commit_id_provider;
pub use git_commit_id_provider::*;
#[path = "domain_types/functions.rs"]
mod functions;
pub use functions::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
