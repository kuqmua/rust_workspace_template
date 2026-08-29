#[cfg(test)]
mod tests {
    #[test]
    fn json_response_wraps_serializable_values() {
        let response =
            crate::json_response::json_response(server_admin_contract::domain_types::AdminNoBody);
        assert_eq!(response.0.status(), http::StatusCode::OK);
    }

    #[test]
    fn page_total_accepts_non_negative_values_and_rejects_negative_values() {
        let total = crate::page_total::page_total(
            crate::repository::AdminPageTotalCount::from(17i64),
        )
        .expect("8d31f2a7 page_total_accepts_non_negative_values_and_rejects_negative_values invariant must hold");
        assert_eq!(u64::from(total), 17u64);
        assert!(matches!(
            crate::page_total::page_total(crate::repository::AdminPageTotalCount::from(
                -constants_i64::ONE
            )),
            Err(crate::AdminError::Validation)
        ));
    }

    #[test]
    fn table_sort_validation_accepts_empty_and_known_keys_only() {
        crate::validate_table_sort::validate_table_sort(
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
        crate::validate_table_sort::validate_table_sort(
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
            crate::validate_table_sort::validate_table_sort(
                &unknown,
                &server_admin_contract::domain_types::AdminTableSortField::USER,
            ),
            Err(crate::AdminError::Validation)
        ));
    }

    #[test]
    fn invalid_repository_values_map_to_validation_errors() {
        assert!(matches!(
            crate::map_repository_error::map_repository_error(
                crate::repository::AdminRepositoryError::InvalidStoredValue,
            ),
            crate::AdminError::Validation
        ));
    }
}

// Root-owned module compatibility wrappers.
pub(crate) mod authorize_custom {
    pub use super::super::authorize_custom::*;
}
pub(crate) mod json_response {
    pub use super::super::json_response::*;
}
pub(crate) mod map_repository_error {
    pub use super::super::map_repository_error::*;
}
pub(crate) mod map_unique_violation {
    pub use super::super::map_unique_violation::*;
}
pub(crate) mod page_total {
    pub use super::super::page_total::*;
}
pub(crate) mod validate_table_sort {
    pub use super::super::validate_table_sort::*;
}
