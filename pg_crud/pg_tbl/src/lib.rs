pub trait CombinationOfAppStateLogicTraits:
    app_state::GetEnableApiGitCommitCheck
    + app_state::GetMaximumSizeOfHttpBodyInBytes
    + app_state::GetPgPool
    + app_state::GetSrcPlaceType
    + app_state::GetTimezone
    + Send
    + Sync
{
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgTableLeaf(pg_crud_cmn::JsonFieldRights);
