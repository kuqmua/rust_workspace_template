#[test]
fn test_generated_read_page_error_contracts() {
    fn read_page_error_contract_matches<ReadPageError>()
    -> server_admin_core::std_admin_bool::StdAdminBool
    where
        ReadPageError: From<crate::admin_repository_error::AdminRepositoryError>
            + From<server_admin_contract::admin_text::AdminTextTryFromStringError>
            + From<server_admin_contract::admin_collection_error::AdminCollectionError>
            + From<pg_crud_common::list_total_error::ListTotalError>
            + From<server_runtime_http::serde_json_error::SerdeJsonError>
            + std::error::Error
            + to_err_string::to_err_string::ToErrString,
    {
        let stored = ReadPageError::from(
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
        );
        let source =
            server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid;
        let query = ReadPageError::from(crate::admin_repository_error::AdminRepositoryError::Sqlx(
            crate::sqlx_admin_error::SqlxAdminError::from(source),
        ));
        let text = ReadPageError::from(
            server_admin_contract::admin_text::AdminTextTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
        );
        let collection_source =
            server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(
                bounded_types::bounded_value_error::BoundedValueError::AboveMax {
                    actual: bounded_types::bounded_len::BoundedLen::from(10_001usize),
                    max: bounded_types::bounded_len::BoundedLen::from(10_000usize),
                },
            );
        let collection = ReadPageError::from(collection_source);
        let total_source = pg_crud_common::list_total_error::ListTotalError::Negative;
        let total = ReadPageError::from(total_source);
        let serialization_source_result = serde_json::from_str::<
            server_admin_contract::admin_bool::AdminBool,
        >(constants_str::EMPTY);
        let Err(serialization_source) = serialization_source_result else {
            return server_admin_core::std_admin_bool::StdAdminBool::from(false);
        };
        let serialization = ReadPageError::from(
            server_runtime_http::serde_json_error::SerdeJsonError::from(serialization_source),
        );
        server_admin_core::std_admin_bool::StdAdminBool::from(
            stored.to_string() == constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT
                && stored.source().is_none()
                && to_err_string::to_err_string::ToErrString::to_err_string(&stored).as_ref() == stored.to_string()
                && query.to_string() == constants_str::ADMIN_DIAGNOSTIC_ADMIN_REPOSITORY_QUERY_FAILED
                && query.source().and_then(|error| error.downcast_ref::<crate::sqlx_admin_error::SqlxAdminError>())
                    .is_some_and(|error| matches!(error.get_inner(), sqlx::Error::Decode(inner)
                        if inner.downcast_ref::<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>() == Some(&source)))
                && to_err_string::to_err_string::ToErrString::to_err_string(&query).as_ref() == query.to_string()
                && collection.source().and_then(|error| error.downcast_ref::<server_admin_contract::admin_collection_error::AdminCollectionError>()) == Some(&collection_source)
                && total.source().and_then(|error| error.downcast_ref::<pg_crud_common::list_total_error::ListTotalError>()) == Some(&total_source)
                && serialization.source().is_some_and(|error| error.downcast_ref::<server_runtime_http::serde_json_error::SerdeJsonError>().is_some())
                && [&collection, &total, &serialization].into_iter().all(|error|
                    error.to_string() == constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT
                        && to_err_string::to_err_string::ToErrString::to_err_string(error).as_ref() == error.to_string()
                )
                && text.to_string() == constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT
                && to_err_string::to_err_string::ToErrString::to_err_string(&text).as_ref() == text.to_string()
        )
    }
    assert!(
        read_page_error_contract_matches::<
            crate::admin_access_sessions_read_page_error::AdminAccessSessionsReadPageError,
        >()
        .get()
    );
    assert!(
        read_page_error_contract_matches::<
            crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError,
        >()
        .get()
    );
    assert!(
        read_page_error_contract_matches::<
            crate::admin_permission_actions_read_page_error::AdminPermissionActionsReadPageError,
        >()
        .get()
    );
    assert!(read_page_error_contract_matches::<crate::admin_permission_resource_actions_read_page_error::AdminPermissionResourceActionsReadPageError>().get());
    assert!(read_page_error_contract_matches::<crate::admin_permission_resources_read_page_error::AdminPermissionResourcesReadPageError>().get());
    assert!(
        read_page_error_contract_matches::<
            crate::admin_role_rules_read_page_error::AdminRoleRulesReadPageError,
        >()
        .get()
    );
    assert!(
        read_page_error_contract_matches::<
            crate::admin_rules_read_page_error::AdminRulesReadPageError,
        >()
        .get()
    );
    assert!(
        read_page_error_contract_matches::<
            crate::admin_system_settings_read_page_error::AdminSystemSettingsReadPageError,
        >()
        .get()
    );
    assert!(
        read_page_error_contract_matches::<
            crate::admin_users_database_read_page_error::AdminUsersDatabaseReadPageError,
        >()
        .get()
    );
    let total_source = pg_crud_common::list_total_error::ListTotalError::Negative;
    let role_error =
        crate::admin_roles_read_page_error::AdminRolesReadPageError::from(total_source);
    assert_eq!(
        role_error.to_string(),
        constants_str::ADMIN_DIAGNOSTIC_STORED_ADMIN_VALUE_DOES_NOT_SATISFY_ITS_CONTRACT
    );
    assert_eq!(
        std::error::Error::source(&role_error).and_then(|error| {
            error.downcast_ref::<pg_crud_common::list_total_error::ListTotalError>()
        }),
        Some(&total_source),
    );
    assert_eq!(
        to_err_string::to_err_string::ToErrString::to_err_string(&role_error).as_ref(),
        role_error.to_string()
    );
}
