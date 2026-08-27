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
    pub fn len(&self) -> super::BoundedVecLen {
        super::BoundedVecLen::from(self.0.len())
    }
}

impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = super::BoundedVecError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::<T, MIN, MAX>::try_from(value)
            .map(|bounded| Self(bounded.into_inner()))
            .map_err(super::BoundedVecError::from)
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
