#[path = "functions/with_git_commit_id_ref_or.rs"]
mod with_git_commit_id_ref_or;
pub(super) use with_git_commit_id_ref_or::*;
#[path = "functions/project_git_info.rs"]
mod project_git_info;
pub use project_git_info::*;
#[path = "functions/is_project_commit.rs"]
mod is_project_commit;
pub use is_project_commit::*;
#[path = "functions/validate_project_commit.rs"]
mod validate_project_commit;
pub use validate_project_commit::*;
#[path = "functions/project_git_commit_link.rs"]
mod project_git_commit_link;
pub use project_git_commit_link::*;
#[path = "functions/project_git_commit_link_ref.rs"]
mod project_git_commit_link_ref;
pub use project_git_commit_link_ref::*;
#[path = "functions/git_commit_link.rs"]
mod git_commit_link;
pub use git_commit_link::*;
#[path = "functions/git_commit_link_cow.rs"]
mod git_commit_link_cow;
pub use git_commit_link_cow::*;
#[path = "functions/write_git_commit_link.rs"]
mod write_git_commit_link;
pub(super) use write_git_commit_link::*;
#[path = "functions/git_commit_link_capacity.rs"]
mod git_commit_link_capacity;
pub use git_commit_link_capacity::*;
