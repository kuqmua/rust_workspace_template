#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct StdBoundedBTreeMapLen(usize);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("bounded map length exceeds limit {0}")]
#[derive(newtype::FromInner)]
pub struct BoundedBTreeMapError(StdBoundedBTreeMapLen);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct StdBoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);

impl<K, V, const MAX: usize> StdBoundedBTreeMap<K, V, MAX> {
    #[must_use]
    pub const fn get(&self) -> &std::collections::BTreeMap<K, V> {
        &self.0
    }
}

impl<K: Ord, V, const MAX: usize> TryFrom<std::collections::BTreeMap<K, V>>
    for StdBoundedBTreeMap<K, V, MAX>
{
    type Error = BoundedBTreeMapError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        bounded_types::StdBoundedBTreeMap::<K, V, MAX>::try_from(value)
            .map(bounded_types::StdBoundedBTreeMap::into_inner)
            .map(Self)
            .map_err(|_error| BoundedBTreeMapError(StdBoundedBTreeMapLen::from(MAX)))
    }
}

impl<K: serde::Serialize, V: serde::Serialize, const MAX: usize> serde::Serialize
    for StdBoundedBTreeMap<K, V, MAX>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::Deserialize<'de> for StdBoundedBTreeMap<K, V, MAX>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value =
            <bounded_types::StdBoundedBTreeMap<K, V, MAX> as serde::Deserialize>::deserialize(
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
        let result = serde_json::from_str::<super::StdBoundedBTreeMap<String, u8, 1>>(
            str_constants::TEST_JSON_MAP_WITH_TWO_ENTRIES,
        );
        let _error = result.expect_err(str_constants::VALUE_AB603731);
    }

    #[test]
    fn map_at_limit_is_accepted() {
        let value = serde_json::from_str::<super::StdBoundedBTreeMap<String, u8, 1>>(
            str_constants::TEST_JSON_MAP_WITH_ONE_ENTRY,
        )
        .expect("298b587f map_at_limit_is_accepted invariant must hold");
        assert_eq!(value.get().len(), 1usize);
    }
}
