#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RouteContracts(
    bounded_types::bounded_vec::BoundedVec<crate::route_contract::RouteContract, 0, { usize::MAX }>,
);

impl TryFrom<Vec<crate::route_contract::RouteContract>> for RouteContracts {
    type Error = bounded_types::bounded_value_error::BoundedValueError;

    fn try_from(vec: Vec<crate::route_contract::RouteContract>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(vec).map(Self::from)
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
