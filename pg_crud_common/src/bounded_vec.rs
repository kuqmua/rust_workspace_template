#[path = "bounded_vec/bounded_vec.rs"]
mod bounded_vec;
#[path = "bounded_vec/bounded_vec_error.rs"]
mod bounded_vec_error;
#[path = "bounded_vec/bounded_vec_len.rs"]
mod bounded_vec_len;

pub use bounded_vec::BoundedVec;
pub use bounded_vec_error::BoundedVecError;
pub use bounded_vec_len::BoundedVecLen;
#[cfg(test)]
mod tests {
    #[test]
    fn try_from_enforces_inclusive_bounds() {
        assert!(matches!(
            super::BoundedVec::<u8, 1, 2>::try_from(Vec::new()),
            Err(super::BoundedVecError::BelowMin { .. })
        ));
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8])
                .expect("0901ec3d try_from_enforces_inclusive_bounds invariant must hold")
                .as_slice(),
            &[1u8]
        );
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8])
                .expect("324b4da9 try_from_enforces_inclusive_bounds invariant must hold")
                .as_slice(),
            &[1u8, 2u8]
        );
        assert!(matches!(
            super::BoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8, 3u8]),
            Err(super::BoundedVecError::AboveMax { .. })
        ));
    }
    #[test]
    fn invalid_bounds_are_rejected() {
        assert!(matches!(
            super::BoundedVec::<u8, 2, 1>::try_from(vec![1u8]),
            Err(super::BoundedVecError::InvalidBounds { .. })
        ));
    }
    #[test]
    fn serde_round_trip_and_limits_are_stable() {
        let value = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [1u8, 2u8].into_iter(),
            ),
        )
        .expect("9dcb60bc serde_round_trip_and_limits_are_stable invariant must hold");
        assert_eq!(value.as_slice(), &[1u8, 2u8]);
        let below_min = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                std::iter::empty::<u8>(),
            ),
        );
        let _error = below_min.expect_err(constants_str::CBBF6ACF);
        let error = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [1u8, 2u8, 3u8, 4u8].into_iter(),
            ),
        );
        let _above_max_error = error.expect_err(constants_str::VALUE_91C59B94);
    }
    #[test]
    fn schemas_match_runtime_bounds() {
        let schema = schemars::schema_for!(super::BoundedVec<u8, 1, 2>);
        assert_eq!(
            schema
                .get("minItems")
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(1u64)
        );
        assert_eq!(
            schema
                .get("maxItems")
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(2u64)
        );
        let open_api_schema = <super::BoundedVec<u8, 1, 2> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) =
            open_api_schema
        else {
            panic!("06be97f2");
        };
        assert_eq!(array.min_items, Some(constants_usize::ONE));
        assert_eq!(array.max_items, Some(2usize));
    }
}
