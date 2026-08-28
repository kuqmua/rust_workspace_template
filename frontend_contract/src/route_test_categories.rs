use super::RouteTestCategory;

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
pub struct RouteTestCategories(
    bounded_types::domain_types::vector::BoundedVec<RouteTestCategory, 0, { usize::MAX }>,
);
impl TryFrom<Vec<RouteTestCategory>> for RouteTestCategories {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<RouteTestCategory>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
