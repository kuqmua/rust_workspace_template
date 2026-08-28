#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
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
    type Error = UniqueVecError;

    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let bounded_values =
            bounded_types::domain_types::vector::BoundedVec::<T, MIN, MAX>::try_from(values)
                .map_err(UniqueVecError::from)?
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
            bounded_unique_vec_visitor_phantom_data::BoundedUniqueVecVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}
#[path = "bounded_unique_vec_visitor_phantom_data.rs"]
mod bounded_unique_vec_visitor_phantom_data;
#[path = "serde_prealloc_max_items.rs"]
mod serde_prealloc_max_items;
#[path = "unique_vec_error.rs"]
mod unique_vec_error;
#[path = "unique_vec_len.rs"]
mod unique_vec_len;
pub use unique_vec_error::UniqueVecError;
pub use unique_vec_len::UniqueVecLen;

#[cfg(test)]
mod tests {
    #[test]
    fn duplicate_is_rejected_before_later_invalid_item() {
        let result = serde_json::from_str::<super::BoundedUniqueVec<u8, 1, 4>>(
            constants_str::TEST_BOUNDED_UNIQUE_VEC_DUPLICATE_THEN_INVALID,
        );
        assert!(
            matches!(result, Err(error) if error.to_string().contains(constants_str::DUPLICATE))
        );
    }

    #[test]
    fn shared_bounds_map_to_existing_unique_errors() {
        assert_eq!(
            super::BoundedUniqueVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("e71d26a6"),
            super::UniqueVecError::BelowMin {
                actual: super::UniqueVecLen::from(constants_usize::ZERO),
                min: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 0, 1>::try_from(vec![1u8, 2u8]).expect_err("c98b4208"),
            super::UniqueVecError::AboveMax {
                max: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("6898eb44"),
            super::UniqueVecError::InvalidBounds {
                min: super::UniqueVecLen::from(2usize),
                max: super::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 0, 2>::try_from(vec![1u8, 1u8]).expect_err("dc0f5d9f"),
            super::UniqueVecError::Duplicate
        );
    }

    #[test]
    fn excess_item_is_ignored_without_deserializing_target_type() {
        let error = serde_json::from_str::<super::BoundedUniqueVec<u8, 0, 1>>(
            constants_str::TEST_BOUNDED_UNIQUE_VEC_EXCESS_INVALID,
        )
        .expect_err(constants_str::VALUE_A37B95DF);
        assert!(
            error
                .to_string()
                .contains(constants_str::BOUNDED_UNIQUE_VEC_ABOVE_MAX)
        );
    }
}
