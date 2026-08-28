pub trait CommonRoutesParameters:
    git_info::domain_types::GitCommitLinkProvider
    + app_state::domain_types::SqlxPgPoolProvider
    + Send
    + Sync
{
}
