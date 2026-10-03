#[test]
fn test_payload_example_error_variants_preserve_response_and_source_contracts() {
    fn assert_payload_example_error_contract<PayloadExampleError>(
        payload_example_error: PayloadExampleError,
        std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
        observed_error_code: server_observability::observed_error_code::ObservedErrorCode,
        std_admin_bool: server_admin_core::std_admin_bool::StdAdminBool,
    ) where
        PayloadExampleError: std::error::Error + axum::response::IntoResponse,
    {
        assert_eq!(payload_example_error.to_string(), std_admin_str_ref.get());
        assert_eq!(
            payload_example_error.source().is_some(),
            std_admin_bool.get()
        );
        if let Some(source) = payload_example_error.source() {
            let identifier = source
                .downcast_ref::<server_observability::observed_error::ObservedError<
                    server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
                >>();
            let collection = source
                .downcast_ref::<server_observability::observed_error::ObservedError<
                    server_admin_contract::admin_collection_error::AdminCollectionError,
                >>();
            assert!(identifier.is_some_and(|observed_error| observed_error.source_ref() == &server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid)
                || collection.is_some_and(|observed_error| matches!(observed_error.source_ref(), server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(_))));
        }
        let response = axum::response::IntoResponse::into_response(payload_example_error);
        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        let diagnostic = response
            .extensions()
            .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>();
        assert!(diagnostic.is_some());
        if let Some(diagnostic) = diagnostic {
            assert_eq!(
                diagnostic.telemetry().error_type().to_string(),
                constants_str::ADMIN_API_ERROR_TYPE
            );
            assert_eq!(
                diagnostic.telemetry().error_code().to_string(),
                observed_error_code.get()
            );
        }
    }
    let collection_source = || {
        server_observability::observed_error::ObservedError::capture(
            server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(
                bounded_types::bounded_value_error::BoundedValueError::AboveMax {
                    actual: bounded_types::bounded_len::BoundedLen::from(10_001usize),
                    max: bounded_types::bounded_len::BoundedLen::from(10_000usize),
                },
            ),
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
            ),
        )
    };
    let identifier_source = || {
        server_observability::observed_error::ObservedError::capture(
            server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid,
            server_observability::observed_error_code::ObservedErrorCode::from(
                constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
            ),
        )
    };
    assert_payload_example_error_contract(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::Collection(collection_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_COLLECTION_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_CREATE_USER_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::DisplayName(server_admin_contract::admin_display_name::AdminDisplayNameTryFromStringError::InvalidBounds { min: 2usize, max: 1usize }),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_DISPLAY_NAME_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_CREATE_USER_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    );
    assert_payload_example_error_contract(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::Login(
            server_admin_contract::admin_login::AdminLoginTryFromStringError::InvalidBounds {
                min: 2usize,
                max: 1usize,
            },
        ),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_LOGIN_ERROR,
        ),
        server_observability::observed_error_code::ObservedErrorCode::from(
            constants_str::ADMIN_OBSERVED_ERROR_CREATE_USER_PAYLOAD_EXAMPLE,
        ),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    );
    assert_payload_example_error_contract(
        crate::admin_create_user_payload_example_error::AdminCreateUserPayloadExampleError::Password(server_admin_contract::admin_new_password::AdminNewPasswordTryFromStringError::InvalidBounds { min: 2usize, max: 1usize }),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_CREATE_USER_PAYLOAD_EXAMPLE_PASSWORD_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_CREATE_USER_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    );
    assert_payload_example_error_contract(
        crate::admin_update_users_payload_example_error::AdminUpdateUsersPayloadExampleError::Collection(collection_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_UPDATE_USERS_PAYLOAD_EXAMPLE_COLLECTION_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_UPDATE_USERS_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_update_users_payload_example_error::AdminUpdateUsersPayloadExampleError::UserIdentifier(identifier_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_UPDATE_USERS_PAYLOAD_EXAMPLE_USER_IDENTIFIER_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_UPDATE_USERS_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_create_roles_payload_example_error::AdminCreateRolesPayloadExampleError::Collection(collection_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_CREATE_ROLES_PAYLOAD_EXAMPLE_COLLECTION_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_CREATE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_create_roles_payload_example_error::AdminCreateRolesPayloadExampleError::Name(server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError::InvalidBounds { min: 2usize, max: 1usize }),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_CREATE_ROLES_PAYLOAD_EXAMPLE_NAME_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_CREATE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    );
    assert_payload_example_error_contract(
        crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::Collection(collection_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_UPDATE_ROLES_PAYLOAD_EXAMPLE_COLLECTION_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_UPDATE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::Name(server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError::InvalidBounds { min: 2usize, max: 1usize }),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_UPDATE_ROLES_PAYLOAD_EXAMPLE_NAME_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_UPDATE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    );
    assert_payload_example_error_contract(
        crate::admin_update_roles_payload_example_error::AdminUpdateRolesPayloadExampleError::RoleIdentifier(identifier_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_UPDATE_ROLES_PAYLOAD_EXAMPLE_ROLE_IDENTIFIER_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_UPDATE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_delete_roles_payload_example_error::AdminDeleteRolesPayloadExampleError::RoleIdentifier(identifier_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_DELETE_ROLES_PAYLOAD_EXAMPLE_ROLE_IDENTIFIER_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_DELETE_ROLES_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
    assert_payload_example_error_contract(
        crate::admin_delete_users_payload_example_error::AdminDeleteUsersPayloadExampleError::UserIdentifier(identifier_source()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(constants_str::ADMIN_DELETE_USERS_PAYLOAD_EXAMPLE_USER_IDENTIFIER_ERROR),
        server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_DELETE_USERS_PAYLOAD_EXAMPLE),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    );
}
