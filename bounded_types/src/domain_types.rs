#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]
#[path = "bounded_string.rs"]
pub mod bounded_string;
#[path = "btree.rs"]
pub mod btree;
#[path = "hash.rs"]
pub mod hash;
#[path = "vector.rs"]
pub mod vector;

pub const COLLECTION_MAX_LEN: usize = 10_000usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct BoundedLen(usize);
impl std::fmt::Display for BoundedLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum BoundedValueError {
    #[error("bounded value length {actual} exceeds maximum {max}")]
    AboveMax { actual: BoundedLen, max: BoundedLen },
    #[error("bounded value length {actual} is below minimum {min}")]
    BelowMin { actual: BoundedLen, min: BoundedLen },
    #[error("bounded value minimum {min} exceeds maximum {max}")]
    InvalidBounds { min: BoundedLen, max: BoundedLen },
}

fn validate_len<const MIN: usize, const MAX: usize>(
    len: BoundedLen,
) -> Result<(), BoundedValueError> {
    if MIN > MAX {
        Err(BoundedValueError::InvalidBounds {
            min: BoundedLen::from(MIN),
            max: BoundedLen::from(MAX),
        })
    } else if len.get() < MIN {
        Err(BoundedValueError::BelowMin {
            actual: len,
            min: BoundedLen::from(MIN),
        })
    } else if len.get() > MAX {
        Err(BoundedValueError::AboveMax {
            actual: len,
            max: BoundedLen::from(MAX),
        })
    } else {
        Ok(())
    }
}

fn deserialize_bounded_map<'de, Map, Key, Value, Values, Insert, const MAX: usize>(
    mut map: Map,
    mut values: Values,
    mut insert: Insert,
) -> Result<Values, Map::Error>
where
    Map: serde::de::MapAccess<'de>,
    Key: serde::Deserialize<'de>,
    Value: serde::Deserialize<'de>,
    Insert: FnMut(&mut Values, Key, Value) -> Result<(), BoundedValueError>,
{
    let mut entry_count = constants_usize::ZERO;
    loop {
        if entry_count == MAX {
            return map.next_key::<serde::de::IgnoredAny>()?.map_or_else(
                || Ok(values),
                |_ignored| {
                    Err(serde::de::Error::custom(BoundedValueError::AboveMax {
                        actual: BoundedLen::from(MAX.saturating_add(constants_usize::ONE)),
                        max: BoundedLen::from(MAX),
                    }))
                },
            );
        }
        let Some(key) = map.next_key()? else {
            return Ok(values);
        };
        let value = map.next_value()?;
        insert(&mut values, key, value).map_err(serde::de::Error::custom)?;
        entry_count = entry_count.saturating_add(constants_usize::ONE);
    }
}

const SERDE_PREALLOC_MAX_ITEMS: usize = 1024usize;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
