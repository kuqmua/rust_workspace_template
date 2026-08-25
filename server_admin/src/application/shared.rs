#![allow(clippy::single_call_fn)] // route facade preserves utoipa inventory while private implementations own handler logic
pub(super) fn map_unique_violation<Error>(value: Error) -> super::AdminError
where
    Error: Into<sqlx::Error>,
{
    let error = value.into();
    if pg_crud_common::domain_types::classify_pg_error(
        pg_crud_common::domain_types::SqlxPgErrorRef::from(&error),
    ) == pg_crud_common::domain_types::PgErrorKind::UniqueViolation
    {
        super::AdminError::Conflict
    } else {
        super::AdminError::from(error)
    }
}
pub(super) fn map_repository_error(
    repository_error: crate::adapters::repository::AdminRepositoryError,
) -> super::AdminError {
    match repository_error {
        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        crate::adapters::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::from(sqlx_error)
        }
    }
}
pub(super) fn json_response<Value>(value: Value) -> super::AxumAdminResponse
where
    Value: serde::Serialize,
{
    super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
        value,
    )))
}
pub(super) fn page_total(
    value: crate::adapters::repository::AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, super::AdminError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| super::AdminError::Validation)
}
pub(super) fn validate_table_sort(
    query: &server_admin_contract::domain_types::AdminTableQuery,
    options: &[server_admin_contract::domain_types::AdminTableSortField],
) -> Result<(), super::AdminError> {
    if query.sort().as_ref().is_empty() {
        return Ok(());
    }
    server_admin_contract::domain_types::AdminTableSortField::try_from_key(
        options,
        server_admin_contract::domain_types::AdminTableSortKeyRef::from(query.sort().as_ref()),
    )
    .map(drop)
    .map_err(|_error| super::AdminError::Validation)
}
pub(super) async fn authorize_custom(
    auth: &super::AdminAuthReq,
    permission: super::super::AdminPermission,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let authenticated = super::authorization::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        super::super::StdAdminBool::from(true),
    )
    .await?;
    Ok(authenticated)
}

#[cfg(test)]
mod tests {
    #[test]
    fn json_response_wraps_serializable_values() {
        let response = super::json_response(server_admin_contract::domain_types::AdminNoBody);
        assert_eq!(response.0.status(), http::StatusCode::OK);
    }

    #[test]
    fn page_total_accepts_non_negative_values_and_rejects_negative_values() {
        let total = super::page_total(crate::adapters::repository::AdminPageTotalCount::from(
            17i64,
        ))
        .expect("8d31f2a7 page_total_accepts_non_negative_values_and_rejects_negative_values invariant must hold");
        assert_eq!(u64::from(total), 17u64);
        assert!(matches!(
            super::page_total(crate::adapters::repository::AdminPageTotalCount::from(
                -constants_i64::ONE
            )),
            Err(super::super::AdminError::Validation)
        ));
    }

    #[test]
    fn table_sort_validation_accepts_empty_and_known_keys_only() {
        super::validate_table_sort(
            &server_admin_contract::domain_types::AdminTableQuery::default(),
            &server_admin_contract::domain_types::AdminTableSortField::USER,
        )
        .expect(
            "41d8a6c2 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        let known = serde_json::from_value::<server_admin_contract::domain_types::AdminTableQuery>(
            serde_json::json!({ "sort": "login" }),
        )
        .expect(
            "f20a91c6 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        super::validate_table_sort(
            &known,
            &server_admin_contract::domain_types::AdminTableSortField::USER,
        )
        .expect(
            "b70c35e9 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        let unknown = serde_json::from_value::<server_admin_contract::domain_types::AdminTableQuery>(
            serde_json::json!({ "sort": "created_at" }),
        )
        .expect(
            "c731d84e table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        assert!(matches!(
            super::validate_table_sort(
                &unknown,
                &server_admin_contract::domain_types::AdminTableSortField::USER,
            ),
            Err(super::super::AdminError::Validation)
        ));
    }

    #[test]
    fn invalid_repository_values_map_to_validation_errors() {
        assert!(matches!(
            super::map_repository_error(
                crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            ),
            super::super::AdminError::Validation
        ));
    }
}
