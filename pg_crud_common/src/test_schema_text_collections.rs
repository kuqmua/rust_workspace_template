#[test]
fn test_schema_text_collections_preserve_empty_inputs_order_and_duplicates() {
    assert!(crate::schema_texts::schema_texts(Vec::new()).is_ok_and(|texts| texts.is_empty()));
    assert!(
        crate::static_schema_texts::static_schema_texts(Vec::new().into())
            .is_ok_and(|texts| texts.is_empty())
    );
    let values = [
        constants_str::TEST_DB_COLUMN_ID,
        constants_str::TEST_DB_DATA_TYPE_UUID,
        constants_str::TEST_DB_COLUMN_ID,
    ];
    let dynamic_values = values.into_iter().map(str::to_owned).collect();
    assert!(
        crate::schema_texts::schema_texts(dynamic_values).is_ok_and(|texts| {
            texts.len() == values.len()
                && texts
                    .iter()
                    .zip(values)
                    .all(|(text, expected)| text.as_ref() == expected)
        })
    );
    let static_values = values
        .into_iter()
        .map(crate::db_static_schema_text::DbStaticSchemaText::from)
        .collect::<Vec<_>>()
        .into();
    assert!(
        crate::static_schema_texts::static_schema_texts(static_values).is_ok_and(|texts| {
            texts.len() == values.len()
                && texts
                    .iter()
                    .zip(values)
                    .all(|(text, expected)| text.as_ref() == expected)
        })
    );
}

#[test]
fn test_schema_text_collection_reports_oversized_entry_at_every_position() {
    assert!([0usize, 1usize, 2usize].into_iter().all(|oversized_position| {
        let oversized = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
        let expected = crate::db_schema_text::DbSchemaText::try_from(oversized.clone());
        let values = (0usize..3usize).map(|position| {
            if position == oversized_position { oversized.clone() } else { constants_str::TEST_DB_COLUMN_ID.to_owned() }
        }).collect();
        let observed = crate::schema_texts::schema_texts(values);
        matches!((expected, observed), (Err(expected_error), Err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong(observed_error))) if expected_error == observed_error)
    }));
}

#[test]
fn test_schema_text_collection_preserves_empty_and_maximum_length_entries() {
    let values = vec![
        String::new(),
        constants_str::X.repeat(constants_usize::VALUE_1_048_576),
        String::new(),
        '\u{00e9}'.to_string().repeat(524_288usize),
    ];
    assert!(
        crate::schema_texts::schema_texts(values.clone()).is_ok_and(|texts| {
            texts.len() == values.len()
                && texts
                    .iter()
                    .zip(&values)
                    .all(|(text, expected)| text.as_ref() == expected)
        })
    );
}

#[test]
fn test_schema_text_collection_preserves_first_oversized_error() {
    let first = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::ONE);
    let second = constants_str::X.repeat(constants_usize::VALUE_1_048_576 + constants_usize::TWO);
    let expected = crate::db_schema_text::DbSchemaText::try_from(first.clone());
    let observed = crate::schema_texts::schema_texts(vec![
        constants_str::TEST_DB_COLUMN_ID.to_owned(),
        first,
        second,
    ]);
    assert!(
        matches!((expected, observed), (Err(expected_error), Err(crate::db_schema_conformance_error::DbSchemaConformanceError::SchemaTextTooLong(observed_error))) if expected_error == observed_error)
    );
}
