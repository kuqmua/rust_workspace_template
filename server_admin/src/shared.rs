#[path = "authorize_custom.rs"]
pub(in crate::domain_types::auth) mod authorize_custom;
#[path = "json_response.rs"]
pub(in crate::domain_types::auth) mod json_response;
#[path = "map_repository_error.rs"]
pub(in crate::domain_types::auth) mod map_repository_error;
#[path = "map_unique_violation.rs"]
pub(in crate::domain_types::auth) mod map_unique_violation;
#[path = "page_total.rs"]
pub(in crate::domain_types::auth) mod page_total;
#[path = "validate_table_sort.rs"]
pub(in crate::domain_types::auth) mod validate_table_sort;

#[cfg(test)]
mod tests {
    #[test]
    fn json_response_wraps_serializable_values() {
        let response =
            super::json_response::json_response(server_admin_contract::domain_types::AdminNoBody);
        assert_eq!(response.0.status(), http::StatusCode::OK);
    }

    #[test]
    fn page_total_accepts_non_negative_values_and_rejects_negative_values() {
        let total = super::page_total::page_total(
            crate::adapters::repository::AdminPageTotalCount::from(17i64),
        )
        .expect("8d31f2a7 page_total_accepts_non_negative_values_and_rejects_negative_values invariant must hold");
        assert_eq!(u64::from(total), 17u64);
        assert!(matches!(
            super::page_total::page_total(crate::adapters::repository::AdminPageTotalCount::from(
                -constants_i64::ONE
            )),
            Err(super::super::AdminError::Validation)
        ));
    }

    #[test]
    fn table_sort_validation_accepts_empty_and_known_keys_only() {
        super::validate_table_sort::validate_table_sort(
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
        super::validate_table_sort::validate_table_sort(
            &known,
            &server_admin_contract::domain_types::AdminTableSortField::USER,
        )
        .expect(
            "b70c35e9 table_sort_validation_accepts_empty_and_known_keys_only invariant must hold",
        );
        let unknown = serde_json::from_value::<server_admin_contract::domain_types::AdminTableQuery>(
            serde_json::json!({ "sort": "created_at" }),
        )
        .expect("c731d84e table_sort_validation_accepts_empty_and_known_keys_only invariant must hold");
        assert!(matches!(
            super::validate_table_sort::validate_table_sort(
                &unknown,
                &server_admin_contract::domain_types::AdminTableSortField::USER,
            ),
            Err(super::super::AdminError::Validation)
        ));
    }

    #[test]
    fn invalid_repository_values_map_to_validation_errors() {
        assert!(matches!(
            super::map_repository_error::map_repository_error(
                crate::adapters::repository::AdminRepositoryError::InvalidStoredValue,
            ),
            super::super::AdminError::Validation
        ));
    }
}
