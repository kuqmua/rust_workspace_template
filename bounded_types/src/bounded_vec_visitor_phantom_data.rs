#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct BoundedVecVisitorPhantomData<T, const MIN: usize, const MAX: usize>(
    std::marker::PhantomData<T>,
);
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::de::Visitor<'de>
    for BoundedVecVisitorPhantomData<T, MIN, MAX>
{
    type Value = super::bounded_vec::BoundedVec<T, MIN, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "an array with {MIN} to {MAX} items")
    }

    fn visit_seq<Seq>(self, mut seq: Seq) -> Result<Self::Value, Seq::Error>
    where
        Seq: serde::de::SeqAccess<'de>,
    {
        super::bounded_vec::BoundedVec::<T, MIN, MAX>::validate_bounds()
            .map_err(serde::de::Error::custom)?;
        let capacity = seq
            .size_hint()
            .unwrap_or(MIN)
            .min(MAX)
            .min(crate::serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS);
        let mut values = Vec::with_capacity(capacity);
        loop {
            if values.len() == MAX {
                return seq.next_element::<serde::de::IgnoredAny>()?.map_or_else(
                    || {
                        super::bounded_vec::BoundedVec::try_from(values)
                            .map_err(serde::de::Error::custom)
                    },
                    |_ignored| {
                        Err(serde::de::Error::custom(
                            crate::bounded_value_error::BoundedValueError::AboveMax {
                                actual: crate::bounded_len::BoundedLen::from(
                                    MAX.saturating_add(constants_usize::ONE),
                                ),
                                max: crate::bounded_len::BoundedLen::from(MAX),
                            },
                        ))
                    },
                );
            }
            match seq.next_element()? {
                Some(value) => values.push(value),
                None => {
                    return super::bounded_vec::BoundedVec::try_from(values)
                        .map_err(serde::de::Error::custom);
                }
            }
        }
    }
}
