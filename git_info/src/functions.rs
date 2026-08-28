#[path = "with_git_commit_id_ref_or.rs"]
mod with_git_commit_id_ref_or;
pub(super) use with_git_commit_id_ref_or::*;
#[path = "project_git_info_value.rs"]
mod project_git_info_value;
pub use project_git_info_value::*;
#[path = "check_is_project_commit.rs"]
mod check_is_project_commit;
pub use check_is_project_commit::*;
#[path = "validate_project_commit.rs"]
mod validate_project_commit;
pub use validate_project_commit::*;
#[path = "project_git_commit_link.rs"]
mod project_git_commit_link;
pub use project_git_commit_link::*;
#[path = "project_git_commit_link_ref_value.rs"]
mod project_git_commit_link_ref_value;
pub use project_git_commit_link_ref_value::*;
#[path = "build_git_commit_link.rs"]
mod build_git_commit_link;
pub use build_git_commit_link::*;
#[path = "build_git_commit_link_cow.rs"]
mod build_git_commit_link_cow;
pub use build_git_commit_link_cow::*;
#[path = "write_git_commit_link.rs"]
mod write_git_commit_link;
pub(super) use write_git_commit_link::*;
#[path = "git_commit_link_capacity_value.rs"]
mod git_commit_link_capacity_value;
pub use git_commit_link_capacity_value::*;
