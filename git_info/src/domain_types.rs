#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]
pub(crate) use crate::base_git_commit_link_len::BASE_GIT_COMMIT_LINK_LEN;
pub use crate::build_git_commit_link::*;
pub use crate::build_git_commit_link_cow::*;
pub use crate::check_is_project_commit::*;
pub use crate::git_commit_id::*;
pub use crate::git_commit_id_cow::*;
pub use crate::git_commit_id_fallback::*;
pub use crate::git_commit_id_provider::*;
pub use crate::git_commit_id_ref::*;
pub use crate::git_commit_link::*;
pub use crate::git_commit_link_capacity::*;
pub use crate::git_commit_link_capacity_value::*;
pub use crate::git_commit_link_cow::*;
pub(crate) use crate::git_commit_link_output_ref_mut::GitCommitLinkOutputRefMut;
pub use crate::git_commit_link_provider::*;
pub(crate) use crate::git_info_string_max_len::GIT_INFO_STRING_MAX_LEN;
pub use crate::git_info_string_try_from_string_error::*;
pub use crate::is_project_commit::*;
pub use crate::project_git_commit_link::*;
pub use crate::project_git_commit_link_ref::*;
pub use crate::project_git_commit_link_ref_value::*;
pub use crate::project_git_info::*;
pub use crate::project_git_info_value::*;
pub use crate::validate_project_commit::*;
pub use crate::validate_project_commit_error::*;
pub(crate) use crate::with_git_commit_id_ref_or::*;
pub(crate) use crate::write_git_commit_link::*;
