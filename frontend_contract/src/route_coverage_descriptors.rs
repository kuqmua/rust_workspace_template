#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
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
