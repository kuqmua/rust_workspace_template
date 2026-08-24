#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
/// A hash map with at most `MAX` retained keys.
///
/// Hash-map deserialization also accepts at most `MAX` wire entries, including repeated keys.
pub struct StdBoundedHashMap<K: Eq + std::hash::Hash, V, const MAX: usize>(
    std::collections::HashMap<K, V>,
);
impl<K: Eq + std::hash::Hash, V, const MAX: usize> StdBoundedHashMap<K, V, MAX> {
    #[cfg(test)]
    pub(super) fn allocation_capacity(&self) -> usize {
        self.0.capacity()
    }

    #[must_use]
    pub const fn as_map(&self) -> &std::collections::HashMap<K, V> {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> std::collections::HashMap<K, V> {
        self.0
    }

    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    #[must_use]
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.0.iter()
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.0.iter_mut()
    }

    #[must_use]
    pub fn len(&self) -> super::BoundedLen {
        super::BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, super::BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => Ok(Some(entry.insert(value))),
            std::collections::hash_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(super::BoundedValueError::AboveMax {
                    actual: super::BoundedLen::from(MAX.saturating_add(usize_constants::ONE)),
                    max: super::BoundedLen::from(MAX),
                })
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let _inserted = entry.insert(value);
                Ok(None)
            }
        }
    }
}
impl<'map_lt, K: Eq + std::hash::Hash, V, const MAX: usize> IntoIterator
    for &'map_lt StdBoundedHashMap<K, V, MAX>
{
    type IntoIter = std::collections::hash_map::Iter<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'map_lt, K: Eq + std::hash::Hash, V, const MAX: usize> IntoIterator
    for &'map_lt mut StdBoundedHashMap<K, V, MAX>
{
    type IntoIter = std::collections::hash_map::IterMut<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt mut V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> Default for StdBoundedHashMap<K, V, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> From<[(K, V); 0]>
    for StdBoundedHashMap<K, V, MAX>
{
    fn from(_value: [(K, V); 0]) -> Self {
        Self(std::collections::HashMap::new())
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> TryFrom<std::collections::HashMap<K, V>>
    for StdBoundedHashMap<K, V, MAX>
{
    type Error = super::BoundedValueError;

    fn try_from(value: std::collections::HashMap<K, V>) -> Result<Self, Self::Error> {
        super::validate_len::<0, MAX>(super::BoundedLen::from(value.len())).map(|()| Self(value))
    }
}
impl<K: Eq + std::hash::Hash + serde::Serialize, V: serde::Serialize, const MAX: usize>
    serde::Serialize for StdBoundedHashMap<K, V, MAX>
{
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serde::Serialize::serialize(&self.0, serializer)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct StdPhantomDataBoundedHashMapVisitor<K, V, const MAX: usize>(
    std::marker::PhantomData<(K, V)>,
);
impl<
    'de,
    K: Eq + std::hash::Hash + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    const MAX: usize,
> serde::de::Visitor<'de> for StdPhantomDataBoundedHashMapVisitor<K, V, MAX>
{
    type Value = StdBoundedHashMap<K, V, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a map with at most {MAX} entries")
    }

    fn visit_map<Map>(self, map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        let capacity = map
            .size_hint()
            .unwrap_or(usize_constants::ZERO)
            .min(MAX)
            .min(super::SERDE_PREALLOC_MAX_ITEMS);
        let mut values = StdBoundedHashMap::default();
        values.0.reserve(capacity);
        super::deserialize_bounded_map::<_, K, V, _, _, MAX>(
            map,
            values,
            |bounded_values, key, value| bounded_values.try_insert(key, value).map(|_previous| ()),
        )
    }
}
impl<
    'de,
    K: Eq + std::hash::Hash + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    const MAX: usize,
> serde::Deserialize<'de> for StdBoundedHashMap<K, V, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(StdPhantomDataBoundedHashMapVisitor::from(
            std::marker::PhantomData,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_map_rejects_new_keys_at_capacity() {
        let mut value = super::StdBoundedHashMap::<u8, u8, 1>::default();
        assert!(matches!(value.try_insert(1u8, 1u8), Ok(None)));
        assert!(matches!(
            value.try_insert(2u8, 2u8),
            Err(super::super::BoundedValueError::AboveMax { .. })
        ));
    }
}
