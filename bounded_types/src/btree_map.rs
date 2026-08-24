#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
/// A B-tree map with at most `MAX` retained keys.
///
/// B-tree-map deserialization accepts at most `MAX` wire entries, including repeated keys.
pub struct StdBoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);
impl<K: Ord, V, const MAX: usize> StdBoundedBTreeMap<K, V, MAX> {
    #[must_use]
    pub const fn as_map(&self) -> &std::collections::BTreeMap<K, V> {
        &self.0
    }

    #[must_use]
    pub fn into_inner(self) -> std::collections::BTreeMap<K, V> {
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

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, K, V> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<'_, K, V> {
        self.0.iter_mut()
    }

    pub fn into_values(self) -> std::collections::btree_map::IntoValues<K, V> {
        self.0.into_values()
    }

    #[must_use]
    pub fn len(&self) -> super::BoundedLen {
        super::BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.0.pop_first()
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, super::BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                Ok(Some(entry.insert(value)))
            }
            std::collections::btree_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(super::BoundedValueError::AboveMax {
                    actual: super::BoundedLen::from(MAX.saturating_add(1usize)),
                    max: super::BoundedLen::from(MAX),
                })
            }
            std::collections::btree_map::Entry::Vacant(entry) => {
                let _inserted = entry.insert(value);
                Ok(None)
            }
        }
    }
}
impl<'map_lt, K: Ord, V, const MAX: usize> IntoIterator for &'map_lt StdBoundedBTreeMap<K, V, MAX> {
    type IntoIter = std::collections::btree_map::Iter<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'map_lt, K: Ord, V, const MAX: usize> IntoIterator
    for &'map_lt mut StdBoundedBTreeMap<K, V, MAX>
{
    type IntoIter = std::collections::btree_map::IterMut<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt mut V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
impl<K: Ord, V, const MAX: usize> Default for StdBoundedBTreeMap<K, V, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<K: Ord, V, const MAX: usize> From<[(K, V); 0]> for StdBoundedBTreeMap<K, V, MAX> {
    fn from(_value: [(K, V); 0]) -> Self {
        Self(std::collections::BTreeMap::new())
    }
}
impl<K: Ord, V, const MAX: usize> TryFrom<std::collections::BTreeMap<K, V>>
    for StdBoundedBTreeMap<K, V, MAX>
{
    type Error = super::BoundedValueError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        super::validate_len::<0, MAX>(super::BoundedLen::from(value.len())).map(|()| Self(value))
    }
}
impl<K: Ord + serde::Serialize, V: serde::Serialize, const MAX: usize> serde::Serialize
    for StdBoundedBTreeMap<K, V, MAX>
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
#[derive(optml::Optml, newtype::FromInner)]
struct StdPhantomDataBoundedBTreeMapVisitor<K, V, const MAX: usize>(
    std::marker::PhantomData<(K, V)>,
);
impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::de::Visitor<'de> for StdPhantomDataBoundedBTreeMapVisitor<K, V, MAX>
{
    type Value = StdBoundedBTreeMap<K, V, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a map with at most {MAX} entries")
    }

    fn visit_map<Map>(self, map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        super::deserialize_bounded_map::<_, K, V, _, _, MAX>(
            map,
            StdBoundedBTreeMap::default(),
            |values, key, value| values.try_insert(key, value).map(|_previous| ()),
        )
    }
}
impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::Deserialize<'de> for StdBoundedBTreeMap<K, V, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(StdPhantomDataBoundedBTreeMapVisitor::from(
            std::marker::PhantomData,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn btree_map_preserves_key_order() {
        let value =
            super::StdBoundedBTreeMap::<u8, u8, 2>::try_from(std::collections::BTreeMap::from([
                (2u8, 2u8),
                (1u8, 1u8),
            ]))
            .expect("c51f09be btree_map_preserves_key_order invariant must hold");
        assert_eq!(
            value.as_map().keys().copied().collect::<Vec<_>>(),
            vec![1u8, 2u8]
        );
    }
}
