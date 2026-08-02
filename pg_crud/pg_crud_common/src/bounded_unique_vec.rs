#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::Display, newtype::FromInner)]
pub struct UniqueVecLen(usize);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UniqueVecError {
    #[error("{} {max}", str_constants::BOUNDED_UNIQUE_VEC_ABOVE_MAX)]
    AboveMax { max: UniqueVecLen },
    #[error("{}: {actual} < {min}", str_constants::BOUNDED_UNIQUE_VEC_BELOW_MIN)]
    BelowMin {
        actual: UniqueVecLen,
        min: UniqueVecLen,
    },
    #[error("{}", str_constants::BOUNDED_UNIQUE_VEC_DUPLICATE)]
    Duplicate,
    #[error("{}: {min} > {max}", str_constants::BOUNDED_UNIQUE_VEC_INVALID_BOUNDS)]
    InvalidBounds {
        min: UniqueVecLen,
        max: UniqueVecLen,
    },
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, serde::Serialize, newtype::AsRefTarget)]
#[serde(transparent)]
pub struct BoundedUniqueVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T: PartialEq, const MIN: usize, const MAX: usize> TryFrom<Vec<T>>
    for BoundedUniqueVec<T, MIN, MAX>
{
    type Error = UniqueVecError;
    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        let bounded_values = bounded_types::BoundedVec::<T, MIN, MAX>::try_from(values)
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

#[derive(optml::Optml, newtype::FromInner)]
struct StdBoundedUniqueVecVisitor<T, const MIN: usize, const MAX: usize>(
    std::marker::PhantomData<T>,
);
impl<'de, T: serde::Deserialize<'de> + PartialEq, const MIN: usize, const MAX: usize>
    serde::de::Visitor<'de> for StdBoundedUniqueVecVisitor<T, MIN, MAX>
{
    type Value = BoundedUniqueVec<T, MIN, MAX>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(str_constants::BOUNDED_UNIQUE_VEC_EXPECTING)
    }
    fn visit_seq<Access>(self, mut seq: Access) -> Result<Self::Value, Access::Error>
    where
        Access: serde::de::SeqAccess<'de>,
    {
        bounded_types::BoundedVec::<T, MIN, MAX>::validate_bounds()
            .map_err(UniqueVecError::from)
            .map_err(serde::de::Error::custom)?;
        let mut values = Vec::new();
        loop {
            if values.len() == MAX {
                return seq.next_element::<serde::de::IgnoredAny>()?.map_or_else(
                    || BoundedUniqueVec::try_from(values).map_err(serde::de::Error::custom),
                    |_ignored| {
                        Err(serde::de::Error::custom(UniqueVecError::AboveMax {
                            max: MAX.into(),
                        }))
                    },
                );
            }
            let Some(item) = seq.next_element::<T>()? else {
                return BoundedUniqueVec::try_from(values).map_err(serde::de::Error::custom);
            };
            if values.contains(&item) {
                return Err(serde::de::Error::custom(UniqueVecError::Duplicate));
            }
            values.push(item);
        }
    }
}
impl<'de, T: serde::Deserialize<'de> + PartialEq, const MIN: usize, const MAX: usize>
    serde::Deserialize<'de> for BoundedUniqueVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(StdBoundedUniqueVecVisitor::from(std::marker::PhantomData))
    }
}
impl From<bounded_types::BoundedValueError> for UniqueVecError {
    fn from(value: bounded_types::BoundedValueError) -> Self {
        match value {
            bounded_types::BoundedValueError::AboveMax { max, .. } => Self::AboveMax {
                max: UniqueVecLen::from(max.get()),
            },
            bounded_types::BoundedValueError::BelowMin { actual, min } => Self::BelowMin {
                actual: UniqueVecLen::from(actual.get()),
                min: UniqueVecLen::from(min.get()),
            },
            bounded_types::BoundedValueError::InvalidBounds { min, max } => Self::InvalidBounds {
                min: UniqueVecLen::from(min.get()),
                max: UniqueVecLen::from(max.get()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn duplicate_is_rejected_before_later_invalid_item() {
        let result = serde_json::from_str::<super::BoundedUniqueVec<u8, 1, 4>>(
            str_constants::TEST_BOUNDED_UNIQUE_VEC_DUPLICATE_THEN_INVALID,
        );
        assert!(
            matches!(result, Err(error) if error.to_string().contains(str_constants::DUPLICATE))
        );
    }

    #[test]
    fn shared_bounds_map_to_existing_unique_errors() {
        assert_eq!(
            super::BoundedUniqueVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("e71d26a6"),
            super::UniqueVecError::BelowMin {
                actual: super::UniqueVecLen::from(0usize),
                min: super::UniqueVecLen::from(1usize),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 0, 1>::try_from(vec![1u8, 2u8]).expect_err("c98b4208"),
            super::UniqueVecError::AboveMax {
                max: super::UniqueVecLen::from(1usize),
            }
        );
        assert_eq!(
            super::BoundedUniqueVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("6898eb44"),
            super::UniqueVecError::InvalidBounds {
                min: super::UniqueVecLen::from(2usize),
                max: super::UniqueVecLen::from(1usize),
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
            str_constants::TEST_BOUNDED_UNIQUE_VEC_EXCESS_INVALID,
        )
        .expect_err("f551f290");
        assert!(
            error
                .to_string()
                .contains(str_constants::BOUNDED_UNIQUE_VEC_ABOVE_MAX)
        );
    }
}
