#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "each bounded type keeps its inherent and trait implementations adjacent"
)]
mod btree_map;
mod hash_map;
mod string;
mod vector;
pub use btree_map::StdBoundedBTreeMap;
pub use hash_map::StdBoundedHashMap;
pub use string::BoundedString;
pub use vector::BoundedVec;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
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

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
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

fn deserialize_bounded_map<'de, Map, Key, Value, Values, Insert, const MAX: usize>(
    mut map: Map,
    mut values: Values,
    mut insert: Insert,
) -> Result<Values, Map::Error>
where
    Map: serde::de::MapAccess<'de>,
    Key: serde::Deserialize<'de>,
    Value: serde::Deserialize<'de>,
    Insert: FnMut(&mut Values, Key, Value) -> Result<(), BoundedValueError>,
{
    let mut entry_count = usize_constants::ZERO;
    loop {
        if entry_count == MAX {
            return map.next_key::<serde::de::IgnoredAny>()?.map_or_else(
                || Ok(values),
                |_ignored| {
                    Err(serde::de::Error::custom(BoundedValueError::AboveMax {
                        actual: BoundedLen::from(MAX.saturating_add(usize_constants::ONE)),
                        max: BoundedLen::from(MAX),
                    }))
                },
            );
        }
        let Some(key) = map.next_key()? else {
            return Ok(values);
        };
        let value = map.next_value()?;
        insert(&mut values, key, value).map_err(serde::de::Error::custom)?;
        entry_count = entry_count.saturating_add(usize_constants::ONE);
    }
}

const SERDE_PREALLOC_MAX_ITEMS: usize = 1024usize;

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
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
            .expect("6f09ad52 string_bounds_are_inclusive invariant must hold");
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
                actual: super::BoundedLen::from(usize_constants::ZERO),
                min: super::BoundedLen::from(usize_constants::ONE),
            }
        );
        assert_eq!(
            super::BoundedString::<2, 1>::try_from(str_constants::A.to_owned())
                .expect_err("2de961c6"),
            super::BoundedValueError::InvalidBounds {
                min: super::BoundedLen::from(2usize),
                max: super::BoundedLen::from(usize_constants::ONE),
            }
        );
    }

    #[test]
    fn byte_string_bounds_count_utf8_bytes() {
        let unicode = String::from_utf8(vec![0xc3u8, 0xa9u8, 0xc3u8, 0xa9u8])
            .expect("9167aed1 byte_string_bounds_count_utf8_bytes invariant must hold");
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
        let extensions = object
            .extensions
            .expect("177a114d byte_string_schema_publishes_byte_extensions invariant must hold");
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
        let extensions = object.extensions.expect(
            "803cfa80 unbounded_byte_string_schema_omits_max_bytes_extension invariant must hold",
        );
        assert!(extensions.contains_key(str_constants::OPENAPI_MIN_BYTES_EXTENSION));
        assert!(!extensions.contains_key(str_constants::OPENAPI_MAX_BYTES_EXTENSION));
    }

    #[test]
    fn vec_bounds_and_growth_are_enforced() {
        let mut values = super::BoundedVec::<u8, 0, 1>::try_from(Vec::new())
            .expect("cb18bc21 vec_bounds_and_growth_are_enforced invariant must hold");
        values
            .try_push(1u8)
            .expect("28f49231 vec_bounds_and_growth_are_enforced invariant must hold");
        assert_eq!(values.as_slice(), &[1u8]);
        assert_above_max(
            values.try_push(2u8).expect_err("9a1c5ee4"),
            2usize,
            usize_constants::ONE,
        );
        assert_eq!(values.into_inner(), vec![1u8]);
    }

    #[test]
    fn vec_rejects_below_minimum_and_invalid_bounds() {
        assert_eq!(
            super::BoundedVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("8bf60687"),
            super::BoundedValueError::BelowMin {
                actual: super::BoundedLen::from(usize_constants::ZERO),
                min: super::BoundedLen::from(usize_constants::ONE),
            }
        );
        assert_eq!(
            super::BoundedVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("7e536e25"),
            super::BoundedValueError::InvalidBounds {
                min: super::BoundedLen::from(2usize),
                max: super::BoundedLen::from(usize_constants::ONE),
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
                .expect(
                    "ea1fdc07 btree_map_replacement_is_allowed_at_capacity invariant must hold",
                );
        let _previous = values
            .try_insert(1u8, 2u8)
            .expect("285278fe btree_map_replacement_is_allowed_at_capacity invariant must hold");
        assert_eq!(
            values.try_insert(1u8, 3u8).expect(
                "946eb9a8 btree_map_replacement_is_allowed_at_capacity invariant must hold"
            ),
            Some(2u8)
        );
        assert_above_max(
            values.try_insert(2u8, 4u8).expect_err("e14a5d23"),
            2usize,
            usize_constants::ONE,
        );
    }

    #[test]
    fn hash_map_capacity_mutation_and_removal_are_enforced() {
        let mut values = super::StdBoundedHashMap::<u8, u8, 1>::default();
        assert_eq!(
            values.try_insert(1u8, 2u8).expect(
                "c1b15ee9 hash_map_capacity_mutation_and_removal_are_enforced invariant must hold"
            ),
            None
        );
        assert_eq!(
            values.try_insert(1u8, 3u8).expect(
                "b4e85208 hash_map_capacity_mutation_and_removal_are_enforced invariant must hold"
            ),
            Some(2u8)
        );
        values.get_mut(&1u8).map(|value| *value = 4u8).expect(
            "32578cec hash_map_capacity_mutation_and_removal_are_enforced invariant must hold",
        );
        assert_eq!(values.get(&1u8), Some(&4u8));
        assert_above_max(
            values.try_insert(2u8, 5u8).expect_err("3f1263eb"),
            2usize,
            usize_constants::ONE,
        );
        assert_eq!(values.remove(&1u8), Some(4u8));
        assert_eq!(
            values.try_insert(2u8, 5u8).expect(
                "98c16ca4 hash_map_capacity_mutation_and_removal_are_enforced invariant must hold"
            ),
            None
        );
    }

    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy forbids for loops"
    )]
    fn btree_map_iteration_and_pop_preserve_key_order() {
        let mut values = super::StdBoundedBTreeMap::<u8, u8, 3>::default();
        [3u8, 1u8, 2u8].into_iter().for_each(|key| {
            let _previous = values.try_insert(key, key).expect(
                "02efac64 btree_map_iteration_and_pop_preserve_key_order invariant must hold",
            );
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
            usize_constants::ONE,
        );
        let tree_values = [(1u8, 1u8), (2u8, 2u8)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_above_max(
            super::StdBoundedBTreeMap::<u8, u8, 1>::try_from(tree_values).expect_err("8c8a9759"),
            2usize,
            usize_constants::ONE,
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
        let consumed = std::cell::Cell::new(usize_constants::ZERO);
        let values = [1u8, 2u8, 3u8].into_iter().inspect(|_value| {
            consumed.set(consumed.get().saturating_add(usize_constants::ONE));
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

    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
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
        .expect("d1ce80f4 vec_deserialization_caps_untrusted_size_hint invariant must hold");
        assert_eq!(values.as_slice(), &[1u8]);
        assert!(values.allocation_capacity() <= super::SERDE_PREALLOC_MAX_ITEMS);
    }

    #[test]
    fn map_deserialization_enforces_capacity_and_allows_duplicate_replacement() {
        let duplicate_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(1u8, 2u8), (1u8, 3u8)].into_iter(),
        );
        let values = <super::StdBoundedBTreeMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            duplicate_map,
        )
        .expect("22d831a5 map_deserialization_enforces_capacity_and_allows_duplicate_replacement invariant must hold");
        assert_eq!(values.get(&1u8), Some(&3u8));

        let hash_duplicate_map =
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                [(1u8, 2u8), (1u8, 3u8)].into_iter(),
            );
        let hash_values = <super::StdBoundedHashMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            hash_duplicate_map,
        )
        .expect("75beb0a8 map_deserialization_enforces_capacity_and_allows_duplicate_replacement invariant must hold");
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
            .expect(
                "b3cda4f2 hash_map_deserialization_caps_untrusted_size_hint invariant must hold",
            );
        assert_eq!(values.get(&1u8), Some(&2u8));
        let capped_capacity =
            std::collections::HashMap::<u8, u8>::with_capacity(super::SERDE_PREALLOC_MAX_ITEMS)
                .capacity();
        assert!(values.allocation_capacity() <= capped_capacity);
    }

    #[test]
    fn unbounded_vector_schema_omits_max_items() {
        let schema = <super::BoundedVec<u8, 0, { usize::MAX }> as utoipa::PartialSchema>::schema();
        let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema
        else {
            panic!("5fb9ee86");
        };
        assert_eq!(array.min_items, Some(usize_constants::ZERO));
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
