#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
/// A B-tree map with at most `MAX` retained keys.
///
/// B-tree-map deserialization accepts at most `MAX` wire entries, including repeated keys.
pub struct BoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);
impl<K: Ord, V, const MAX: usize> BoundedBTreeMap<K, V, MAX> {
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
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.0.pop_first()
    }

    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<V>, crate::bounded_value_error::BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                Ok(Some(entry.insert(value)))
            }
            std::collections::btree_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(crate::bounded_value_error::BoundedValueError::AboveMax {
                    actual: crate::bounded_len::BoundedLen::from(
                        MAX.saturating_add(constants_usize::ONE),
                    ),
                    max: crate::bounded_len::BoundedLen::from(MAX),
                })
            }
            std::collections::btree_map::Entry::Vacant(entry) => {
                let _inserted = entry.insert(value);
                Ok(None)
            }
        }
    }
}
impl<'map_lt, K: Ord, V, const MAX: usize> IntoIterator for &'map_lt BoundedBTreeMap<K, V, MAX> {
    type IntoIter = std::collections::btree_map::Iter<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'map_lt, K: Ord, V, const MAX: usize> IntoIterator
    for &'map_lt mut BoundedBTreeMap<K, V, MAX>
{
    type IntoIter = std::collections::btree_map::IterMut<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt mut V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
impl<K: Ord, V, const MAX: usize> Default for BoundedBTreeMap<K, V, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<K: Ord, V, const MAX: usize> From<[(K, V); 0]> for BoundedBTreeMap<K, V, MAX> {
    fn from(value: [(K, V); 0]) -> Self {
        let _: [(K, V); 0] = value;
        Self(std::collections::BTreeMap::new())
    }
}
impl<K: Ord, V, const MAX: usize> TryFrom<std::collections::BTreeMap<K, V>>
    for BoundedBTreeMap<K, V, MAX>
{
    type Error = crate::bounded_value_error::BoundedValueError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        crate::validate_len::validate_len::<0, MAX>(crate::bounded_len::BoundedLen::from(
            value.len(),
        ))
        .map(|()| Self(value))
    }
}
impl<K: Ord + serde::Serialize, V: serde::Serialize, const MAX: usize> serde::Serialize
    for BoundedBTreeMap<K, V, MAX>
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
impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::Deserialize<'de> for BoundedBTreeMap<K, V, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(
            super::bounded_b_tree_map_visitor_phantom_data::BoundedBTreeMapVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_btree_map_preserves_key_order() {
        let value =
            super::BoundedBTreeMap::<u8, u8, 2>::try_from(std::collections::BTreeMap::from([
                (2u8, 2u8),
                (1u8, 1u8),
            ]))
            .expect(constants_str::DIAGNOSTIC_C51F09BE);
        assert_eq!(
            value.as_map().keys().copied().collect::<Vec<_>>(),
            [1u8, 2u8]
        );
    }
}
