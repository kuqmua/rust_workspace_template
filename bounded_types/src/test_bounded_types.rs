#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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

fn assert_above_max(
    bounded_value_error: crate::bounded_value_error::BoundedValueError,
    actual: usize,
    max: usize,
) {
    assert_eq!(
        bounded_value_error,
        crate::bounded_value_error::BoundedValueError::AboveMax {
            actual: crate::bounded_len::BoundedLen::from(actual),
            max: crate::bounded_len::BoundedLen::from(max),
        }
    );
}

#[test]
fn test_string_bounds_are_inclusive() {
    let value =
        crate::bounded_string::BoundedString::<1, 3>::try_from(constants_str::ABC_ALT_3.to_owned())
            .expect(constants_str::DIAGNOSTIC_6F09AD52);
    assert_eq!(value.as_ref(), constants_str::ABC_ALT_3);
    assert_eq!(value.len().get(), 3usize);
    assert_eq!(
        crate::bounded_string::BoundedString::<1, 2>::try_from(constants_str::ABC_ALT_3.to_owned())
            .expect_err(constants_str::VALUE_E4A5AF09),
        crate::bounded_string_error::BoundedStringError::AboveMaximum {
            actual_length: crate::bounded_len::BoundedLen::from(3usize),
            maximum_length: crate::bounded_len::BoundedLen::from(2usize),
        }
    );
}

#[test]
fn test_string_rejects_below_minimum_and_invalid_bounds() {
    assert_eq!(
        crate::bounded_string::BoundedString::<1, 3>::try_from(String::new())
            .expect_err(constants_str::VALUE_0EF05B85),
        crate::bounded_string_error::BoundedStringError::BelowMinimum {
            actual_length: crate::bounded_len::BoundedLen::from(constants_usize::ZERO),
            minimum_length: crate::bounded_len::BoundedLen::from(constants_usize::ONE),
        }
    );
    assert_eq!(
        crate::bounded_string::BoundedString::<2, 1>::try_from(constants_str::A_ALT.to_owned())
            .expect_err(constants_str::VALUE_2DE961C6),
        crate::bounded_string_error::BoundedStringError::BelowMinimum {
            actual_length: crate::bounded_len::BoundedLen::from(constants_usize::ONE),
            minimum_length: crate::bounded_len::BoundedLen::from(2usize),
        }
    );
}

#[test]
fn test_byte_string_bounds_count_utf8_bytes() {
    let unicode = String::from_utf8(vec![0xc3u8, 0xa9u8, 0xc3u8, 0xa9u8])
        .expect(constants_str::DIAGNOSTIC_9167AED1);
    assert_eq!(
        crate::bounded_string::BoundedString::<0, 2>::try_from(unicode)
            .expect_err(constants_str::VALUE_311B8C86),
        crate::bounded_string_error::BoundedStringError::AboveMaximum {
            actual_length: crate::bounded_len::BoundedLen::from(4usize),
            maximum_length: crate::bounded_len::BoundedLen::from(2usize),
        }
    );
}

#[test]
fn test_bounded_string_supports_default_unbounded_mutation_and_conversion() {
    let mut value =
        crate::bounded_string::BoundedString::from_unbounded(String::from(constants_str::A_ALT));
    value.as_mut_string().push_str(constants_str::B);
    assert_eq!(value, constants_str::AB);
    assert_eq!(value, String::from(constants_str::AB));
    let borrowed: &str = std::borrow::Borrow::borrow(&value);
    assert_eq!(borrowed, constants_str::AB);
    let string: String = value.into();
    assert_eq!(string, constants_str::AB);
}

#[test]
fn test_bounded_string_truncates_at_utf8_boundary() {
    let unicode = String::from_utf8(vec![0xc3u8, 0xa9u8, 0xc3u8, 0xa9u8])
        .expect(constants_str::DIAGNOSTIC_43AF9B78);
    let value = crate::bounded_string::BoundedString::<0, 3>::from_truncated(unicode);
    assert_eq!(value.as_str().len(), 2usize);
    assert!(value.as_str().is_char_boundary(value.as_str().len()));
}

#[test]
fn test_bounded_string_default_and_prevalidated_construction_preserve_values() {
    let empty = crate::bounded_string::BoundedString::<0, 4>::default();
    assert_eq!(empty.as_str(), constants_str::EMPTY);
    let value = crate::bounded_string::BoundedString::<1, 1>::from_prevalidated(String::from(
        constants_str::A,
    ));
    assert_eq!(value.as_str(), constants_str::A);
}

#[test]
fn test_byte_string_schema_publishes_byte_extensions() {
    let schema = <crate::bounded_string::BoundedString<1, 4> as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema else {
        std::panic::panic_any(constants_str::PANIC_43EA6E9B);
    };
    let extensions = object.extensions.expect(constants_str::DIAGNOSTIC_177A114D);
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
fn test_unbounded_byte_string_schema_omits_max_bytes_extension() {
    let schema =
        <crate::bounded_string::BoundedString<1, { usize::MAX }> as utoipa::PartialSchema>::schema(
        );
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) = schema else {
        std::panic::panic_any(constants_str::PANIC_43FBEA64);
    };
    let extensions = object.extensions.expect(constants_str::DIAGNOSTIC_803CFA80);
    assert!(extensions.contains_key(constants_str::OPENAPI_MIN_BYTES_EXTENSION));
    assert!(!extensions.contains_key(constants_str::OPENAPI_MAX_BYTES_EXTENSION));
}

#[test]
fn test_vec_bounds_and_growth_are_enforced() {
    let mut values = crate::bounded_vec::BoundedVec::<u8, 0, 1>::try_from(Vec::new())
        .expect(constants_str::DIAGNOSTIC_CB18BC21);
    values
        .try_push(1u8)
        .expect(constants_str::DIAGNOSTIC_28F49231);
    assert_eq!(values.as_slice(), &[1u8]);
    assert_above_max(
        values
            .try_push(2u8)
            .expect_err(constants_str::VALUE_F2921AC3),
        2usize,
        constants_usize::ONE,
    );
    assert_eq!(values.into_inner(), [1u8]);
}

#[test]
fn test_vec_rejects_below_minimum_and_invalid_bounds() {
    assert_eq!(
        crate::bounded_vec::BoundedVec::<u8, 1, 2>::try_from(Vec::new())
            .expect_err(constants_str::VALUE_8BF60687),
        crate::bounded_value_error::BoundedValueError::BelowMin {
            actual: crate::bounded_len::BoundedLen::from(constants_usize::ZERO),
            min: crate::bounded_len::BoundedLen::from(constants_usize::ONE),
        }
    );
    assert_eq!(
        crate::bounded_vec::BoundedVec::<u8, 2, 1>::try_from(vec![1u8])
            .expect_err(constants_str::VALUE_7E536E25),
        crate::bounded_value_error::BoundedValueError::InvalidBounds {
            min: crate::bounded_len::BoundedLen::from(2usize),
            max: crate::bounded_len::BoundedLen::from(constants_usize::ONE),
        }
    );
}

#[test]
fn test_max_vec_construction_preserves_order_and_supports_consuming_iteration() {
    let values =
        crate::bounded_vec::BoundedVec::<u8, 0, { usize::MAX }>::from_max_iter([3u8, 1u8, 2u8]);
    assert_eq!(values.len().get(), 3usize);
    assert_eq!(values.into_iter().collect::<Vec<u8>>(), [3u8, 1u8, 2u8]);
}

#[test]
fn test_btree_map_replacement_is_allowed_at_capacity() {
    let mut values = crate::bounded_b_tree_map::BoundedBTreeMap::<u8, u8, 1>::try_from(
        std::collections::BTreeMap::new(),
    )
    .expect(constants_str::DIAGNOSTIC_EA1FDC07);
    let _previous = values
        .try_insert(1u8, 2u8)
        .expect(constants_str::DIAGNOSTIC_285278FE);
    assert_eq!(
        values
            .try_insert(1u8, 3u8)
            .expect(constants_str::DIAGNOSTIC_946EB9A8),
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
fn test_hash_map_capacity_mutation_and_removal_are_enforced() {
    let mut values = crate::bounded_hash_map::BoundedHashMap::<u8, u8, 1>::default();
    assert_eq!(
        values
            .try_insert(1u8, 2u8)
            .expect(constants_str::DIAGNOSTIC_C1B15EE9),
        None
    );
    assert_eq!(
        values
            .try_insert(1u8, 3u8)
            .expect(constants_str::DIAGNOSTIC_B4E85208),
        Some(2u8)
    );
    values
        .get_mut(&1u8)
        .map(|value| *value = 4u8)
        .expect(constants_str::DIAGNOSTIC_32578CEC);
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
        values
            .try_insert(2u8, 5u8)
            .expect(constants_str::DIAGNOSTIC_98C16CA4),
        None
    );
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "repository policy forbids for loops"
)]
fn test_btree_map_iteration_and_pop_preserve_key_order() {
    let mut values = crate::bounded_b_tree_map::BoundedBTreeMap::<u8, u8, 3>::default();
    [3u8, 1u8, 2u8].into_iter().for_each(|key| {
        let _previous = values
            .try_insert(key, key)
            .expect(constants_str::DIAGNOSTIC_02EFAC64);
    });
    values.iter_mut().for_each(|(_key, value)| {
        *value = value.saturating_add(10u8);
    });
    assert_eq!(
        values
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>(),
        [(1u8, 11u8), (2u8, 12u8), (3u8, 13u8)]
    );
    assert_eq!(values.pop_first(), Some((1u8, 11u8)));
    assert_eq!(values.into_values().collect::<Vec<u8>>(), [12u8, 13u8]);
}

#[test]
fn test_raw_map_conversions_reject_values_above_capacity() {
    let hash_values = [(1u8, 1u8), (2u8, 2u8)]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_above_max(
        crate::bounded_hash_map::BoundedHashMap::<u8, u8, 1>::try_from(hash_values)
            .expect_err(constants_str::VALUE_C531636A),
        2usize,
        constants_usize::ONE,
    );
    let tree_values = [(1u8, 1u8), (2u8, 2u8)]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_above_max(
        crate::bounded_b_tree_map::BoundedBTreeMap::<u8, u8, 1>::try_from(tree_values)
            .expect_err(constants_str::VALUE_9FCB248E),
        2usize,
        constants_usize::ONE,
    );
}

#[test]
fn test_serde_rejects_string_and_vec_values_outside_bounds() {
    let vec_result = <crate::bounded_vec::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [1u8, 2u8].into_iter(),
        ),
    );
    assert!(matches!(vec_result, Err(serde::de::value::Error { .. })));
    let string_result =
        <crate::bounded_string::BoundedString<2, 3> as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
        );
    assert!(matches!(string_result, Err(serde::de::value::Error { .. })));
}

#[test]
fn test_vec_deserialization_reports_lower_and_invalid_bounds() {
    let below_min = <crate::bounded_vec::BoundedVec<u8, 1, 2> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            std::iter::empty::<u8>(),
        ),
    )
    .expect_err(constants_str::VALUE_DA49EE30);
    assert!(
        below_min
            .to_string()
            .contains(constants_str::VALUE_227386E2)
    );

    let invalid = <crate::bounded_vec::BoundedVec<u8, 2, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            std::iter::empty::<u8>(),
        ),
    )
    .expect_err(constants_str::VALUE_D93AD2D2);
    assert!(invalid.to_string().contains(constants_str::VALUE_DF55C59B));
}

#[test]
fn test_zero_capacity_vec_rejects_without_deserializing_item_type() {
    let error = <crate::bounded_vec::BoundedVec<u8, 0, 0> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [TestDeserializerValue::Text(constants_str::UNKNOWN)].into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_30A3CA27);
    assert!(error.to_string().contains(constants_str::VALUE_9016A762));
}

#[test]
fn test_vec_deserialization_stops_after_first_excess_item() {
    let consumed = std::cell::Cell::new(constants_usize::ZERO);
    let values = [1u8, 2u8, 3u8].into_iter().inspect(|_value| {
        consumed.set(consumed.get().saturating_add(constants_usize::ONE));
    });
    let result = <crate::bounded_vec::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(values),
    );
    let _error = result.expect_err(constants_str::VALUE_1FA2F1E3);
    assert_eq!(consumed.get(), 2usize);
}

#[test]
fn test_vec_deserialization_ignores_excess_item_type() {
    let error = <crate::bounded_vec::BoundedVec<u8, 0, 1> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            [
                TestDeserializerValue::Number(1u8),
                TestDeserializerValue::Text(constants_str::UNKNOWN),
            ]
            .into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_563E607E);
    assert!(error.to_string().contains(constants_str::VALUE_476C9E40));
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct MisleadingSizeHintIter<Values> {
    values: Values,
}
impl<Values> Iterator for MisleadingSizeHintIter<Values>
where
    Values: Iterator,
{
    type Item = Values::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, Some(usize::MAX))
    }
}

#[test]
fn test_vec_deserialization_caps_untrusted_size_hint() {
    let values =
        <crate::bounded_vec::BoundedVec<u8, 0, { usize::MAX }> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                MisleadingSizeHintIter {
                    values: [1u8].into_iter(),
                },
            ),
        )
        .expect(constants_str::DIAGNOSTIC_D1CE80F4);
    assert_eq!(values.as_slice(), &[1u8]);
    assert!(
        values.allocation_capacity() <= crate::serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS
    );
}

#[test]
fn test_map_deserialization_enforces_capacity_and_allows_duplicate_replacement() {
    let duplicate_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
        [(1u8, 2u8), (1u8, 3u8)].into_iter(),
    );
    let values =
        <crate::bounded_b_tree_map::BoundedBTreeMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            duplicate_map,
        )
        .expect(constants_str::DIAGNOSTIC_22D831A5);
    assert_eq!(values.get(&1u8), Some(&3u8));

    let hash_duplicate_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
        [(1u8, 2u8), (1u8, 3u8)].into_iter(),
    );
    let hash_values =
        <crate::bounded_hash_map::BoundedHashMap<u8, u8, 2> as serde::Deserialize>::deserialize(
            hash_duplicate_map,
        )
        .expect(constants_str::DIAGNOSTIC_75BEB0A8);
    assert_eq!(hash_values.get(&1u8), Some(&3u8));

    let duplicate_above_wire_limit =
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(1u8, 2u8), (1u8, 3u8)].into_iter(),
        );
    let duplicate_result =
        <crate::bounded_b_tree_map::BoundedBTreeMap<u8, u8, 1> as serde::Deserialize>::deserialize(
            duplicate_above_wire_limit,
        );
    let _error = duplicate_result.expect_err(constants_str::VALUE_97CBBD88);

    let distinct_map = serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
        [(1u8, 2u8), (2u8, 3u8)].into_iter(),
    );
    let result =
        <crate::bounded_hash_map::BoundedHashMap<u8, u8, 1> as serde::Deserialize>::deserialize(
            distinct_map,
        );
    assert!(matches!(result, Err(serde::de::value::Error { .. })));
}

#[test]
fn test_map_deserialization_bounds_wire_entries_before_excess_value() {
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
    let tree_error = <crate::bounded_b_tree_map::BoundedBTreeMap<String, u8, 1> as serde::Deserialize>::deserialize(
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            tree_entries.into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_575CFAD6);
    assert!(
        tree_error
            .to_string()
            .contains(constants_str::VALUE_476C9E40)
    );

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
    let hash_error = <crate::bounded_hash_map::BoundedHashMap<String, u8, 1> as serde::Deserialize>::deserialize(
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            hash_entries.into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_1DD35A8D);
    assert!(
        hash_error
            .to_string()
            .contains(constants_str::VALUE_476C9E40)
    );
}

#[test]
fn test_zero_capacity_maps_reject_without_deserializing_key_or_value_types() {
    let tree_entries = [(
        TestDeserializerValue::Number(1u8),
        TestDeserializerValue::Text(constants_str::UNKNOWN),
    )];
    let tree_error = <crate::bounded_b_tree_map::BoundedBTreeMap<String, u8, 0> as serde::Deserialize>::deserialize(
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            tree_entries.into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_4B9C9667);
    assert!(
        tree_error
            .to_string()
            .contains(constants_str::VALUE_9016A762)
    );

    let hash_entries = [(
        TestDeserializerValue::Number(1u8),
        TestDeserializerValue::Text(constants_str::UNKNOWN),
    )];
    let hash_error = <crate::bounded_hash_map::BoundedHashMap<String, u8, 0> as serde::Deserialize>::deserialize(
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            hash_entries.into_iter(),
        ),
    )
    .expect_err(constants_str::VALUE_C189B6DC);
    assert!(
        hash_error
            .to_string()
            .contains(constants_str::VALUE_9016A762)
    );
}

#[test]
fn test_hash_map_deserialization_caps_untrusted_size_hint() {
    let entries = MisleadingSizeHintIter {
        values: [(1u8, 2u8)].into_iter(),
    };
    let values =
        <crate::bounded_hash_map::BoundedHashMap<u8, u8, { usize::MAX }> as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(entries),
        )
        .expect(constants_str::DIAGNOSTIC_B3CDA4F2);
    assert_eq!(values.get(&1u8), Some(&2u8));
    let capped_capacity = std::collections::HashMap::<u8, u8>::with_capacity(
        crate::serde_prealloc_max_items::SERDE_PREALLOC_MAX_ITEMS,
    )
    .capacity();
    assert!(values.allocation_capacity() <= capped_capacity);
}

#[test]
fn test_unbounded_vector_schema_omits_max_items() {
    let schema =
        <crate::bounded_vec::BoundedVec<u8, 0, { usize::MAX }> as utoipa::PartialSchema>::schema();
    let utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) = schema else {
        std::panic::panic_any(constants_str::PANIC_5FB9EE86);
    };
    assert_eq!(array.min_items, Some(constants_usize::ZERO));
    assert_eq!(array.max_items, None);
}

#[test]
fn test_vector_schema_names_include_item_type_and_bounds() {
    let first = <crate::bounded_vec::BoundedVec<u8, 0, 1> as utoipa::ToSchema>::name();
    let second = <crate::bounded_vec::BoundedVec<u16, 1, 2> as utoipa::ToSchema>::name();
    assert_ne!(first, second);
    assert!(first.contains(constants_str::BOUNDEDVEC));
    assert!(second.contains(constants_str::BOUNDEDVEC));
}
