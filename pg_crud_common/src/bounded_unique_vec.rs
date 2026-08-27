#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "bounded_unique_vec/bounded_unique_vec.rs"]
mod bounded_unique_vec;
#[path = "bounded_unique_vec/bounded_unique_vec_visitor_phantom_data.rs"]
mod bounded_unique_vec_visitor_phantom_data;
#[path = "bounded_unique_vec/serde_prealloc_max_items.rs"]
mod serde_prealloc_max_items;
#[path = "bounded_unique_vec/unique_vec_error.rs"]
mod unique_vec_error;
#[path = "bounded_unique_vec/unique_vec_len.rs"]
mod unique_vec_len;

pub use bounded_unique_vec::BoundedUniqueVec;
pub use unique_vec_error::UniqueVecError;
pub use unique_vec_len::UniqueVecLen;

#[cfg(test)]
mod tests {
    #[test]
    fn duplicate_is_rejected_before_later_invalid_item() {
        let result = serde_json::from_str::<super::BoundedUniqueVec<u8, 1, 4>>(
            constants_str::TEST_BOUNDED_UNIQUE_VEC_DUPLICATE_THEN_INVALID,
        );
        assert!(
            matches!(result, Err(error) if error.to_string().contains(constants_str::DUPLICATE))
        );
    }

    #[test]
    fn shared_bounds_map_to_existing_unique_errors() {
        assert_eq!(
            super::BoundedUniqueVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("e71d26a6"),
            super::UniqueVecError::BelowMin {
                actual: super::UniqueVecLen::from(constants_usize::ZERO),
                min: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 0, 1>::try_from(vec![1u8, 2u8]).expect_err("c98b4208"),
            super::UniqueVecError::AboveMax {
                max: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("6898eb44"),
            super::UniqueVecError::InvalidBounds {
                min: super::UniqueVecLen::from(2usize),
                max: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 0, 2>::try_from(vec![1u8, 1u8]).expect_err("dc0f5d9f"),
            super::UniqueVecError::Duplicate
        );
    }

    #[test]
    fn excess_item_is_ignored_without_deserializing_target_type() {
        let error = serde_json::from_str::<super::BoundedUniqueVec<u8, 0, 1>>(
            constants_str::TEST_BOUNDED_UNIQUE_VEC_EXCESS_INVALID,
        )
        .expect_err(constants_str::VALUE_A37B95DF);
        assert!(
            error
                .to_string()
                .contains(constants_str::BOUNDED_UNIQUE_VEC_ABOVE_MAX)
        );
    }
}
