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
    let value =
        super::bounded_string::BoundedString::<1, 3>::try_from(constants_str::ABC_ALT_3.to_owned())
            .expect("6f09ad52 string_bounds_are_inclusive invariant must hold");
    assert_eq!(value.as_ref(), constants_str::ABC_ALT_3);
    assert_eq!(value.len().get(), 3usize);
    assert_above_max(
        super::bounded_string::BoundedString::<1, 2>::try_from(constants_str::ABC_ALT_3.to_owned())
            .expect_err(constants_str::VALUE_E4A5AF09),
        3usize,
        2usize,
    );
}

#[test]
fn string_rejects_below_minimum_and_invalid_bounds() {
    assert_eq!(
        super::bounded_string::BoundedString::<1, 3>::try_from(String::new())
            .expect_err("0ef05b85"),
        super::BoundedValueError::BelowMin {
            actual: super::BoundedLen::from(constants_usize::ZERO),
            min: super::BoundedLen::from(constants_usize::ONE),
        }
    );
    assert_eq!(
        super::bounded_string::BoundedString::<2, 1>::try_from(constants_str::A.to_owned())
            .expect_err("2de961c6"),
        super::BoundedValueError::InvalidBounds {
            min: super::BoundedLen::from(2usize),
            max: super::BoundedLen::from(constants_usize::ONE),
        }
    );
}

#[test]
fn byte_string_bounds_count_utf8_bytes() {
    let unicode = String::from_utf8(vec![0xc3u8, 0xa9u8, 0xc3u8, 0xa9u8])
        .expect("9167aed1 byte_string_bounds_count_utf8_bytes invariant must hold");
    assert_above_max(
        super::bounded_string::BoundedString::<0, 2>::try_from(unicode)
            .expect_err(constants_str::VALUE_311B8C86),
        4usize,
        2usize,
    );
}

#[test]
fn byte_string_schema_publishes_byte_extensions() {
    let schema = <super::bounded_string::BoundedString<1, 4> as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema else {
        panic!("43ea6e9b");
    };
    let extensions = object
        .extensions
        .expect("177a114d byte_string_schema_publishes_byte_extensions invariant must hold");
    assert_eq!(
        extensions
            .get(constants_str::OPENAPI_MIN_BYTES_EXTENSION)
            .and_then(utoipa::r#gen::serde_json::value::Value::as_u64),
        Some(1u64)
    );
    assert_eq!(
        extensions
            .get(constants_str::OPENAPI_MAX_BYTES_EXTENSION)
            .and_then(utoipa::r#gen::serde_json::value::Value::as_u64),
        Some(4u64)
    );
    assert_eq!(object.min_length, None);
    assert_eq!(object.max_length, None);
}

#[test]
fn unbounded_byte_string_schema_omits_max_bytes_extension() {
    let schema =
        <super::bounded_string::BoundedString<1, { usize::MAX }> as utoipa::PartialSchema>::schema(
        );
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema else {
        panic!("43fbea64");
    };
    let extensions = object.extensions.expect(
        "803cfa80 unbounded_byte_string_schema_omits_max_bytes_extension invariant must hold",
    );
    assert!(extensions.contains_key(constants_str::OPENAPI_MIN_BYTES_EXTENSION));
    assert!(!extensions.contains_key(constants_str::OPENAPI_MAX_BYTES_EXTENSION));
}

#[test]
fn vec_bounds_and_growth_are_enforced() {
    let mut values = super::vector::BoundedVec::<u8, 0, 1>::try_from(Vec::new())
        .expect("cb18bc21 vec_bounds_and_growth_are_enforced invariant must hold");
    values
        .try_push(1u8)
        .expect("28f49231 vec_bounds_and_growth_are_enforced invariant must hold");
    assert_eq!(values.as_slice(), &[1u8]);
    assert_above_max(
        values
            .try_push(2u8)
            .expect_err(constants_str::VALUE_F2921AC3),
        2usize,
        constants_usize::ONE,
    );
    assert_eq!(values.into_inner(), vec![1u8]);
}

#[test]
fn vec_rejects_below_minimum_and_invalid_bounds() {
    assert_eq!(
        super::vector::BoundedVec::<u8, 1, 2>::try_from(Vec::new()).expect_err("8bf60687"),
        super::BoundedValueError::BelowMin {
            actual: super::BoundedLen::from(constants_usize::ZERO),
            min: super::BoundedLen::from(constants_usize::ONE),
        }
    );
    assert_eq!(
        super::vector::BoundedVec::<u8, 2, 1>::try_from(vec![1u8]).expect_err("7e536e25"),
        super::BoundedValueError::InvalidBounds {
            min: super::BoundedLen::from(2usize),
            max: super::BoundedLen::from(constants_usize::ONE),
        }
    );
}

#[test]
fn max_vec_construction_preserves_order_and_supports_consuming_iteration() {
    let values = super::vector::BoundedVec::<u8, 0, { usize::MAX }>::from_max_iter([3u8, 1u8, 2u8]);
    assert_eq!(values.len().get(), 3usize);
    assert_eq!(values.into_iter().collect::<Vec<u8>>(), vec![3u8, 1u8, 2u8]);
}

#[test]
fn btree_map_replacement_is_allowed_at_capacity() {
    let mut values =
        super::btree::BoundedBTreeMap::<u8, u8, 1>::try_from(std::collections::BTreeMap::new())
            .expect("ea1fdc07 btree_map_replacement_is_allowed_at_capacity invariant must hold");
    let _previous = values
        .try_insert(1u8, 2u8)
        .expect("285278fe btree_map_replacement_is_allowed_at_capacity invariant must hold");
    assert_eq!(
        values
            .try_insert(1u8, 3u8)
            .expect("946eb9a8 btree_map_replacement_is_allowed_at_capacity invariant must hold"),
        Some(2u8)
    );
    assert_above_max(
        values
            .try_insert(2u8, 4u8)
            .expect_err(constants_str::VALUE_0C2A598A),
        2usize,
        constants_usize::ONE,
    );
}

#[test]
fn hash_map_capacity_mutation_and_removal_are_enforced() {
    let mut values = super::hash::BoundedHashMap::<u8, u8, 1>::default();
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
    values
        .get_mut(&1u8)
        .map(|value| *value = 4u8)
        .expect("32578cec hash_map_capacity_mutation_and_removal_are_enforced invariant must hold");
    assert_eq!(values.get(&1u8), Some(&4u8));
    assert_above_max(
        values
            .try_insert(2u8, 5u8)
            .expect_err(constants_str::VALUE_9ADBD6D0),
        2usize,
        constants_usize::ONE,
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
    let mut values = super::btree::BoundedBTreeMap::<u8, u8, 3>::default();
    [3u8, 1u8, 2u8].into_iter().for_each(|key| {
        let _previous = values
            .try_insert(key, key)
            .expect("02efac64 btree_map_iteration_and_pop_preserve_key_order invariant must hold");
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
        super::hash::BoundedHashMap::<u8, u8, 1>::try_from(hash_values)
            .expect_err(constants_str::VALUE_C531636A),
        2usize,
        constants_usize::ONE,
    );
    let tree_values = [(1u8, 1u8), (2u8, 2u8)]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_above_max(
        super::btree::BoundedBTreeMap::<u8, u8, 1>::try_from(tree_values)
            .expect_err(constants_str::VALUE_9FCB248E),
        2usize,
        constants_usize::ONE,
    );
}

#[test]
fn serde_rejects_string_and_vec_values_outside_bounds() {
    let vec_result = <super::vector::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [1u8, 2u8].into_iter(),
        ),
    );
    assert!(matches!(vec_result, Err(serde::de::value::Error { .. })));
    let string_result =
        <super::bounded_string::BoundedString<2, 3> as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
        );
    assert!(matches!(string_result, Err(serde::de::value::Error { .. })));
}

#[test]
fn vec_deserialization_reports_lower_and_invalid_bounds() {
    let below_min = <super::vector::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            std::iter::empty::<u8>(),
        ),
    )
    .expect_err(constants_str::VALUE_DA49EE30);
    assert!(below_min.to_string().contains("below minimum 1"));

    let invalid = <super::vector::BoundedVec<u8, 2, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            std::iter::empty::<u8>(),
        ),
    )
    .expect_err(constants_str::VALUE_D93AD2D2);
    assert!(invalid.to_string().contains("minimum 2 exceeds maximum 1"));
}

#[test]
fn zero_capacity_vec_rejects_without_deserializing_item_type() {
    let error = <super::vector::BoundedVec<u8, 0, 0> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [TestDeserializerValue::Text(constants_str::UNKNOWN)].into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_30A3CA27);
    assert!(error.to_string().contains("exceeds maximum 0"));
}

#[test]
fn vec_deserialization_stops_after_first_excess_item() {
    let consumed = std::cell::Cell::new(constants_usize::ZERO);
    let values = [1u8, 2u8, 3u8].into_iter().inspect(|_value| {
        consumed.set(consumed.get().saturating_add(constants_usize::ONE));
    });
    let result = <super::vector::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(values),
    );
    let _error = result.expect_err(constants_str::VALUE_1FA2F1E3);
    assert_eq!(consumed.get(), 2usize);
}

#[test]
fn vec_deserialization_ignores_excess_item_type() {
    let error = <super::vector::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [
                TestDeserializerValue::Number(1u8),
                TestDeserializerValue::Text(constants_str::UNKNOWN),
            ]
            .into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_563E607E);
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
    let values =
        <super::vector::BoundedVec<u8, 0, { usize::MAX }> as serde::Deserialize>::deserialize(
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
    let values = <super::btree::BoundedBTreeMap<u8, u8, 2> as serde::Deserialize>::deserialize(
        duplicate_map,
    )
    .expect("22d831a5 map_deserialization_enforces_capacity_and_allows_duplicate_replacement invariant must hold");
    assert_eq!(values.get(&1u8), Some(&3u8));

    let hash_duplicate_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
        [(1u8, 2u8), (1u8, 3u8)].into_iter(),
    );
    let hash_values = <super::hash::BoundedHashMap<u8, u8, 2> as serde::Deserialize>::deserialize(
        hash_duplicate_map,
    )
    .expect("75beb0a8 map_deserialization_enforces_capacity_and_allows_duplicate_replacement invariant must hold");
    assert_eq!(hash_values.get(&1u8), Some(&3u8));

    let duplicate_above_wire_limit =
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(1u8, 2u8), (1u8, 3u8)].into_iter(),
        );
    let duplicate_result =
        <super::btree::BoundedBTreeMap<u8, u8, 1> as serde::Deserialize>::deserialize(
            duplicate_above_wire_limit,
        );
    let _error = duplicate_result.expect_err(constants_str::VALUE_97CBBD88);

    let distinct_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
        [(1u8, 2u8), (2u8, 3u8)].into_iter(),
    );
    let result =
        <super::hash::BoundedHashMap<u8, u8, 1> as serde::Deserialize>::deserialize(distinct_map);
    assert!(matches!(result, Err(serde::de::value::Error { .. })));
}

#[test]
fn map_deserialization_bounds_wire_entries_before_excess_value() {
    let tree_entries = [
        (
            TestDeserializerValue::Text(constants_str::A),
            TestDeserializerValue::Number(1u8),
        ),
        (
            TestDeserializerValue::Number(2u8),
            TestDeserializerValue::Text(constants_str::UNKNOWN),
        ),
    ];
    let tree_error =
        <super::btree::BoundedBTreeMap<String, u8, 1> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                tree_entries.into_iter(),
            ),
        )
        .expect_err(constants_str::VALUE_575CFAD6);
    assert!(tree_error.to_string().contains("exceeds maximum 1"));

    let hash_entries = [
        (
            TestDeserializerValue::Text(constants_str::A),
            TestDeserializerValue::Number(1u8),
        ),
        (
            TestDeserializerValue::Number(2u8),
            TestDeserializerValue::Text(constants_str::UNKNOWN),
        ),
    ];
    let hash_error =
        <super::hash::BoundedHashMap<String, u8, 1> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                hash_entries.into_iter(),
            ),
        )
        .expect_err(constants_str::VALUE_1DD35A8D);
    assert!(hash_error.to_string().contains("exceeds maximum 1"));
}

#[test]
fn zero_capacity_maps_reject_without_deserializing_key_or_value_types() {
    let tree_entries = [(
        TestDeserializerValue::Number(1u8),
        TestDeserializerValue::Text(constants_str::UNKNOWN),
    )];
    let tree_error =
        <super::btree::BoundedBTreeMap<String, u8, 0> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                tree_entries.into_iter(),
            ),
        )
        .expect_err(constants_str::VALUE_4B9C9667);
    assert!(tree_error.to_string().contains("exceeds maximum 0"));

    let hash_entries = [(
        TestDeserializerValue::Number(1u8),
        TestDeserializerValue::Text(constants_str::UNKNOWN),
    )];
    let hash_error =
        <super::hash::BoundedHashMap<String, u8, 0> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                hash_entries.into_iter(),
            ),
        )
        .expect_err(constants_str::VALUE_C189B6DC);
    assert!(hash_error.to_string().contains("exceeds maximum 0"));
}

#[test]
fn hash_map_deserialization_caps_untrusted_size_hint() {
    let entries = MisleadingSizeHintIter {
        values: vec![(1u8, 2u8)].into_iter(),
    };
    let values =
        <super::hash::BoundedHashMap<u8, u8, { usize::MAX }> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(entries),
        )
        .expect("b3cda4f2 hash_map_deserialization_caps_untrusted_size_hint invariant must hold");
    assert_eq!(values.get(&1u8), Some(&2u8));
    let capped_capacity =
        std::collections::HashMap::<u8, u8>::with_capacity(super::SERDE_PREALLOC_MAX_ITEMS)
            .capacity();
    assert!(values.allocation_capacity() <= capped_capacity);
}

#[test]
fn unbounded_vector_schema_omits_max_items() {
    let schema =
        <super::vector::BoundedVec<u8, 0, { usize::MAX }> as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        panic!("5fb9ee86");
    };
    assert_eq!(array.min_items, Some(constants_usize::ZERO));
    assert_eq!(array.max_items, None);
}

#[test]
fn vector_schema_names_include_item_type_and_bounds() {
    let first = <super::vector::BoundedVec<u8, 0, 1> as utoipa::ToSchema>::name();
    let second = <super::vector::BoundedVec<u16, 1, 2> as utoipa::ToSchema>::name();
    assert_ne!(first, second);
    assert!(first.contains(constants_str::BOUNDEDVEC));
    assert!(second.contains(constants_str::BOUNDEDVEC));
}
