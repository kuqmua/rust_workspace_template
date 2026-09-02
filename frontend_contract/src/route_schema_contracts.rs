#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct RouteSchemaContracts(
    bounded_types::bounded_vec::BoundedVec<
        crate::route_schema_contract::RouteSchemaContract,
        0,
        { usize::MAX },
    >,
);
impl TryFrom<Vec<crate::route_schema_contract::RouteSchemaContract>> for RouteSchemaContracts {
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(
        vec: Vec<crate::route_schema_contract::RouteSchemaContract>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(vec).map(Self::from)
    }
}
impl RouteSchemaContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::route_schema_contract::RouteSchemaContract>,
    {
        Self::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
            values,
        ))
    }
}
