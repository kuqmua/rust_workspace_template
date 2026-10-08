#[test]
fn test_between_string_maps_preserve_bounds_order_and_unknown_fields() {
    assert!(
        [(i32::MIN, i32::MAX), (0i32, 0i32), (1i32, 2i32)]
            .into_iter()
            .all(|(start, end)| {
                crate::between::Between::try_new(start, end).is_ok_and(|expected| {
                    [
                        vec![
                            (constants_str::PG_CRUD_START_FIELD, start),
                            (constants_str::PG_CRUD_END_FIELD, end),
                        ],
                        vec![
                            (constants_str::PG_CRUD_END_FIELD, end),
                            (constants_str::PG_CRUD_START_FIELD, start),
                        ],
                        vec![
                            (constants_str::X, 99i32),
                            (constants_str::PG_CRUD_START_FIELD, start),
                            (constants_str::PG_CRUD_END_FIELD, end),
                        ],
                        vec![
                            (constants_str::PG_CRUD_START_FIELD, start),
                            (constants_str::X, 99i32),
                            (constants_str::PG_CRUD_END_FIELD, end),
                        ],
                        vec![
                            (constants_str::PG_CRUD_START_FIELD, start),
                            (constants_str::PG_CRUD_END_FIELD, end),
                            (constants_str::X, 99i32),
                        ],
                    ]
                    .into_iter()
                    .all(|entries| {
                        <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                                entries.into_iter(),
                            ),
                        )
                        .is_ok_and(|actual| actual == expected)
                    })
                })
            })
    );
}

#[test]
fn test_between_numeric_and_byte_field_identifiers_preserve_existing_mapping() {
    assert!(
        crate::between::Between::try_new(1i32, 2i32).is_ok_and(|expected| {
            let numeric = <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    [(0u64, 99i32), (2u64, 2i32), (1u64, 1i32), (u64::MAX, 99i32)].into_iter(),
                ),
            );
            let bytes = <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    [
                        (constants_str::X, 99i32),
                        (constants_str::PG_CRUD_END_FIELD, 2i32),
                        (constants_str::PG_CRUD_START_FIELD, 1i32),
                    ]
                    .into_iter()
                    .map(|(name, value)| {
                        (
                                serde::de::value::BorrowedBytesDeserializer::<
                                    serde::de::value::Error,
                                >::new(name.as_bytes()),
                                value,
                            )
                    }),
                ),
            );
            numeric.is_ok_and(|actual| actual == expected)
                && bytes.is_ok_and(|actual| actual == expected)
        })
    );
}

#[test]
fn test_between_map_missing_and_duplicate_fields_preserve_exact_errors() {
    [
        (
            Vec::new(),
            <serde::de::value::Error as serde::de::Error>::missing_field(
                constants_str::PG_CRUD_START_FIELD,
            ),
        ),
        (
            vec![(constants_str::PG_CRUD_START_FIELD, 1i32)],
            <serde::de::value::Error as serde::de::Error>::missing_field(
                constants_str::PG_CRUD_END_FIELD,
            ),
        ),
        (
            vec![(constants_str::PG_CRUD_END_FIELD, 2i32)],
            <serde::de::value::Error as serde::de::Error>::missing_field(
                constants_str::PG_CRUD_START_FIELD,
            ),
        ),
        (
            vec![
                (constants_str::PG_CRUD_START_FIELD, 1i32),
                (constants_str::PG_CRUD_START_FIELD, 2i32),
            ],
            <serde::de::value::Error as serde::de::Error>::duplicate_field(
                constants_str::PG_CRUD_START_FIELD,
            ),
        ),
        (
            vec![
                (constants_str::PG_CRUD_END_FIELD, 2i32),
                (constants_str::PG_CRUD_END_FIELD, 3i32),
            ],
            <serde::de::value::Error as serde::de::Error>::duplicate_field(
                constants_str::PG_CRUD_END_FIELD,
            ),
        ),
    ]
    .into_iter()
    .fold((), |(), (entries, expected)| {
        assert!(
            <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    entries.into_iter()
                ),
            )
            .is_err_and(|error| error.to_string() == expected.to_string())
        );
    });
}

#[test]
fn test_between_sequences_preserve_values_and_exact_missing_element_diagnostics() {
    assert!(
        [(i32::MIN, i32::MAX), (0i32, 0i32), (1i32, 2i32)]
            .into_iter()
            .all(|(start, end)| {
                crate::between::Between::try_new(start, end).is_ok_and(|expected| {
                    <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                            [start, end].into_iter(),
                        ),
                    )
                    .is_ok_and(|actual| actual == expected)
                })
            })
    );
    [(Vec::new(), 1usize), (vec![1i32], 2usize)]
        .into_iter()
        .fold((), |(), (values, length)| {
            let expected = <serde::de::value::Error as serde::de::Error>::invalid_length(
                length,
                &constants_str::PG_CRUD_BETWEEN_EXPECTING,
            );
            assert!(
                <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                    serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                        values.into_iter()
                    ),
                )
                .is_err_and(|error| error.to_string() == expected.to_string())
            );
        });
}

#[test]
fn test_between_leaf_decode_failures_preserve_sources_at_each_bound() {
    let expected = <i32 as serde::Deserialize>::deserialize(serde::de::value::U64Deserializer::<
        serde::de::value::Error,
    >::new(u64::MAX))
    .err();
    assert!(expected.is_some());
    if let Some(source) = expected {
        assert!(
            [(u64::MAX, 2u64), (1u64, u64::MAX)]
                .into_iter()
                .all(|(start, end)| {
                    let map = <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                            [
                                (constants_str::PG_CRUD_START_FIELD, start),
                                (constants_str::PG_CRUD_END_FIELD, end),
                            ]
                            .into_iter(),
                        ),
                    );
                    let sequence =
                        <crate::between::Between<i32> as serde::Deserialize>::deserialize(
                            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                                [start, end].into_iter(),
                            ),
                        );
                    map.is_err_and(|error| error.to_string() == source.to_string())
                        && sequence.is_err_and(|error| error.to_string() == source.to_string())
                })
        );
    }
}

#[test]
fn test_between_constructor_rejections_retain_exact_bound_values() {
    assert!(
        [(2i32, 1i32), (i32::MAX, i32::MIN)]
            .into_iter()
            .all(|(start, end)| {
                matches!(crate::between::Between::try_new(start, end),
            Err(crate::between_try_new_error::BetweenTryNewError::StartNotLessThanOrEqualToEnd {
                start: rejected_start, end: rejected_end, ..
            }) if rejected_start == start && rejected_end == end)
            })
    );
    assert!([(f32::NAN, 1.0f32), (1.0f32, f32::NAN), (f32::NAN, f32::NAN)]
        .into_iter().all(|(start, end)| {
            matches!(crate::between::Between::try_new(start, end),
                Err(crate::between_try_new_error::BetweenTryNewError::StartNotLessThanOrEqualToEnd {
                    start: rejected_start, end: rejected_end, ..
                }) if rejected_start.to_bits() == start.to_bits() && rejected_end.to_bits() == end.to_bits())
        }));
}
