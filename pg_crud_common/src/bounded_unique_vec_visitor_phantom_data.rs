#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct BoundedUniqueVecVisitorPhantomData<T, const MIN: usize, const MAX: usize>(
    std::marker::PhantomData<T>,
);

impl<'de, T: serde::Deserialize<'de> + PartialEq, const MIN: usize, const MAX: usize>
    serde::de::Visitor<'de> for BoundedUniqueVecVisitorPhantomData<T, MIN, MAX>
{
    type Value = super::BoundedUniqueVec<T, MIN, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::BOUNDED_UNIQUE_VEC_EXPECTING)
    }

    fn visit_seq<Access>(self, mut seq: Access) -> Result<Self::Value, Access::Error>
    where
        Access: serde::de::SeqAccess<'de>,
    {
        bounded_types::BoundedVec::<T, MIN, MAX>::validate_bounds()
            .map_err(super::UniqueVecError::from)
            .map_err(serde::de::Error::custom)?;
        let mut values = Vec::with_capacity(
            seq.size_hint()
                .unwrap_or(constants_usize::ZERO)
                .min(MAX)
                .min(super::serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS),
        );
        loop {
            if values.len() == MAX {
                return seq.next_element::<serde::de::IgnoredAny>()?.map_or_else(
                    || super::BoundedUniqueVec::try_from(values).map_err(serde::de::Error::custom),
                    |_ignored| {
                        Err(serde::de::Error::custom(super::UniqueVecError::AboveMax {
                            max: MAX.into(),
                        }))
                    },
                );
            }
            let Some(item) = seq.next_element::<T>()? else {
                return super::BoundedUniqueVec::try_from(values).map_err(serde::de::Error::custom);
            };
            if values.contains(&item) {
                return Err(serde::de::Error::custom(super::UniqueVecError::Duplicate));
            }
            values.push(item);
        }
    }
}
