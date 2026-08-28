#![allow(
    clippy::module_name_repetitions,
    reason = "bounded vector companion types retain their established public names"
)]
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::IntoInnerFrom,
)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);

impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[must_use]
    pub fn len(&self) -> BoundedVecLen {
        BoundedVecLen::from(self.0.len())
    }
}

impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = BoundedVecError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::<T, MIN, MAX>::try_from(value)
            .map(|bounded| Self(bounded.into_inner()))
            .map_err(BoundedVecError::from)
    }
}

impl<T: serde::Serialize, const MIN: usize, const MAX: usize> serde::Serialize
    for BoundedVec<T, MIN, MAX>
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
    for BoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <bounded_types::domain_types::vector::BoundedVec<T, MIN, MAX> as serde::Deserialize>::deserialize(
            deserializer,
        )?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl<T: schemars::JsonSchema, const MIN: usize, const MAX: usize> schemars::JsonSchema
    for BoundedVec<T, MIN, MAX>
{
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = generator.subschema_for::<Vec<T>>();
        let _previous_min = schema.insert(constants_str::MINITEMS.to_owned(), MIN.into());
        let _previous_max = schema.insert(constants_str::MAXITEMS.to_owned(), MAX.into());
        schema
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("{}::BoundedVec<{MIN},{MAX}>", T::schema_id()))
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(format!("BoundedVec_{MIN}_{MAX}_{}", T::schema_name()))
    }
}

impl<T: utoipa::PartialSchema, const MIN: usize, const MAX: usize> utoipa::PartialSchema
    for BoundedVec<T, MIN, MAX>
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
    for BoundedVec<T, MIN, MAX>
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::BOUNDEDVEC)
    }
}
pub use crate::bounded_vec_error::BoundedVecError;
pub use crate::bounded_vec_len::BoundedVecLen;
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
