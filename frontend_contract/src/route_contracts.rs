#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct RouteContracts(
    bounded_types::bounded_vec::BoundedVec<crate::route_contract::RouteContract, 0, { usize::MAX }>,
);

impl TryFrom<Vec<crate::route_contract::RouteContract>> for RouteContracts {
    type Error = bounded_types::bounded_value_error::BoundedValueError;

    fn try_from(value: Vec<crate::route_contract::RouteContract>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}

impl RouteContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::route_contract::RouteContract>,
    {
        Self::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
            values,
        ))
    }
}
