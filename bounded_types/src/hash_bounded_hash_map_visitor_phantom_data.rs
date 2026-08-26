#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct BoundedHashMapVisitorPhantomData<K, V, const MAX: usize>(
    std::marker::PhantomData<(K, V)>,
);
impl<
    'de,
    K: Eq + std::hash::Hash + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    const MAX: usize,
> serde::de::Visitor<'de> for BoundedHashMapVisitorPhantomData<K, V, MAX>
{
    type Value = super::bounded_hash_map::BoundedHashMap<K, V, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a map with at most {MAX} entries")
    }

    fn visit_map<Map>(self, map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        let capacity = map
            .size_hint()
            .unwrap_or(constants_usize::ZERO)
            .min(MAX)
            .min(super::super::SERDE_PREALLOC_MAX_ITEMS);
        let mut values = super::bounded_hash_map::BoundedHashMap::default();
        values.reserve(capacity);
        super::super::deserialize_bounded_map::<_, K, V, _, _, MAX>(
            map,
            values,
            |bounded_values, key, value| bounded_values.try_insert(key, value).map(|_previous| ()),
        )
    }
}
