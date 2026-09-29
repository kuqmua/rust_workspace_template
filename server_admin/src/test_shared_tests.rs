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
}
