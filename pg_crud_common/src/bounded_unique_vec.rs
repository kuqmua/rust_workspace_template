#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    proc_macro_newtype::AsRefTarget,
)]
#[serde(transparent)]
pub struct BoundedUniqueVec<T, const MIN: usize, const MAX: usize>(Vec<T>);

impl<T: PartialEq, const MIN: usize, const MAX: usize> TryFrom<Vec<T>>
    for BoundedUniqueVec<T, MIN, MAX>
{
    type Error = crate::unique_vec_error::UniqueVecError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let bounded_values = bounded_types::bounded_vec::BoundedVec::<T, MIN, MAX>::try_from(value)
            .map_err(crate::unique_vec_error::UniqueVecError::from)?
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
            crate::bounded_unique_vec_visitor_phantom_data::BoundedUniqueVecVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_duplicate_is_rejected_before_later_invalid_item() {
        let result = serde_json::from_str::<crate::bounded_unique_vec::BoundedUniqueVec<u8, 1, 4>>(
            constants_str::TEST_BOUNDED_UNIQUE_VEC_DUPLICATE_THEN_INVALID,
        );
        assert!(
            matches!(result, Err(error) if error.to_string().contains(constants_str::DUPLICATE))
        );
    }

    #[test]
    fn test_shared_bounds_map_to_existing_unique_errors() {
        assert_eq!(
            crate::bounded_unique_vec::BoundedUniqueVec::<u8, 1, 2>::try_from(Vec::new())
                .expect_err(constants_str::VALUE_E71D26A6),
            crate::unique_vec_error::UniqueVecError::BelowMin {
                actual: crate::unique_vec_len::UniqueVecLen::from(constants_usize::ZERO),
                min: crate::unique_vec_len::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            crate::bounded_unique_vec::BoundedUniqueVec::<u8, 0, 1>::try_from(vec![1u8, 2u8])
                .expect_err(constants_str::VALUE_C98B4208),
            crate::unique_vec_error::UniqueVecError::AboveMax {
                max: crate::unique_vec_len::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            crate::bounded_unique_vec::BoundedUniqueVec::<u8, 2, 1>::try_from(vec![1u8])
                .expect_err(constants_str::VALUE_6898EB44),
            crate::unique_vec_error::UniqueVecError::InvalidBounds {
                min: crate::unique_vec_len::UniqueVecLen::from(2usize),
                max: crate::unique_vec_len::UniqueVecLen::from(constants_usize::ONE),
            }
        );
        assert_eq!(
            crate::bounded_unique_vec::BoundedUniqueVec::<u8, 0, 2>::try_from(vec![1u8, 1u8])
                .expect_err(constants_str::VALUE_DC0F5D9F),
            crate::unique_vec_error::UniqueVecError::Duplicate
        );
    }

    #[test]
    fn test_excess_item_is_ignored_without_deserializing_target_type() {
        let error = serde_json::from_str::<crate::bounded_unique_vec::BoundedUniqueVec<u8, 0, 1>>(
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
