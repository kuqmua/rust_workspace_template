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
    bounded_types::BoundedVec<crate::RouteCoverageDescriptor, 0, { usize::MAX }>,
);
impl TryFrom<Vec<crate::RouteCoverageDescriptor>> for RouteCoverageDescriptors {
    type Error = bounded_types::BoundedValueError;
    fn try_from(value: Vec<crate::RouteCoverageDescriptor>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}
impl RouteCoverageDescriptors {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::RouteCoverageDescriptor>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
