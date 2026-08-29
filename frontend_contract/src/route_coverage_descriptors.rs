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
    bounded_types::bounded_vec::BoundedVec<
        crate::route_coverage_descriptor::RouteCoverageDescriptor,
        0,
        { usize::MAX },
    >,
);
impl TryFrom<Vec<crate::route_coverage_descriptor::RouteCoverageDescriptor>>
    for RouteCoverageDescriptors
{
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(
        value: Vec<crate::route_coverage_descriptor::RouteCoverageDescriptor>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}
impl RouteCoverageDescriptors {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::route_coverage_descriptor::RouteCoverageDescriptor>,
    {
        Self::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
            values,
        ))
    }
}
