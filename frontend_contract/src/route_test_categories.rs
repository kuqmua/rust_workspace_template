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
pub struct RouteTestCategories(
    bounded_types::bounded_vec::BoundedVec<
        crate::route_test_category::RouteTestCategory,
        0,
        { usize::MAX },
    >,
);
impl TryFrom<Vec<crate::route_test_category::RouteTestCategory>> for RouteTestCategories {
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(
        value: Vec<crate::route_test_category::RouteTestCategory>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}
