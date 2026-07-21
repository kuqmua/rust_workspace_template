#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Display, newtype::FromInner)]
pub struct StdBoundedBTreeMapLen(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("bounded map length exceeds limit {0}")]
pub struct BoundedBTreeMapError(StdBoundedBTreeMapLen);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdBoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);

impl<K, V, const MAX: usize> StdBoundedBTreeMap<K, V, MAX> {
    #[must_use]
    pub const fn get(&self) -> &std::collections::BTreeMap<K, V> {
        &self.0
    }
}

impl<K, V, const MAX: usize> TryFrom<std::collections::BTreeMap<K, V>>
    for StdBoundedBTreeMap<K, V, MAX>
{
    type Error = BoundedBTreeMapError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        if value.len() > MAX {
            Err(BoundedBTreeMapError(StdBoundedBTreeMapLen::from(MAX)))
        } else {
            Ok(Self(value))
        }
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

struct StdBoundedBTreeMapVisitor<K, V, const MAX: usize>(std::marker::PhantomData<(K, V)>);

impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::de::Visitor<'de> for StdBoundedBTreeMapVisitor<K, V, MAX>
{
    type Value = StdBoundedBTreeMap<K, V, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a map with at most {MAX} entries")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut values = std::collections::BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            if values.len() == MAX {
                return Err(serde::de::Error::custom(BoundedBTreeMapError(
                    StdBoundedBTreeMapLen::from(MAX),
                )));
            }
            let _previous = values.insert(key, value);
        }
        Ok(StdBoundedBTreeMap(values))
    }
}

impl<'de, K: Ord + serde::Deserialize<'de>, V: serde::Deserialize<'de>, const MAX: usize>
    serde::Deserialize<'de> for StdBoundedBTreeMap<K, V, MAX>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(StdBoundedBTreeMapVisitor(std::marker::PhantomData))
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
        .expect("298b587f");
        assert_eq!(value.get().len(), 1usize);
    }
}
