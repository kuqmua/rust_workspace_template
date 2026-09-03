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
pub struct RouteMetadataList(
    bounded_types::bounded_vec::BoundedVec<crate::route_metadata::RouteMetadata, 0, { usize::MAX }>,
);
impl TryFrom<Vec<crate::route_metadata::RouteMetadata>> for RouteMetadataList {
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(vec: Vec<crate::route_metadata::RouteMetadata>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(vec).map(Self::from)
    }
}
