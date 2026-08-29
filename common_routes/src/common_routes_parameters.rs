pub trait CommonRoutesParameters:
    git_info::git_commit_link_provider::GitCommitLinkProvider
    + app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider
    + Send
    + Sync
{
}
