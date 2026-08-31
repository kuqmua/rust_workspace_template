#[cfg(test)]
mod tests {
    #[test]
    fn json_response_wraps_serializable_values() {
        let response =
            crate::json_response::json_response(server_admin_contract::admin_no_body::AdminNoBody);
        assert_eq!(response.get_inner().status(), http::StatusCode::OK);
    }

    #[test]
    fn page_total_accepts_non_negative_values_and_rejects_negative_values() {
        let total = crate::page_total::page_total(
            crate::admin_page_total_count::AdminPageTotalCount::from(17i64),
        )
        .expect("8d31f2a7 page_total_accepts_non_negative_values_and_rejects_negative_values invariant must hold");
        assert_eq!(u64::from(total), 17u64);
        assert!(matches!(
            crate::page_total::page_total(
                crate::admin_page_total_count::AdminPageTotalCount::from(-constants_i64::ONE)
            ),
            Err(crate::admin_error::AdminError::Validation)
        ));
    }

    #[test]
    fn table_sort_validation_accepts_empty_and_known_keys_only() {
        crate::validate_table_sort::validate_table_sort(
            &server_admin_contract::admin_table_query::AdminTableQuery::default(),
            &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
        )
        .expect(
            "41d8a6c2 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        let known = serde_json::from_value::<
            server_admin_contract::admin_table_query::AdminTableQuery,
        >(serde_json::json!({ "sort": "login" }))
        .expect(
            "f20a91c6 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        crate::validate_table_sort::validate_table_sort(
            &known,
            &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
        )
        .expect(
            "b70c35e9 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        let unknown = serde_json::from_value::<
            server_admin_contract::admin_table_query::AdminTableQuery,
        >(serde_json::json!({ "sort": "created_at" }))
        .expect(
            "c731d84e table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        assert!(matches!(
            crate::validate_table_sort::validate_table_sort(
                &unknown,
                &server_admin_contract::admin_table_sort_field::AdminTableSortField::USER,
            ),
            Err(crate::admin_error::AdminError::Validation)
        ));
    }

    #[test]
    fn invalid_repository_values_map_to_validation_errors() {
        assert!(matches!(
            crate::map_repository_error::map_repository_error(
                crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            ),
            crate::admin_error::AdminError::Validation
        ));
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod authorize_custom {}
pub(crate) mod json_response {}
pub(crate) mod map_repository_error {}
pub(crate) mod map_unique_violation {}
pub(crate) mod page_total {}
pub(crate) mod validate_table_sort {}
