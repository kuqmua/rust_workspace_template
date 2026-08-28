use super::RouteMetadata;

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
pub struct RouteMetadataList(
    bounded_types::domain_types::vector::BoundedVec<RouteMetadata, 0, { usize::MAX }>,
);
impl TryFrom<Vec<RouteMetadata>> for RouteMetadataList {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<RouteMetadata>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
