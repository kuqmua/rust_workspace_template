#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UniqueVecLen(usize);
impl From<usize> for UniqueVecLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for UniqueVecLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct BoundedUniqueVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> AsRef<[T]> for BoundedUniqueVec<T, MIN, MAX> {
    fn as_ref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T: PartialEq, const MIN: usize, const MAX: usize> TryFrom<Vec<T>>
    for BoundedUniqueVec<T, MIN, MAX>
{
    type Error = UniqueVecError;
    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        validate_bounds::<MIN, MAX>()?;
        if values.len() < MIN {
            return Err(Self::Error::BelowMin {
                actual: values.len().into(),
                min: MIN.into(),
            });
        }
        if values.len() > MAX {
            return Err(Self::Error::AboveMax { max: MAX.into() });
        }
        if values
            .iter()
            .enumerate()
            .any(|(idx, item)| values.get(..idx).is_some_and(|seen| seen.contains(item)))
        {
            return Err(Self::Error::Duplicate);
        }
        Ok(Self(values))
    }
}

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
        validate_bounds::<MIN, MAX>().map_err(serde::de::Error::custom)?;
        let mut values = Vec::with_capacity(seq.size_hint().unwrap_or(0usize).min(MAX));
        while let Some(item) = seq.next_element::<T>()? {
            if values.len() == MAX {
                return Err(serde::de::Error::custom(UniqueVecError::AboveMax {
                    max: MAX.into(),
                }));
            }
            if values.contains(&item) {
                return Err(serde::de::Error::custom(UniqueVecError::Duplicate));
            }
            values.push(item);
        }
        if values.len() < MIN {
            Err(serde::de::Error::custom(UniqueVecError::BelowMin {
                actual: values.len().into(),
                min: MIN.into(),
            }))
        } else {
            Ok(BoundedUniqueVec(values))
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
        deserializer.deserialize_seq(StdBoundedUniqueVecVisitor(std::marker::PhantomData))
    }
}
const fn validate_bounds<const MIN: usize, const MAX: usize>() -> Result<(), UniqueVecError> {
    if MIN > MAX {
        Err(UniqueVecError::InvalidBounds {
            min: UniqueVecLen(MIN),
            max: UniqueVecLen(MAX),
        })
    } else {
        Ok(())
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
}
