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
pub struct RouteMetadataList(
    bounded_types::bounded_vec::BoundedVec<crate::route_metadata::RouteMetadata, 0, { usize::MAX }>,
);
impl TryFrom<Vec<crate::route_metadata::RouteMetadata>> for RouteMetadataList {
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(value: Vec<crate::route_metadata::RouteMetadata>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}
