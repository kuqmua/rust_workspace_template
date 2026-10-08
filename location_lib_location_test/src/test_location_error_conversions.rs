#[test]
fn test_location_error_conversions_preserve_all_field_adapters_and_empty_collections() {
    let location_fixture = || {
        <location_lib::location::Location as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                [
                    (
                        stringify!(file),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Text(
                            crate::location_test_text::LocationTestText::from(
                                constants_str::SRC_LIB_RS,
                            ),
                        ),
                    ),
                    (
                        stringify!(commit),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Text(
                            crate::location_test_text::LocationTestText::from(
                                constants_str::TEST_VALUES_COMMIT,
                            ),
                        ),
                    ),
                    (
                        stringify!(duration),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Duration,
                    ),
                    (
                        stringify!(occurrence),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Absent,
                    ),
                    (
                        stringify!(line),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Coordinate,
                    ),
                    (
                        stringify!(column),
                        crate::test_location_fixture_value::TestLocationFixtureValue::Coordinate,
                    ),
                ]
                .into_iter(),
            ),
        )
        .unwrap_or_else(|error| std::panic::panic_any(error))
    };
    [constants_str::EMPTY, constants_str::X, constants_str::U_1F496]
        .into_iter()
        .fold((), |(), value| {
            let expected_location = location_fixture();
            let expected_text = crate::location_test_text::LocationTestText::from(value);
            let original_another = crate::error_two::ErrorTwo::Another {
                sdasdasd: crate::location_test_text::LocationTestText::from(value),
                location: location_fixture(),
            };
            let expected_another_display = format!("{}: {value}\n{expected_location}", stringify!(sdasdasd));
            assert_eq!(original_another.to_string(), expected_another_display);
            let another = original_another.into_serde_version();
            assert_eq!(another.to_string(), expected_another_display);
            assert!(matches!(another,
                crate::error_two::ErrorTwoWithSerde::Another { sdasdasd, location }
                    if sdasdasd == expected_text && location == expected_location
            ));
            let original_nested = crate::error_unnamed_one::ErrorUnnamedOne::Something(
                crate::error_two::ErrorTwo::Variant {
                    error_field_display_with_serde_field: crate::location_test_text::LocationTestText::from(value),
                    location: location_fixture(),
                },
            );
            let expected_nested_display = format!("{}: {value}\n{expected_location}", stringify!(error_field_display_with_serde_field));
            assert_eq!(original_nested.to_string(), expected_nested_display);
            let nested = original_nested.into_serde_version();
            assert_eq!(nested.to_string(), expected_nested_display);
            assert!(matches!(nested,
                crate::error_unnamed_one::ErrorUnnamedOneWithSerde::Something(
                    crate::error_two::ErrorTwoWithSerde::Variant { error_field_display_with_serde_field, location }
                ) if error_field_display_with_serde_field == expected_text && location == expected_location
            ));
        });
    let display_fixture = |location_test_text| {
        crate::display_struct::DisplayStruct::new(
            location_test_text,
            crate::location_test_flag::LocationTestFlag::from(true),
        )
    };
    let serde_fixture = |location_test_text| {
        crate::serde_struct::SerdeStruct::new(
            location_test_text,
            crate::location_test_count::LocationTestCount::from(u32::MAX),
            crate::location_test_flag::LocationTestFlag::from(false),
        )
    };
    let nested_fixture = |location_test_text| {
        crate::error_unnamed_one::ErrorUnnamedOne::Something(crate::error_two::ErrorTwo::Variant {
            error_field_display_with_serde_field: location_test_text,
            location: location_fixture(),
        })
    };
    [false, true].into_iter().fold((), |(), populated| {
        let collection_length = usize::from(populated).saturating_mul(2usize);
        let values = [constants_str::X, constants_str::ROOT];
        let display_values = values.into_iter().take(collection_length).map(|value| {
            display_fixture(crate::location_test_text::LocationTestText::from(value))
        }).collect::<Vec<_>>();
        let expected_display_values = display_values.iter().map(|display_struct| {
            to_err_string::to_err_string::ToErrString::to_err_string(display_struct).into_inner()
        }).collect::<Vec<_>>();
        let serde_values = values.into_iter().take(collection_length).map(|value| {
            serde_fixture(crate::location_test_text::LocationTestText::from(value))
        }).collect::<Vec<_>>();
        let expected_serde_values = serde_values.iter().map(|serde_struct| {
            to_err_string::to_err_string::ToErrString::to_err_string(serde_struct)
        }).collect::<Vec<_>>();
        let scalar_display = display_fixture(crate::location_test_text::LocationTestText::from(constants_str::X));
        let expected_scalar_display = to_err_string::to_err_string::ToErrString::to_err_string(&scalar_display).into_inner();
        let scalar_serde = serde_fixture(crate::location_test_text::LocationTestText::from(constants_str::ROOT));
        let expected_scalar_serde = to_err_string::to_err_string::ToErrString::to_err_string(&scalar_serde);
        let converted = crate::error_one::ErrorOne::Variant {
            error_field_display_field: scalar_display,
            error_field_serde: scalar_serde,
            error_field_location_field: crate::error_two::ErrorTwo::Another {
                sdasdasd: crate::location_test_text::LocationTestText::from(constants_str::U_1F496),
                location: location_fixture(),
            },
            error_field_vec_display_field: display_values,
            error_field_vec_serde: serde_values,
            error_field_vec_location_field: values.into_iter().take(collection_length).map(|value| {
                nested_fixture(crate::location_test_text::LocationTestText::from(value))
            }).collect(),
            hashmap_string_string: values.into_iter().take(collection_length).map(|value| (
                crate::location_test_text::LocationTestText::from(value),
                display_fixture(crate::location_test_text::LocationTestText::from(value)),
            )).collect(),
            hashmap_string_serde: values.into_iter().take(collection_length).map(|value| (
                crate::location_test_text::LocationTestText::from(value),
                serde_fixture(crate::location_test_text::LocationTestText::from(value)),
            )).collect(),
            hashmap_string_location: values.into_iter().take(collection_length).map(|value| (
                crate::location_test_text::LocationTestText::from(value),
                nested_fixture(crate::location_test_text::LocationTestText::from(value)),
            )).collect(),
            location: location_fixture(),
        }.into_serde_version();
        let crate::error_one::ErrorOneWithSerde::Variant {
            error_field_display_field,
            error_field_serde,
            error_field_location_field,
            error_field_vec_display_field,
            error_field_vec_serde,
            error_field_vec_location_field,
            hashmap_string_string,
            hashmap_string_serde,
            hashmap_string_location,
            location: converted_location,
        } = converted;
        assert_eq!(converted_location, location_fixture());
        assert_eq!(error_field_display_field, expected_scalar_display);
        assert_eq!(to_err_string::to_err_string::ToErrString::to_err_string(&error_field_serde), expected_scalar_serde);
        assert!(matches!(error_field_location_field,
            crate::error_two::ErrorTwoWithSerde::Another { sdasdasd, location }
                if sdasdasd == crate::location_test_text::LocationTestText::from(constants_str::U_1F496)
                    && location == location_fixture()
        ));
        assert_eq!(error_field_vec_display_field, expected_display_values);
        assert_eq!(error_field_vec_serde.iter().map(|serde_struct| {
            to_err_string::to_err_string::ToErrString::to_err_string(serde_struct)
        }).collect::<Vec<_>>(), expected_serde_values);
        assert_eq!(error_field_vec_location_field.len(), collection_length);
        assert_eq!(hashmap_string_string.len(), collection_length);
        assert_eq!(hashmap_string_serde.len(), collection_length);
        assert_eq!(hashmap_string_location.len(), collection_length);
        let nested_matches = |error_unnamed_one_with_serde: &crate::error_unnamed_one::ErrorUnnamedOneWithSerde, location_test_text: &crate::location_test_text::LocationTestText| {
            matches!(error_unnamed_one_with_serde,
                crate::error_unnamed_one::ErrorUnnamedOneWithSerde::Something(
                    crate::error_two::ErrorTwoWithSerde::Variant { error_field_display_with_serde_field, location }
                ) if error_field_display_with_serde_field == location_test_text && *location == location_fixture()
            )
        };
        values.into_iter().take(collection_length).enumerate().fold((), |(), (index, value)| {
            let text = crate::location_test_text::LocationTestText::from(value);
            let key = to_err_string::to_err_string::ToErrString::to_err_string(&text).into_inner();
            assert!(error_field_vec_location_field.get(index).is_some_and(|nested_error| nested_matches(nested_error, &text)));
            assert_eq!(hashmap_string_string.get(&key), Some(&to_err_string::to_err_string::ToErrString::to_err_string(&display_fixture(crate::location_test_text::LocationTestText::from(value))).into_inner()));
            assert!(hashmap_string_serde.get(&key).is_some_and(|serde_struct| {
                to_err_string::to_err_string::ToErrString::to_err_string(serde_struct)
                    == to_err_string::to_err_string::ToErrString::to_err_string(&serde_fixture(crate::location_test_text::LocationTestText::from(value)))
            }));
            assert!(hashmap_string_location.get(&key).is_some_and(|nested_error| nested_matches(nested_error, &text)));
        });
    });
}
