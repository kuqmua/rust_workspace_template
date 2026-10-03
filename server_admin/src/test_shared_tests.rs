#[cfg(test)]
mod tests {
    #[test]
    fn test_collection_validation_retains_bounded_source_and_validation_status() {
        let bounded_value_error = bounded_types::bounded_value_error::BoundedValueError::AboveMax {
            actual: bounded_types::bounded_len::BoundedLen::from(10_001),
            max: bounded_types::bounded_len::BoundedLen::from(10_000),
        };
        let admin_error = crate::admin_error::AdminError::validation_collection(
            server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(
                bounded_value_error,
            ),
        );
        assert!(std::error::Error::source(&admin_error).is_some());
        assert!(matches!(
            admin_error,
            crate::admin_error::AdminError::ValidationCollection(_)
        ));
        if let crate::admin_error::AdminError::ValidationCollection(observed) = &admin_error {
            assert!(std::error::Error::source(observed.source_ref()).is_some());
        }
        assert_eq!(
            axum::response::IntoResponse::into_response(admin_error).status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
    }

    #[test]
    fn test_collection_validation_source_survives_typed_operation_conversion() {
        let admin_error = crate::admin_error::AdminError::validation_collection(
            server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(
                bounded_types::bounded_value_error::BoundedValueError::AboveMax {
                    actual: bounded_types::bounded_len::BoundedLen::from(10_001),
                    max: bounded_types::bounded_len::BoundedLen::from(10_000),
                },
            ),
        );
        let operation_error = crate::application_auth::AdminCreateRolesError::from(admin_error);
        assert!(std::error::Error::source(&operation_error).is_some());
        assert_eq!(
            axum::response::IntoResponse::into_response(operation_error).status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
    }

    #[test]
    fn test_json_response_wraps_serializable_values() {
        let response =
            crate::json_response::json_response(server_admin_contract::admin_no_body::AdminNoBody);
        assert_eq!(response.get_inner().status(), http::StatusCode::OK);
    }

    #[test]
    fn test_page_total_accepts_non_negative_values_and_rejects_negative_values() {
        let total = crate::page_total::page_total(
            crate::admin_page_total_count::AdminPageTotalCount::from(17i64),
        )
        .expect(constants_str::DIAGNOSTIC_8D31F2A7);
        assert_eq!(u64::from(total), 17u64);
        assert!(matches!(
            crate::page_total::page_total(
                crate::admin_page_total_count::AdminPageTotalCount::from(-constants_i64::ONE)
            ),
            Err(crate::admin_error::AdminError::Validation)
        ));
    }

    #[test]
    fn test_table_sort_validation_accepts_empty_and_known_keys_only() {
        crate::validate_table_sort::validate_table_sort(
            &server_admin_contract::admin_table_query::AdminTableQuery::default(),
            &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
        )
        .expect(constants_str::DIAGNOSTIC_41D8A6C2);
        let known = serde_json::from_value::<
            server_admin_contract::admin_table_query::AdminTableQuery,
        >(serde_json::json!({ "sort": "login" }))
        .expect(constants_str::DIAGNOSTIC_F20A91C6);
        crate::validate_table_sort::validate_table_sort(
            &known,
            &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
        )
        .expect(constants_str::DIAGNOSTIC_B70C35E9);
        let unknown = serde_json::from_value::<
            server_admin_contract::admin_table_query::AdminTableQuery,
        >(serde_json::json!({ "sort": "created_at" }))
        .expect(constants_str::DIAGNOSTIC_C731D84E);
        assert!(matches!(
            crate::validate_table_sort::validate_table_sort(
                &unknown,
                &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
            ),
            Err(crate::admin_error::AdminError::Validation)
        ));
    }

    #[test]
    fn test_invalid_repository_values_map_to_validation_errors() {
        assert!(matches!(
            crate::map_repository_error::map_repository_error(
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            ),
            crate::admin_error::AdminError::Validation
        ));
    }
    #[test]
    fn test_plain_admin_error_responses_have_problem_content_type_without_diagnostics() {
        let cases = [
            (
                crate::admin_error::AdminError::Authentication,
                http::StatusCode::UNAUTHORIZED,
            ),
            (
                crate::admin_error::AdminError::Authorization,
                http::StatusCode::FORBIDDEN,
            ),
            (
                crate::admin_error::AdminError::Csrf,
                http::StatusCode::FORBIDDEN,
            ),
            (
                crate::admin_error::AdminError::Conflict,
                http::StatusCode::CONFLICT,
            ),
            (
                crate::admin_error::AdminError::MethodNotAllowed,
                http::StatusCode::METHOD_NOT_ALLOWED,
            ),
            (
                crate::admin_error::AdminError::PayloadTooLarge,
                http::StatusCode::PAYLOAD_TOO_LARGE,
            ),
            (
                crate::admin_error::AdminError::RateLimited,
                http::StatusCode::TOO_MANY_REQUESTS,
            ),
            (
                crate::admin_error::AdminError::Validation,
                http::StatusCode::UNPROCESSABLE_ENTITY,
            ),
        ];
        assert!(cases.into_iter().all(|(error, expected)| {
            let response = axum::response::IntoResponse::into_response(error);
            response.status() == expected
                && response
                    .headers()
                    .get(http::header::CONTENT_TYPE)
                    .is_some_and(|content_type| {
                        content_type == constants_str::APPLICATION_PROBLEM_PLUS_JSON
                    })
                && response
                    .extensions()
                    .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                    .is_none()
        }));
    }

    #[test]
    fn test_database_admin_error_response_carries_internal_diagnostic() {
        let response = axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::from(sqlx::Error::RowNotFound),
        );
        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        assert!(
            response
                .extensions()
                .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                .is_some_and(|diagnostic| diagnostic.telemetry().error_code().to_string()
                    == constants_str::ADMIN_OBSERVED_ERROR_DATABASE)
        );
        assert!(
            response
                .headers()
                .get(http::header::CONTENT_TYPE)
                .is_some_and(
                    |content_type| content_type == constants_str::APPLICATION_PROBLEM_PLUS_JSON
                )
        );
    }
    #[test]
    fn test_secret_error_responses_keep_operation_diagnostics() {
        let source = crate::admin_secret_text_error::AdminSecretTextError::ContainsNul;
        let cases = [
            (
                crate::admin_error::AdminError::authentication_secret_text(source),
                http::StatusCode::UNAUTHORIZED,
                constants_str::ADMIN_OBSERVED_ERROR_AUTH_SECRET_TEXT,
            ),
            (
                crate::admin_error::AdminError::csrf_secret_text(source),
                http::StatusCode::FORBIDDEN,
                constants_str::ADMIN_OBSERVED_ERROR_CSRF_SECRET_TEXT,
            ),
            (
                crate::admin_error::AdminError::secret_text(source),
                http::StatusCode::UNPROCESSABLE_ENTITY,
                constants_str::ADMIN_OBSERVED_ERROR_SECRET_TEXT,
            ),
        ];
        assert!(cases.into_iter().all(|(error, status, code)| {
            let response = axum::response::IntoResponse::into_response(error);
            response.status() == status
                && response
                    .extensions()
                    .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                    .is_some_and(|diagnostic| {
                        diagnostic.telemetry().error_code().to_string() == code
                    })
        }));
    }
    #[test]
    fn test_observed_admin_response_codes_follow_error_categories() {
        let cases = [
            (crate::admin_error::AdminError::password_text(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength), http::StatusCode::UNPROCESSABLE_ENTITY, constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_TEXT),
            (crate::admin_error::AdminError::validation_collection(server_admin_contract::admin_collection_error::AdminCollectionError::TooLong(bounded_types::bounded_value_error::BoundedValueError::AboveMax { actual: 2usize.into(), max: 1usize.into() })), http::StatusCode::UNPROCESSABLE_ENTITY, constants_str::ADMIN_OBSERVED_ERROR_COLLECTION),
            (crate::admin_error::AdminError::session(crate::admin_session_error::AdminSessionError::SystemClock), http::StatusCode::INTERNAL_SERVER_ERROR, constants_str::ADMIN_OBSERVED_ERROR_SESSION),
            (crate::admin_error::AdminError::password_hash(crate::admin_password_hash_error::AdminPasswordHashError::BoundedText(bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { actual_length: 2usize.into(), maximum_length: 1usize.into() })), http::StatusCode::INTERNAL_SERVER_ERROR, constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_HASH),
        ];
        assert!(cases.into_iter().all(|(error, status, code)| {
            let response = axum::response::IntoResponse::into_response(error);
            response.status() == status
                && response
                    .extensions()
                    .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                    .is_some_and(|diagnostic| {
                        diagnostic.telemetry().error_code().to_string() == code
                    })
        }));
        assert!(
            http::HeaderValue::from_bytes(constants_str::NEWLINE.as_bytes()).is_err_and(|source| {
                let response = axum::response::IntoResponse::into_response(
                    crate::admin_error::AdminError::header(
                        crate::http_admin_header_value_error::HttpAdminHeaderValueError::from(
                            source,
                        ),
                    ),
                );
                response.status() == http::StatusCode::INTERNAL_SERVER_ERROR
                    && response
                        .extensions()
                        .get::<server_runtime_http::http_error_diagnostic::HttpErrorDiagnostic>()
                        .is_some_and(|diagnostic| {
                            diagnostic.telemetry().error_code().to_string()
                                == constants_str::ADMIN_OBSERVED_ERROR_RESPONSE_HEADER
                        })
            })
        );
    }
    #[test]
    fn test_repository_database_error_preserves_source_and_diagnostic() {
        let error = crate::map_repository_error::map_repository_error(
            crate::admin_repository_error::AdminRepositoryError::from(
                crate::sqlx_admin_error::SqlxAdminError::from(sqlx::Error::RowNotFound),
            ),
        );
        assert!(
            matches!(error, crate::admin_error::AdminError::Pg(observed) if matches!(observed.source_ref().get_inner(), sqlx::Error::RowNotFound) && observed.error_code() == server_observability::observed_error_code::ObservedErrorCode::from(constants_str::ADMIN_OBSERVED_ERROR_DATABASE))
        );
    }
}
