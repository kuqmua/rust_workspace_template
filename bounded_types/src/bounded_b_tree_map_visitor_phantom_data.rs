#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
pub(super) struct BoundedBTreeMapVisitorPhantomData<K, V, const MAX: usize>(
    std::marker::PhantomData<(K, V)>,
);
impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::de::Visitor<'de> for BoundedBTreeMapVisitorPhantomData<K, V, MAX>
{
    type Value = super::bounded_b_tree_map::BoundedBTreeMap<K, V, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a map with at most {MAX} entries")
    }

    fn visit_map<Map>(self, map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        crate::deserialize_bounded_map::deserialize_bounded_map::<_, K, V, _, _, MAX>(
            map,
            super::bounded_b_tree_map::BoundedBTreeMap::default(),
            |values, key, value| values.try_insert(key, value).map(|_previous| ()),
        )
    }
}
