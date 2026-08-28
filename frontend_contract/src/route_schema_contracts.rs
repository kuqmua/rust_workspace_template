use super::RouteSchemaContract;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RouteSchemaContracts(
    bounded_types::domain_types::vector::BoundedVec<RouteSchemaContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<RouteSchemaContract>> for RouteSchemaContracts {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<RouteSchemaContract>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl RouteSchemaContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = RouteSchemaContract>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
    }
}
