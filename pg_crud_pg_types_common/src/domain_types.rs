pub use crate::is_primary_key::IsPrimaryKey;
pub use crate::maybe_primary_key::maybe_primary_key;
pub use crate::pagination_starts_with_one::PaginationStartsWithOne;
pub use crate::pagination_starts_with_one_try_new_error::PaginationStartsWithOneTryNewError;
pub use crate::pagination_starts_with_one_value::PaginationStartsWithOneValue;
#[cfg(test)]
mod tests {
    #[test]
    fn pagination_starts_with_one_accepts_inclusive_boundaries() {
        let pagination = super::PaginationStartsWithOne::try_new(2i64, constants_i64::ONE).expect(
            "007c805e pagination_starts_with_one_accepts_inclusive_boundaries invariant must hold",
        );
        assert_eq!(pagination.start().get(), constants_i64::ONE);
        assert_eq!(pagination.end().get(), 3i64);
    }

    #[test]
    fn pagination_starts_with_one_distinguishes_validation_errors() {
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ZERO, constants_i64::ONE),
            Err(super::PaginationStartsWithOneTryNewError::LimitIsLessThanOrEqToZero { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, constants_i64::ZERO),
            Err(super::PaginationStartsWithOneTryNewError::OffsetIsLessThanOne { .. })
        ));
        assert!(matches!(
            super::PaginationStartsWithOne::try_new(constants_i64::ONE, i64::MAX),
            Err(super::PaginationStartsWithOneTryNewError::OffsetPlusLimitIsIntOverflow { .. })
        ));
    }

    #[test]
    fn pagination_defaults_start_at_one_and_use_the_expected_limits() {
        let standard =
            <super::PaginationStartsWithOne as pg_crud_common::domain_types::DefaultSomeOneElement>::default_some_one_element();
        assert_eq!(standard.start().get(), constants_i64::ONE);
        assert_eq!(
            standard.end().get(),
            pg_crud_common::domain_types::PaginationPolicy::standard()
                .default_limit()
                .get()
                + constants_i64::ONE
        );
        let maximum =
            <super::PaginationStartsWithOne as pg_crud_common::domain_types::DefaultSomeOneElementMaxPageSize>::default_some_one_element_max_page_size();
        assert_eq!(maximum.start().get(), constants_i64::ONE);
        assert_eq!(maximum.end().get(), i64::from(i32::MAX));
    }

    #[test]
    fn primary_key_suffix_matches_the_typed_flag() {
        assert_eq!(
            super::maybe_primary_key(super::IsPrimaryKey::from(true)).to_string(),
            constants_str::PRIMARY_KEY
        );
        assert_eq!(
            super::maybe_primary_key(super::IsPrimaryKey::from(false)).to_string(),
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
        );
        assert_eq!(
            super::maybe_primary_key(pg_crud_common::domain_types::IsPrimaryKey::from(true))
                .to_string(),
            constants_str::PRIMARY_KEY
        );
    }
}
