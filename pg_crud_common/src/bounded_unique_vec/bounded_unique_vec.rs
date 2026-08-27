#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    newtype::AsRefTarget,
)]
#[serde(transparent)]
pub struct BoundedUniqueVec<T, const MIN: usize, const MAX: usize>(Vec<T>);

impl<T: PartialEq, const MIN: usize, const MAX: usize> TryFrom<Vec<T>>
    for BoundedUniqueVec<T, MIN, MAX>
{
    type Error = super::UniqueVecError;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let bounded_values =
            bounded_types::domain_types::vector::BoundedVec::<T, MIN, MAX>::try_from(values)
                .map_err(super::UniqueVecError::from)?
                .into_inner();
        if bounded_values.iter().enumerate().any(|(idx, item)| {
            bounded_values
                .get(..idx)
                .is_some_and(|seen| seen.contains(item))
        }) {
            return Err(Self::Error::Duplicate);
        }
        Ok(Self(bounded_values))
    }
}

impl<'de, T: serde::Deserialize<'de> + PartialEq, const MIN: usize, const MAX: usize>
    serde::Deserialize<'de> for BoundedUniqueVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(
            super::bounded_unique_vec_visitor_phantom_data::BoundedUniqueVecVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}
