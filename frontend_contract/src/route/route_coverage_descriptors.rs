#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct RouteCoverageDescriptors(
    bounded_types::domain_types::vector::BoundedVec<
        crate::domain_types::RouteCoverageDescriptor,
        0,
        { usize::MAX },
    >,
);
impl TryFrom<Vec<crate::domain_types::RouteCoverageDescriptor>> for RouteCoverageDescriptors {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(
        value: Vec<crate::domain_types::RouteCoverageDescriptor>,
    ) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl RouteCoverageDescriptors {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::domain_types::RouteCoverageDescriptor>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
    }
}
