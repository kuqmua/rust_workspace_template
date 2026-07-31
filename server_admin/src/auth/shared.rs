#![allow(clippy::single_call_fn)] // route facade preserves utoipa inventory while private implementations own handler logic
pub(super) fn map_unique_violation<Error>(value: Error) -> super::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::classify_pg_error(pg_crud_common::SqlxPgErrorRef::from(&error))
        == pg_crud_common::PgErrorKind::UniqueViolation
    {
        super::AdminError::Conflict
    } else {
        super::AdminError::from(error)
    }
}
pub(super) fn map_repository_error(
    repository_error: super::super::repository::AdminRepositoryError,
) -> super::AdminError {
    match repository_error {
        super::super::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        super::super::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::from(sqlx_error)
        }
    }
}
pub(super) fn page_total(
    value: super::super::repository::AdminPageTotalCount,
) -> Result<server_admin_contract::AdminPageTotal, super::AdminError> {
    u64::try_from(value.get())
        .map(server_admin_contract::AdminPageTotal::from)
        .map_err(|_error| super::AdminError::Validation)
}
pub(super) fn validate_table_sort(
    query: &server_admin_contract::AdminTableQuery,
    options: &[server_admin_contract::AdminTableSortField],
) -> Result<(), super::AdminError> {
    if query.sort().as_ref().is_empty() {
        return Ok(());
    }
    server_admin_contract::AdminTableSortField::try_from_key(
        options,
        server_admin_contract::AdminTableSortKeyRef::from(query.sort().as_ref()),
    )
    .map(drop)
    .map_err(|_error| super::AdminError::Validation)
}
pub(super) async fn authenticate_mutation(
    auth: &super::AdminAuthReq,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let actor = super::authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await?;
    let subject = super::super::StdAdminString::try_from(actor.id.get().to_string())
        .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        super::rate_limit::AdminRateLimitScope::Mutation,
        &subject,
        auth.state.as_ref().policy.mutation_limit,
        auth.state.as_ref().policy.mutation_window,
    )
    .await?;
    super::validate_csrf(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &actor,
    )
    .await?;
    Ok(actor)
}
