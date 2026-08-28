pub trait CommonRoutesParameters:
    git_info::GitCommitLinkProvider + app_state::SqlxPgPoolProvider + Send + Sync
{
}
