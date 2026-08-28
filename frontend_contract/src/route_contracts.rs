#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct RouteContracts(bounded_types::BoundedVec<super::RouteContract, 0, { usize::MAX }>);

impl TryFrom<Vec<super::RouteContract>> for RouteContracts {
    type Error = bounded_types::BoundedValueError;

    fn try_from(value: Vec<super::RouteContract>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}

impl RouteContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = super::RouteContract>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
