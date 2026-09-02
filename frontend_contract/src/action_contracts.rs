#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub struct ActionContracts(
    bounded_types::bounded_vec::BoundedVec<
        crate::action_contract::ActionContract,
        0,
        { usize::MAX },
    >,
);

impl TryFrom<Vec<crate::action_contract::ActionContract>> for ActionContracts {
    type Error = bounded_types::bounded_value_error::BoundedValueError;

    fn try_from(value: Vec<crate::action_contract::ActionContract>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}

impl ActionContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = crate::action_contract::ActionContract>,
    {
        Self::from(bounded_types::bounded_vec::BoundedVec::from_max_iter(
            values,
        ))
    }
}
