use super::FieldContract;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct FieldContracts(
    bounded_types::domain_types::vector::BoundedVec<FieldContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<FieldContract>> for FieldContracts {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<FieldContract>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl FieldContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = FieldContract>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
    }
}
