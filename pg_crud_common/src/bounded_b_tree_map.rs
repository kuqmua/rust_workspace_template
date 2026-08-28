#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct BoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);

impl<K, V, const MAX: usize> BoundedBTreeMap<K, V, MAX> {
    #[must_use]
    pub const fn get(&self) -> &std::collections::BTreeMap<K, V> {
        &self.0
    }
}

impl<K: Ord, V, const MAX: usize> TryFrom<std::collections::BTreeMap<K, V>>
    for BoundedBTreeMap<K, V, MAX>
{
    type Error = super::bounded_b_tree_map_error::BoundedBTreeMapError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        bounded_types::BoundedBTreeMap::<K, V, MAX>::try_from(value)
            .map(bounded_types::BoundedBTreeMap::into_inner)
            .map(Self)
            .map_err(|_error| {
                super::bounded_b_tree_map_error::BoundedBTreeMapError::from(
                    super::std_bounded_b_tree_map_len::StdBoundedBTreeMapLen::from(MAX),
                )
            })
    }
}

impl<K: serde::Serialize, V: serde::Serialize, const MAX: usize> serde::Serialize
    for BoundedBTreeMap<K, V, MAX>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::Deserialize<'de> for BoundedBTreeMap<K, V, MAX>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <bounded_types::BoundedBTreeMap<K, V, MAX> as serde::Deserialize>::deserialize(
            deserializer,
        )?
        .into_inner();
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialization_stops_above_limit() {
        let result = serde_json::from_str::<super::BoundedBTreeMap<String, u8, 1>>(
            constants_str::TEST_JSON_MAP_WITH_TWO_ENTRIES,
        );
        let _error = result.expect_err(constants_str::VALUE_AB603731);
    }

    #[test]
    fn map_at_limit_is_accepted() {
        let value = serde_json::from_str::<super::BoundedBTreeMap<String, u8, 1>>(
            constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY,
        )
        .expect("298b587f map_at_limit_is_accepted invariant must hold");
        assert_eq!(value.get().len(), constants_usize::ONE);
    }
}
