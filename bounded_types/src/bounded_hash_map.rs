#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
/// A hash map with at most `MAX` retained keys.
///
/// Hash-map deserialization also accepts at most `MAX` wire entries, including repeated keys.
pub struct BoundedHashMap<K: Eq + std::hash::Hash, V, const MAX: usize>(
    std::collections::HashMap<K, V>,
);
impl<K: Eq + std::hash::Hash, V, const MAX: usize> BoundedHashMap<K, V, MAX> {
    #[cfg(test)]
    pub(crate) fn allocation_capacity(&self) -> usize {
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
    pub fn len(&self) -> crate::bounded_len::BoundedLen {
        crate::bounded_len::BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub(super) fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    pub fn try_insert(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<V>, crate::bounded_value_error::BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => Ok(Some(entry.insert(value))),
            std::collections::hash_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(crate::bounded_value_error::BoundedValueError::AboveMax {
                    actual: crate::bounded_len::BoundedLen::from(
                        MAX.saturating_add(constants_usize::ONE),
                    ),
                    max: crate::bounded_len::BoundedLen::from(MAX),
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
    for &'map_lt BoundedHashMap<K, V, MAX>
{
    type IntoIter = std::collections::hash_map::Iter<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'map_lt, K: Eq + std::hash::Hash, V, const MAX: usize> IntoIterator
    for &'map_lt mut BoundedHashMap<K, V, MAX>
{
    type IntoIter = std::collections::hash_map::IterMut<'map_lt, K, V>;
    type Item = (&'map_lt K, &'map_lt mut V);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> Default for BoundedHashMap<K, V, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> From<[(K, V); 0]> for BoundedHashMap<K, V, MAX> {
    fn from(_value: [(K, V); 0]) -> Self {
        Self(std::collections::HashMap::new())
    }
}
impl<K: Eq + std::hash::Hash, V, const MAX: usize> TryFrom<std::collections::HashMap<K, V>>
    for BoundedHashMap<K, V, MAX>
{
    type Error = crate::bounded_value_error::BoundedValueError;

    fn try_from(value: std::collections::HashMap<K, V>) -> Result<Self, Self::Error> {
        crate::validate_len::validate_len::<0, MAX>(crate::bounded_len::BoundedLen::from(
            value.len(),
        ))
        .map(|()| Self(value))
    }
}
impl<K: Eq + std::hash::Hash + serde::Serialize, V: serde::Serialize, const MAX: usize>
    serde::Serialize for BoundedHashMap<K, V, MAX>
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
impl<
    'de,
    K: Eq + std::hash::Hash + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
    const MAX: usize,
> serde::Deserialize<'de> for BoundedHashMap<K, V, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(
            super::bounded_hash_map_visitor_phantom_data::BoundedHashMapVisitorPhantomData::from(
                std::marker::PhantomData,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_map_rejects_new_keys_at_capacity() {
        let mut value = super::BoundedHashMap::<u8, u8, 1>::default();
        assert!(matches!(value.try_insert(1u8, 1u8), Ok(None)));
        assert!(matches!(
            value.try_insert(2u8, 2u8),
            Err(crate::bounded_value_error::BoundedValueError::AboveMax { .. })
        ));
    }
}
