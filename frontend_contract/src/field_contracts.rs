#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct FieldContracts(
    bounded_types::bounded_vec::BoundedVec<crate::field_contract::FieldContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<crate::field_contract::FieldContract>> for FieldContracts {
    type Error = bounded_types::bounded_value_error::BoundedValueError;
    fn try_from(value: Vec<crate::field_contract::FieldContract>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}
impl FieldContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::field_contract::FieldContract>,
    {
        Self::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
            values,
        ))
    }
}
