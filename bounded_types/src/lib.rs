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
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(MIN))
            .max_length(Some(MAX))
            .build()
            .into()
    }
}
impl<const MIN: usize, const MAX: usize> utoipa::ToSchema for BoundedString<MIN, MAX> {}

#[derive(Clone, Debug, Eq, PartialEq, newtype::DerefTarget, newtype::IntoIterator)]
pub struct BoundedVec<T, const MIN: usize, const MAX: usize>(Vec<T>);
impl<T, const MIN: usize, const MAX: usize> BoundedVec<T, MIN, MAX> {
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
impl<'de, T: serde::Deserialize<'de>, const MIN: usize, const MAX: usize> serde::Deserialize<'de>
    for BoundedVec<T, MIN, MAX>
{
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let values = <Vec<T> as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(values).map_err(serde::de::Error::custom)
    }
}
impl<T: utoipa::PartialSchema, const MIN: usize, const MAX: usize> utoipa::__dev::ComposeSchema
    for BoundedVec<T, MIN, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .min_items(Some(MIN))
            .max_items(Some(MAX))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema, const MIN: usize, const MAX: usize> utoipa::ToSchema
    for BoundedVec<T, MIN, MAX>
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(str_constants::BOUNDEDVEC)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdBoundedHashMap<K: Eq + std::hash::Hash, V, const MAX: usize>(
    std::collections::HashMap<K, V>,
);
impl<K: Eq + std::hash::Hash, V, const MAX: usize> StdBoundedHashMap<K, V, MAX> {
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
        if !self.0.contains_key(&key) && self.0.len() == MAX {
            Err(BoundedValueError::AboveMax {
                actual: BoundedLen::from(MAX.saturating_add(1usize)),
                max: BoundedLen::from(MAX),
            })
        } else {
            Ok(self.0.insert(key, value))
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StdBoundedBTreeMap<K, V, const MAX: usize>(std::collections::BTreeMap<K, V>);
impl<K: Ord, V, const MAX: usize> StdBoundedBTreeMap<K, V, MAX> {
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
        if !self.0.contains_key(&key) && self.0.len() == MAX {
            Err(BoundedValueError::AboveMax {
                actual: BoundedLen::from(MAX.saturating_add(1usize)),
                max: BoundedLen::from(MAX),
            })
        } else {
            Ok(self.0.insert(key, value))
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

#[cfg(test)]
mod tests {
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
}
