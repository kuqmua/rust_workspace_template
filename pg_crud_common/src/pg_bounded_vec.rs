#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct PgBoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);

impl<T, const MIN: usize, const MAX: usize> PgBoundedVec<T, MIN, MAX> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[must_use]
    pub fn len(&self) -> crate::pg_bounded_vec_len::PgBoundedVecLen {
        crate::pg_bounded_vec_len::PgBoundedVecLen::from(self.0.len())
    }
}

impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for PgBoundedVec<T, MIN, MAX> {
    type Error = crate::bounded_vec_error::BoundedVecError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::<T, MIN, MAX>::try_from(vec)
            .map(|bounded| Self(bounded.into_inner()))
            .map_err(crate::bounded_vec_error::BoundedVecError::from)
    }
}

impl<T: serde::Serialize, const MIN: usize, const MAX: usize> serde::Serialize
    for PgBoundedVec<T, MIN, MAX>
{
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for PgBoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::bounded_vec::BoundedVec<T, MIN, MAX> as serde::Deserialize>::deserialize(
            deserializer,
        )?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl<T: schemars::JsonSchema, const MIN: usize, const MAX: usize> schemars::JsonSchema
    for PgBoundedVec<T, MIN, MAX>
{
    fn json_schema(schema_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = schema_generator.subschema_for::<Vec<T>>();
        let _previous_min = schema.insert(constants_str::MINITEMS.to_owned(), MIN.into());
        let _previous_max = schema.insert(constants_str::MAXITEMS.to_owned(), MAX.into());
        schema
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}::PgBoundedVec<{MIN},{MAX}>", T::schema_id()))
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("BoundedVec_{MIN}_{MAX}_{}", T::schema_name()))
    }
}

impl<T: utoipa::PartialSchema, const MIN: usize, const MAX: usize> utoipa::PartialSchema
    for PgBoundedVec<T, MIN, MAX>
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(MIN))
            .max_items(Some(MAX))
            .build()
            .into()
    }
}

impl<T: utoipa::ToSchema, const MIN: usize, const MAX: usize> utoipa::ToSchema
    for PgBoundedVec<T, MIN, MAX>
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::BOUNDEDVEC)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_try_from_enforces_inclusive_bounds() {
        assert!(matches!(
            crate::pg_bounded_vec::PgBoundedVec::<u8, 1, 2>::try_from(Vec::new()),
            Err(crate::bounded_vec_error::BoundedVecError::BelowMin { .. })
        ));
        assert_eq!(
            crate::pg_bounded_vec::PgBoundedVec::<u8, 1, 2>::try_from(vec![1u8])
                .expect(constants_str::DIAGNOSTIC_0901EC3D)
                .as_slice(),
            &[1u8]
        );
        assert_eq!(
            crate::pg_bounded_vec::PgBoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8])
                .expect(constants_str::DIAGNOSTIC_324B4DA9)
                .as_slice(),
            &[1u8, 2u8]
        );
        assert!(matches!(
            crate::pg_bounded_vec::PgBoundedVec::<u8, 1, 2>::try_from(vec![1u8, 2u8, 3u8]),
            Err(crate::bounded_vec_error::BoundedVecError::AboveMax { .. })
        ));
    }
    #[test]
    fn test_invalid_bounds_are_rejected() {
        assert!(matches!(
            crate::pg_bounded_vec::PgBoundedVec::<u8, 2, 1>::try_from(vec![1u8]),
            Err(crate::bounded_vec_error::BoundedVecError::InvalidBounds { .. })
        ));
    }
    #[test]
    fn test_serde_round_trip_and_limits_are_stable() {
        let value =
            <crate::pg_bounded_vec::PgBoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
                serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                    [1u8, 2u8].into_iter(),
                ),
            )
            .expect(constants_str::DIAGNOSTIC_9DCB60BC);
        assert_eq!(value.as_slice(), &[1u8, 2u8]);
        let below_min =
            <crate::pg_bounded_vec::PgBoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
                serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                    std::iter::empty::<u8>(),
                ),
            );
        let _error = below_min.expect_err(constants_str::CBBF6ACF);
        let error =
            <crate::pg_bounded_vec::PgBoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
                serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                    [1u8, 2u8, 3u8, 4u8].into_iter(),
                ),
            );
        let _above_max_error = error.expect_err(constants_str::VALUE_91C59B94);
    }
    #[test]
    fn test_schemas_match_runtime_bounds() {
        let schema = schemars::schema_for!(crate::pg_bounded_vec::PgBoundedVec<u8, 1, 2>);
        assert_eq!(
            schema
                .get(constants_str::MINITEMS)
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(1u64)
        );
        assert_eq!(
            schema
                .get(constants_str::MAXITEMS)
                .and_then(sqlx::types::JsonValue::as_u64),
            Some(2u64)
        );
        let open_api_schema =
            <crate::pg_bounded_vec::PgBoundedVec<u8, 1, 2> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) =
            open_api_schema
        else {
            std::panic::panic_any(constants_str::PANIC_06BE97F2);
        };
        assert_eq!(array.min_items, Some(constants_usize::ONE));
        assert_eq!(array.max_items, Some(2usize));
    }
}
