#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::FromInner)]
pub struct BoundedLen(usize);
impl BoundedLen {
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
impl std::fmt::Display for BoundedLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BoundedValueError {
    #[error("bounded value length {actual} exceeds maximum {max}")]
    AboveMax { actual: BoundedLen, max: BoundedLen },
    #[error("bounded value length {actual} is below minimum {min}")]
    BelowMin { actual: BoundedLen, min: BoundedLen },
    #[error("bounded value minimum {min} exceeds maximum {max}")]
    InvalidBounds { min: BoundedLen, max: BoundedLen },
}

fn validate_len<const MIN: usize, const MAX: usize>(
    len: BoundedLen,
) -> Result<(), BoundedValueError> {
    if MIN > MAX {
        Err(BoundedValueError::InvalidBounds {
            min: BoundedLen::from(MIN),
            max: BoundedLen::from(MAX),
        })
    } else if len.get() < MIN {
        Err(BoundedValueError::BelowMin {
            actual: len,
            min: BoundedLen::from(MIN),
        })
    } else if len.get() > MAX {
        Err(BoundedValueError::AboveMax {
            actual: len,
            max: BoundedLen::from(MAX),
        })
    } else {
        Ok(())
    }
}

const SERDE_PREALLOC_MAX_ITEMS: usize = 1024usize;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, newtype::DerefInner)]
pub struct BoundedString<const MIN: usize, const MAX: usize>(String);
impl<const MIN: usize, const MAX: usize> BoundedString<MIN, MAX> {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> BoundedLen {
        BoundedLen::from(self.0.len())
    }
}
impl<const MIN: usize, const MAX: usize> AsRef<str> for BoundedString<MIN, MAX> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl<const MIN: usize, const MAX: usize> std::fmt::Display for BoundedString<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl<const MIN: usize, const MAX: usize> TryFrom<String> for BoundedString<MIN, MAX> {
    type Error = BoundedValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_len::<MIN, MAX>(BoundedLen::from(value.len())).map(|()| Self(value))
    }
}
impl<const MIN: usize, const MAX: usize> serde::Serialize for BoundedString<MIN, MAX> {
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
impl<'de, const MIN: usize, const MAX: usize> serde::Deserialize<'de> for BoundedString<MIN, MAX> {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl<const MIN: usize, const MAX: usize> utoipa::PartialSchema for BoundedString<MIN, MAX> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let extensions_builder = utoipa::openapi::extensions::ExtensionsBuilder::new()
            .add(str_constants::OPENAPI_MIN_BYTES_EXTENSION, MIN);
        let extensions = if MAX == usize::MAX {
            extensions_builder
        } else {
            extensions_builder.add(str_constants::OPENAPI_MAX_BYTES_EXTENSION, MAX)
        };
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .extensions(Some(extensions.build()))
            .build()
            .into()
    }
}
impl<const MIN: usize, const MAX: usize> utoipa::ToSchema for BoundedString<MIN, MAX> {}

#[derive(Clone, Debug, Eq, PartialEq, newtype::DerefTarget, newtype::IntoIterator)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
    pub fn validate_bounds() -> Result<(), BoundedValueError> {
        validate_len::<MIN, MAX>(BoundedLen::from(MIN))
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    #[must_use]
    pub fn len(&self) -> BoundedLen {
        BoundedLen::from(self.0.len())
    }

    pub fn try_push(&mut self, value: T) -> Result<(), BoundedValueError> {
        let next_len =
            self.0
                .len()
                .checked_add(1usize)
                .ok_or_else(|| BoundedValueError::AboveMax {
                    actual: BoundedLen::from(usize::MAX),
                    max: BoundedLen::from(MAX),
                })?;
        validate_len::<0, MAX>(BoundedLen::from(next_len)).map(|()| self.0.push(value))
    }
}
impl<T> BoundedVec<T, 0, { usize::MAX }> {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = T>,
    {
        let mut bounded = Self::default();
        values
            .into_iter()
            .for_each(|value| bounded.push_max_capacity(value));
        bounded
    }

    pub fn push_max_capacity(&mut self, value: T) {
        self.0.push(value);
    }
}
impl<T, const MAX: usize> Default for BoundedVec<T, 0, MAX> {
    fn default() -> Self {
        Self::from([])
    }
}
impl<T, const MAX: usize> From<[T; 0]> for BoundedVec<T, 0, MAX> {
    fn from(_value: [T; 0]) -> Self {
        Self(Vec::new())
    }
}
impl<T, const MIN: usize, const MAX: usize> TryFrom<Vec<T>> for BoundedVec<T, MIN, MAX> {
    type Error = BoundedValueError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        validate_len::<MIN, MAX>(BoundedLen::from(value.len())).map(|()| Self(value))
    }
}
impl<T, const MIN: usize, const MAX: usize> AsRef<[T]> for BoundedVec<T, MIN, MAX> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T: serde::Serialize, const MIN: usize, const MAX: usize> serde::Serialize
    for BoundedVec<T, MIN, MAX>
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
#[derive(newtype::FromInner)]
struct StdPhantomDataBoundedVecVisitor<T, const MIN: usize, const MAX: usize>(
    std::marker::PhantomData<T>,
);
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::de::Visitor<'de>
    for StdPhantomDataBoundedVecVisitor<T, MIN, MAX>
{
    type Value = BoundedVec<T, MIN, MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "an array with {MIN} to {MAX} items")
    }

    fn visit_seq<Seq>(self, mut seq: Seq) -> Result<Self::Value, Seq::Error>
    where
        Seq: serde::de::SeqAccess<'de>,
    {
        BoundedVec::<T, MIN, MAX>::validate_bounds().map_err(serde::de::Error::custom)?;
        let capacity = seq
            .size_hint()
            .unwrap_or(MIN)
            .min(MAX)
            .min(SERDE_PREALLOC_MAX_ITEMS);
        let mut values = Vec::with_capacity(capacity);
        loop {
            if values.len() == MAX {
                return seq.next_element::<serde::de::IgnoredAny>()?.map_or_else(
                    || BoundedVec::try_from(values).map_err(serde::de::Error::custom),
                    |_ignored| {
                        Err(serde::de::Error::custom(BoundedValueError::AboveMax {
                            actual: BoundedLen::from(MAX.saturating_add(1usize)),
                            max: BoundedLen::from(MAX),
                        }))
                    },
                );
            }
            match seq.next_element()? {
                Some(value) => values.push(value),
                None => {
                    return BoundedVec::try_from(values).map_err(serde::de::Error::custom);
                }
            }
        }
    }
}
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for BoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(StdPhantomDataBoundedVecVisitor::from(
            std::marker::PhantomData,
        ))
    }
}
impl<T: utoipa::PartialSchema, const MIN: usize, const MAX: usize> utoipa::__dev::ComposeSchema
    for BoundedVec<T, MIN, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        let builder = utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(MIN));
        if MAX == usize::MAX {
            builder.build().into()
        } else {
            builder.max_items(Some(MAX)).build().into()
        }
    }
}
impl<T: utoipa::ToSchema, const MIN: usize, const MAX: usize> utoipa::ToSchema
    for BoundedVec<T, MIN, MAX>
{
    fn name() -> std::borrow::Cow<'static, str> {
        let mut name = T::name().into_owned();
        name.push('_');
        name.push_str(str_constants::BOUNDEDVEC);
        name.push('_');
        name.push_str(MIN.to_string().as_str());
        name.push('_');
        name.push_str(MAX.to_string().as_str());
        std::borrow::Cow::Owned(name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// A hash map with at most `MAX` retained keys.
///
/// Hash-map deserialization also accepts at most `MAX` wire entries, including repeated keys.
pub struct StdBoundedHashMap<K: Eq + std::hash::Hash, V, const MAX: usize>(
    std::collections::HashMap<K, V>,
);
impl<K: Eq + std::hash::Hash, V, const MAX: usize> StdBoundedHashMap<K, V, MAX> {
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
    pub fn len(&self) -> BoundedLen {
        BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => Ok(Some(entry.insert(value))),
            std::collections::hash_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(BoundedValueError::AboveMax {
                    actual: BoundedLen::from(MAX.saturating_add(1usize)),
                    max: BoundedLen::from(MAX),
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
    type Error = BoundedValueError;

    fn try_from(value: std::collections::HashMap<K, V>) -> Result<Self, Self::Error> {
        validate_len::<0, MAX>(BoundedLen::from(value.len())).map(|()| Self(value))
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
#[derive(newtype::FromInner)]
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

    fn visit_map<Map>(self, mut map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        let capacity = map
            .size_hint()
            .unwrap_or(0usize)
            .min(MAX)
            .min(SERDE_PREALLOC_MAX_ITEMS);
        let mut values = StdBoundedHashMap::default();
        values.0.reserve(capacity);
        let mut entry_count = 0usize;
        loop {
            if entry_count == MAX {
                return map.next_key::<serde::de::IgnoredAny>()?.map_or_else(
                    || Ok(values),
                    |_ignored| {
                        Err(serde::de::Error::custom(BoundedValueError::AboveMax {
                            actual: BoundedLen::from(MAX.saturating_add(1usize)),
                            max: BoundedLen::from(MAX),
                        }))
                    },
                );
            }
            let Some(key) = map.next_key()? else {
                return Ok(values);
            };
            let value = map.next_value()?;
            let _previous = values
                .try_insert(key, value)
                .map_err(serde::de::Error::custom)?;
            entry_count = entry_count.saturating_add(1usize);
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn len(&self) -> BoundedLen {
        BoundedLen::from(self.0.len())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        self.0.pop_first()
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, BoundedValueError> {
        let is_full = self.0.len() >= MAX;
        match self.0.entry(key) {
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                Ok(Some(entry.insert(value)))
            }
            std::collections::btree_map::Entry::Vacant(entry) if is_full => {
                drop(entry);
                Err(BoundedValueError::AboveMax {
                    actual: BoundedLen::from(MAX.saturating_add(1usize)),
                    max: BoundedLen::from(MAX),
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
    type Error = BoundedValueError;

    fn try_from(value: std::collections::BTreeMap<K, V>) -> Result<Self, Self::Error> {
        validate_len::<0, MAX>(BoundedLen::from(value.len())).map(|()| Self(value))
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
#[derive(newtype::FromInner)]
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

    fn visit_map<Map>(self, mut map: Map) -> Result<Self::Value, Map::Error>
    where
        Map: serde::de::MapAccess<'de>,
    {
        let mut values = StdBoundedBTreeMap::default();
        let mut entry_count = 0usize;
        loop {
            if entry_count == MAX {
                return map.next_key::<serde::de::IgnoredAny>()?.map_or_else(
                    || Ok(values),
                    |_ignored| {
                        Err(serde::de::Error::custom(BoundedValueError::AboveMax {
                            actual: BoundedLen::from(MAX.saturating_add(1usize)),
                            max: BoundedLen::from(MAX),
                        }))
                    },
                );
            }
            let Some(key) = map.next_key()? else {
                return Ok(values);
            };
            let value = map.next_value()?;
            let _previous = values
                .try_insert(key, value)
                .map_err(serde::de::Error::custom)?;
            entry_count = entry_count.saturating_add(1usize);
        }
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
    enum TestDeserializerValue {
        Number(u8),
        Text(&'static str),
    }
    impl serde::de::IntoDeserializer<'_, serde::de::value::Error> for TestDeserializerValue {
        type Deserializer = Self;

        fn into_deserializer(self) -> Self::Deserializer {
            self
        }
    }
    impl<'de> serde::Deserializer<'de> for TestDeserializerValue {
        type Error = serde::de::value::Error;

        fn deserialize_any<Visitor>(self, visitor: Visitor) -> Result<Visitor::Value, Self::Error>
        where
            Visitor: serde::de::Visitor<'de>,
        {
            match self {
                Self::Number(value) => visitor.visit_u8(value),
                Self::Text(value) => visitor.visit_borrowed_str(value),
            }
        }

        fn deserialize_ignored_any<Visitor>(
            self,
            visitor: Visitor,
        ) -> Result<Visitor::Value, Self::Error>
        where
            Visitor: serde::de::Visitor<'de>,
        {
            visitor.visit_unit()
        }

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
            byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct
            enum identifier
        }
    }

    fn assert_above_max(error: super::BoundedValueError, actual: usize, max: usize) {
        assert_eq!(
            error,
            super::BoundedValueError::AboveMax {
                actual: super::BoundedLen::from(actual),
                max: super::BoundedLen::from(max),
            }
        );
    }

    #[test]
    fn string_bounds_are_inclusive() {
        let value = super::BoundedString::<1, 3>::try_from(str_constants::ABC_ALT_3.to_owned())
            .expect("6f09ad52");
        assert_eq!(value.as_ref(), str_constants::ABC_ALT_3);
        assert_eq!(value.len().get(), 3usize);
        assert_above_max(
            super::BoundedString::<1, 2>::try_from(str_constants::ABC_ALT_3.to_owned())
                .expect_err("99e3065c"),
            3usize,
            2usize,
        );
    }

    #[test]
    fn string_rejects_below_minimum_and_invalid_bounds() {
        assert_eq!(
            super::BoundedString::<1, 3>::try_from(String::new()).expect_err("0ef05b85"),
            super::BoundedValueError::BelowMin {
                actual: super::BoundedLen::from(0usize),
                min: super::BoundedLen::from(1usize),
            }
        );
        assert_eq!(
            super::BoundedString::<2, 1>::try_from(str_constants::A.to_owned())
                .expect_err("2de961c6"),
            super::BoundedValueError::InvalidBounds {
                min: super::BoundedLen::from(2usize),
                max: super::BoundedLen::from(1usize),
            }
        );
    }

    #[test]
    fn byte_string_bounds_count_utf8_bytes() {
        let unicode = String::from_utf8(vec![0xc3u8, 0xa9u8, 0xc3u8, 0xa9u8]).expect("9167aed1");
        assert_above_max(
            super::BoundedString::<0, 2>::try_from(unicode).expect_err("9fd40773"),
            4usize,
            2usize,
        );
    }

    #[test]
    fn byte_string_schema_publishes_byte_extensions() {
        let schema = <super::BoundedString<1, 4> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema
        else {
            panic!("43ea6e9b");
        };
        let extensions = object.extensions.expect("177a114d");
        assert_eq!(
            extensions
                .get(str_constants::OPENAPI_MIN_BYTES_EXTENSION)
                .and_then(utoipa::r#gen::serde_json::value::Value::as_u64),
            Some(1u64)
        );
        assert_eq!(
            extensions
                .get(str_constants::OPENAPI_MAX_BYTES_EXTENSION)
                .and_then(utoipa::r#gen::serde_json::value::Value::as_u64),
            Some(4u64)
        );
        assert_eq!(object.min_length, None);
        assert_eq!(object.max_length, None);
    }

    #[test]
    fn unbounded_byte_string_schema_omits_max_bytes_extension() {
        let schema = <super::BoundedString<1, { usize::MAX }> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema
        else {
            panic!("43fbea64");
        };
        let extensions = object.extensions.expect("803cfa80");
        assert!(extensions.contains_key(str_constants::OPENAPI_MIN_BYTES_EXTENSION));
        assert!(!extensions.contains_key(str_constants::OPENAPI_MAX_BYTES_EXTENSION));
    }

    #[test]
    fn vec_bounds_and_growth_are_enforced() {
        let mut values = super::BoundedVec::<u8, 0, 1>::try_from(Vec::new()).expect("cb18bc21");
        values.try_push(1u8).expect("28f49231");
        assert_eq!(values.as_slice(), &[1u8]);
        assert_above_max(values.try_push(2u8).expect_err("9a1c5ee4"), 2usize, 1usize);
        assert_eq!(values.into_inner(), vec![1u8]);
    }

    #[test]
    fn vec_rejects_below_minimum_and_invalid_bounds() {
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("8bf60687"),
            super::BoundedValueError::BelowMin {
                actual: super::BoundedLen::from(0usize),
                min: super::BoundedLen::from(1usize),
            }
        );
        assert_eq!(
            super::BoundedVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("7e536e25"),
            super::BoundedValueError::InvalidBounds {
                min: super::BoundedLen::from(2usize),
                max: super::BoundedLen::from(1usize),
            }
        );
    }

    #[test]
    fn max_vec_construction_preserves_order_and_supports_consuming_iteration() {
        let values = super::BoundedVec::<u8, 0, { usize::MAX }>::from_max_iter([3u8, 1u8, 2u8]);
        assert_eq!(values.len().get(), 3usize);
        assert_eq!(values.into_iter().collect::<Vec<u8>>(), vec![3u8, 1u8, 2u8]);
    }

    #[test]
    fn btree_map_replacement_is_allowed_at_capacity() {
        let mut values =
            super::StdBoundedBTreeMap::<u8, u8, 1>::try_from(std::collections::BTreeMap::new())
                .expect("ea1fdc07");
        let _previous = values.try_insert(1u8, 2u8).expect("285278fe");
        assert_eq!(values.try_insert(1u8, 3u8).expect("946eb9a8"), Some(2u8));
        assert_above_max(
            values.try_insert(2u8, 4u8).expect_err("e14a5d23"),
            2usize,
            1usize,
        );
    }

    #[test]
    fn hash_map_capacity_mutation_and_removal_are_enforced() {
        let mut values = super::StdBoundedHashMap::<u8, u8, 1>::default();
        assert_eq!(values.try_insert(1u8, 2u8).expect("c1b15ee9"), None);
        assert_eq!(values.try_insert(1u8, 3u8).expect("b4e85208"), Some(2u8));
        values
            .get_mut(&1u8)
            .map(|value| *value = 4u8)
            .expect("32578cec");
        assert_eq!(values.get(&1u8), Some(&4u8));
        assert_above_max(
            values.try_insert(2u8, 5u8).expect_err("3f1263eb"),
            2usize,
            1usize,
        );
        assert_eq!(values.remove(&1u8), Some(4u8));
        assert_eq!(values.try_insert(2u8, 5u8).expect("98c16ca4"), None);
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy forbids for loops"
    )]
    fn btree_map_iteration_and_pop_preserve_key_order() {
        let mut values = super::StdBoundedBTreeMap::<u8, u8, 3>::default();
        [3u8, 1u8, 2u8].into_iter().for_each(|key| {
            let _previous = values.try_insert(key, key).expect("02efac64");
        });
        values.iter_mut().for_each(|(_key, value)| {
            *value = value.saturating_add(10u8);
        });
        assert_eq!(
            values
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect::<Vec<_>>(),
            vec![(1u8, 11u8), (2u8, 12u8), (3u8, 13u8)]
        );
        assert_eq!(values.pop_first(), Some((1u8, 11u8)));
        assert_eq!(values.into_values().collect::<Vec<u8>>(), vec![12u8, 13u8]);
    }

    #[test]
    fn raw_map_conversions_reject_values_above_capacity() {
        let hash_values = [(1u8, 1u8), (2u8, 2u8)]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>();
        assert_above_max(
            super::StdBoundedHashMap::<u8, u8, 1>::try_from(hash_values).expect_err("5c0d1871"),
            2usize,
            1usize,
        );
        let tree_values = [(1u8, 1u8), (2u8, 2u8)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_above_max(
            super::StdBoundedBTreeMap::<u8, u8, 1>::try_from(tree_values).expect_err("8c8a9759"),
            2usize,
            1usize,
        );
    }

    #[test]
    fn serde_rejects_string_and_vec_values_outside_bounds() {
        let vec_result = <super::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [1u8, 2u8].into_iter(),
            ),
        );
        assert!(matches!(vec_result, Err(serde::de::value::Error { .. })));
        let string_result = <super::BoundedString<2, 3> as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
        );
        assert!(matches!(string_result, Err(serde::de::value::Error { .. })));
    }

    #[test]
    fn vec_deserialization_reports_lower_and_invalid_bounds() {
        let below_min = <super::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                std::iter::empty::<u8>(),
            ),
        )
        .expect_err("6769c946");
        assert!(below_min.to_string().contains("below minimum 1"));

        let invalid = <super::BoundedVec<u8, 2, 1> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                std::iter::empty::<u8>(),
            ),
        )
        .expect_err("a0c71f21");
        assert!(invalid.to_string().contains("minimum 2 exceeds maximum 1"));
    }

    #[test]
    fn zero_capacity_vec_rejects_without_deserializing_item_type() {
        let error = <super::BoundedVec<u8, 0, 0> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [TestDeserializerValue::Text(str_constants::UNKNOWN)].into_iter(),
            ),
        )
        .expect_err("c80ad225");
        assert!(error.to_string().contains("exceeds maximum 0"));
    }

    #[test]
    fn vec_deserialization_stops_after_first_excess_item() {
        let consumed = std::cell::Cell::new(0usize);
        let values = [1u8, 2u8, 3u8].into_iter().inspect(|_value| {
            consumed.set(consumed.get().saturating_add(1usize));
        });
        let result = <super::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(values),
        );
        let _error = result.expect_err("505efc76");
        assert_eq!(consumed.get(), 2usize);
    }

    #[test]
    fn vec_deserialization_ignores_excess_item_type() {
        let error = <super::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                [
                    TestDeserializerValue::Number(1u8),
                    TestDeserializerValue::Text(str_constants::UNKNOWN),
                ]
                .into_iter(),
            ),
        )
        .expect_err("4b556495");
        assert!(error.to_string().contains("exceeds maximum 1"));
    }

    struct MisleadingSizeHintIter<Value> {
        values: std::vec::IntoIter<Value>,
    }
    impl<Value> Iterator for MisleadingSizeHintIter<Value> {
        type Item = Value;

        fn next(&mut self) -> Option<Self::Item> {
            self.values.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX, Some(usize::MAX))
        }
    }

    #[test]
    fn vec_deserialization_caps_untrusted_size_hint() {
        let values = <super::BoundedVec<u8, 0, { usize::MAX }> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                MisleadingSizeHintIter {
                    values: vec![1u8].into_iter(),
                },
            ),
        )
        .expect("d1ce80f4");
        assert_eq!(values.as_slice(), &[1u8]);
        assert!(values.0.capacity() <= super::SERDE_PREALLOC_MAX_ITEMS);
    }

    #[test]
    fn map_deserialization_enforces_capacity_and_allows_duplicate_replacement() {
        let duplicate_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(1u8, 2u8), (1u8, 3u8)].into_iter(),
        );
        let values = <super::StdBoundedBTreeMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            duplicate_map,
        )
        .expect("22d831a5");
        assert_eq!(values.get(&1u8), Some(&3u8));

        let hash_duplicate_map =
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                [(1u8, 2u8), (1u8, 3u8)].into_iter(),
            );
        let hash_values = <super::StdBoundedHashMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            hash_duplicate_map,
        )
        .expect("75beb0a8");
        assert_eq!(hash_values.get(&1u8), Some(&3u8));

        let duplicate_above_wire_limit = serde::de::value::MapDeserializer::<
            _,
            serde::de::value::Error,
        >::new([(1u8, 2u8), (1u8, 3u8)].into_iter());
        let duplicate_result =
            <super::StdBoundedBTreeMap<u8, u8, 1> as serde::Deserialize>::deserialize(
                duplicate_above_wire_limit,
            );
        let _error = duplicate_result.expect_err("ace97816");

        let distinct_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(1u8, 2u8), (2u8, 3u8)].into_iter(),
        );
        let result =
            <super::StdBoundedHashMap<u8, u8, 1> as serde::Deserialize>::deserialize(distinct_map);
        assert!(matches!(result, Err(serde::de::value::Error { .. })));
    }

    #[test]
    fn map_deserialization_bounds_wire_entries_before_excess_value() {
        let tree_entries = [
            (
                TestDeserializerValue::Text(str_constants::A),
                TestDeserializerValue::Number(1u8),
            ),
            (
                TestDeserializerValue::Number(2u8),
                TestDeserializerValue::Text(str_constants::UNKNOWN),
            ),
        ];
        let tree_error =
            <super::StdBoundedBTreeMap<String, u8, 1> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    tree_entries.into_iter(),
                ),
            )
            .expect_err("159266eb");
        assert!(tree_error.to_string().contains("exceeds maximum 1"));

        let hash_entries = [
            (
                TestDeserializerValue::Text(str_constants::A),
                TestDeserializerValue::Number(1u8),
            ),
            (
                TestDeserializerValue::Number(2u8),
                TestDeserializerValue::Text(str_constants::UNKNOWN),
            ),
        ];
        let hash_error =
            <super::StdBoundedHashMap<String, u8, 1> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    hash_entries.into_iter(),
                ),
            )
            .expect_err("a894f87e");
        assert!(hash_error.to_string().contains("exceeds maximum 1"));
    }

    #[test]
    fn zero_capacity_maps_reject_without_deserializing_key_or_value_types() {
        let tree_entries = [(
            TestDeserializerValue::Number(1u8),
            TestDeserializerValue::Text(str_constants::UNKNOWN),
        )];
        let tree_error =
            <super::StdBoundedBTreeMap<String, u8, 0> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    tree_entries.into_iter(),
                ),
            )
            .expect_err("51d4fb77");
        assert!(tree_error.to_string().contains("exceeds maximum 0"));

        let hash_entries = [(
            TestDeserializerValue::Number(1u8),
            TestDeserializerValue::Text(str_constants::UNKNOWN),
        )];
        let hash_error =
            <super::StdBoundedHashMap<String, u8, 0> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    hash_entries.into_iter(),
                ),
            )
            .expect_err("cf7fb56d");
        assert!(hash_error.to_string().contains("exceeds maximum 0"));
    }

    #[test]
    fn hash_map_deserialization_caps_untrusted_size_hint() {
        let entries = MisleadingSizeHintIter {
            values: vec![(1u8, 2u8)].into_iter(),
        };
        let values =
            <super::StdBoundedHashMap<u8, u8, { usize::MAX }> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(entries),
            )
            .expect("b3cda4f2");
        assert_eq!(values.get(&1u8), Some(&2u8));
        let capped_capacity =
            std::collections::HashMap::<u8, u8>::with_capacity(super::SERDE_PREALLOC_MAX_ITEMS)
                .capacity();
        assert!(values.0.capacity() <= capped_capacity);
    }

    #[test]
    fn unbounded_vector_schema_omits_max_items() {
        let schema = <super::BoundedVec<u8, 0, { usize::MAX }> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema
        else {
            panic!("5fb9ee86");
        };
        assert_eq!(array.min_items, Some(0usize));
        assert_eq!(array.max_items, None);
    }

    #[test]
    fn vector_schema_names_include_item_type_and_bounds() {
        let first = <super::BoundedVec<u8, 0, 1> as utoipa::ToSchema>::name();
        let second = <super::BoundedVec<u16, 1, 2> as utoipa::ToSchema>::name();
        assert_ne!(first, second);
        assert!(first.contains(str_constants::BOUNDEDVEC));
        assert!(second.contains(str_constants::BOUNDEDVEC));
    }
}
