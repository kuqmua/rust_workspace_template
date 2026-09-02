#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
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
        vec: Vec<crate::route_test_category::RouteTestCategory>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(vec).map(Self::from)
    }
}
