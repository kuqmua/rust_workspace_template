#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ActionContracts(bounded_types::BoundedVec<super::ActionContract, 0, { usize::MAX }>);

impl TryFrom<Vec<super::ActionContract>> for ActionContracts {
    type Error = bounded_types::BoundedValueError;

    fn try_from(value: Vec<super::ActionContract>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from_collection_vec(value).map(Self::from)
    }
}

impl ActionContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = super::ActionContract>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
