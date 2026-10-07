#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DatetimeFmt<'location_lt> {
    location: &'location_lt crate::location::Location,
}
impl std::fmt::Display for DatetimeFmt<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location
            .fmt_datetime(crate::formatter_ref_mut::FormatterRefMut::from(formatter))
    }
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct PlaceFmt<'location_lt> {
    location: &'location_lt crate::location::Location,
    source_place_type: config_lib::source_place_type::SourcePlaceType,
}
impl std::fmt::Display for PlaceFmt<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location.fmt_place(
            self.source_place_type,
            crate::formatter_ref_mut::FormatterRefMut::from(formatter),
        )
    }
}
fn test_location(
    duration: std::time::Duration,
    option: Option<crate::occurrence::Occurrence>,
) -> crate::location::Location {
    crate::location::Location::from((
        crate::location_file::LocationFile::try_from(String::from(constants_str::SRC_LIB_RS))
            .unwrap_or_else(crate::location_file::LocationFile::from),
        crate::location_commit::LocationCommit::try_from(String::from(
            constants_str::TEST_VALUES_COMMIT,
        ))
        .unwrap_or_else(crate::location_commit::LocationCommit::from),
        crate::location_duration::LocationDuration::from(duration),
        option,
        crate::location_line::LocationLine::try_from(10).expect(constants_str::DIAGNOSTIC_FC5A52E8),
        crate::location_column::LocationColumn::try_from(20)
            .expect(constants_str::DIAGNOSTIC_8A180198),
    ))
}
fn test_occurrence() -> crate::occurrence::Occurrence {
    crate::occurrence::Occurrence::new(
        crate::location_file::LocationFile::try_from(String::from(constants_str::SRC_ERROR_RS))
            .unwrap_or_else(crate::location_file::LocationFile::from),
        crate::location_line::LocationLine::try_from(30).expect(constants_str::DIAGNOSTIC_1FBD3424),
        crate::location_column::LocationColumn::try_from(40)
            .expect(constants_str::DIAGNOSTIC_44A1F8CA),
    )
}
fn fmt_place(
    location: &crate::location::Location,
    source_place_type: config_lib::source_place_type::SourcePlaceType,
) -> String {
    format!(
        "{:}",
        PlaceFmt {
            location,
            source_place_type
        }
    )
}
#[test]
fn test_fmt_place_src_without_occurrence() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        fmt_place(
            &location,
            config_lib::source_place_type::SourcePlaceType::Src
        ),
        constants_str::VALUE_2FF162D0
    );
}
#[test]
fn test_fmt_place_src_with_occurrence() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occurrence()));
    assert_eq!(
        fmt_place(
            &location,
            config_lib::source_place_type::SourcePlaceType::Src
        ),
        constants_str::VALUE_C5939F43
    );
}
#[test]
fn test_fmt_place_github_without_occurrence() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        fmt_place(
            &location,
            config_lib::source_place_type::SourcePlaceType::Github
        ),
        format!(
            "{}/blob/abc123/src/lib.rs#L10",
            constants_str::NAMING_GITHUB_URL
        )
    );
}
#[test]
fn test_fmt_place_github_with_occurrence() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occurrence()));
    assert_eq!(
        fmt_place(
            &location,
            config_lib::source_place_type::SourcePlaceType::Github
        ),
        format!(
            "{}/blob/abc123/src/lib.rs#L10 ({}/blob/abc123/src/error.rs#L30)",
            constants_str::NAMING_GITHUB_URL,
            constants_str::NAMING_GITHUB_URL
        )
    );
}
#[test]
fn test_fmt_datetime_returns_fallback_for_overflowed_duration() {
    let location = test_location(std::time::Duration::MAX, None);
    assert_eq!(
        format!(
            "{}",
            DatetimeFmt {
                location: &location
            }
        ),
        constants_str::LOCATION_INCORRECT_DATETIME_MSG
    );
}
#[test]
fn test_datetime_with_tz_returns_expected_epoch_time_for_zero_duration() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        format!(
            "{}",
            DatetimeFmt {
                location: &location
            }
        ),
        constants_str::VALUE_BA5B49F1
    );
    let date_time = location
        .datetime_with_tz()
        .expect(constants_str::DIAGNOSTIC_F5C41DD8);
    assert_eq!(
        chrono::DateTime::<chrono::FixedOffset>::from(date_time)
            .format(constants_str::VALUE_34A18516)
            .to_string(),
        constants_str::VALUE_BA5B49F1
    );
}
#[test]
fn test_location_text_deserialization_uses_bounded_try_from() {
    let oversized =
        constants_str::X.repeat(crate::domain_types::LOC_FILE_MAX_LEN + constants_usize::ONE);
    let _file_error = <crate::location_file::LocationFile as serde::Deserialize>::deserialize(
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized.clone()),
    )
    .expect_err(constants_str::VALUE_AC9468A7);
    let _commit_error =
        <crate::location_commit::LocationCommit as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized),
        )
        .expect_err(constants_str::VALUE_1E61B1AF);
}
#[test]
fn test_overlong_location_file_keeps_original_path_prefix() {
    let raw = format!(
        "{}{}",
        constants_str::SRC_LIB_RS,
        constants_str::X.repeat(crate::domain_types::LOC_FILE_MAX_LEN)
    );
    let location_file = crate::location_file::LocationFile::from(
        crate::location_file_ref::LocationFileRef::from(raw.as_str()),
    );
    assert_eq!(
        location_file.as_ref().len(),
        crate::domain_types::LOC_FILE_MAX_LEN
    );
    assert!(
        location_file
            .as_ref()
            .starts_with(constants_str::SRC_LIB_RS)
    );
}
#[test]
fn test_coordinates_and_nanoseconds_reject_zero_based_or_overflowing_values() {
    let _line_error = crate::location_line::LocationLine::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_3AF5C47B);
    let _column_error = crate::location_column::LocationColumn::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_B0E3542F);
    assert!([0u32, 1u32, 999_999_999u32].into_iter().all(|value| {
        crate::std_time_duration_nanos::StdTimeDurationNanos::try_from(value)
            .is_ok_and(|std_time_duration_nanos| *std_time_duration_nanos == value)
    }));
    assert!([1_000_000_000u32, u32::MAX].into_iter().all(|value| matches!(
        crate::std_time_duration_nanos::StdTimeDurationNanos::try_from(value),
        Err(crate::std_time_duration_nanos_try_from_u32_error::StdTimeDurationNanosTryFromU32Error::OutOfRange)
    )));
}

#[test]
fn test_location_coordinates_validate_deserialization_and_preserve_extreme_values() {
    assert_eq!(
        crate::location_line::LocationLine::first().to_string(),
        1u32.to_string()
    );
    assert_eq!(
        crate::location_column::LocationColumn::first().to_string(),
        1u32.to_string()
    );
    assert!([0u32, 1u32, u32::MAX].into_iter().all(|value| {
        let deserializer = || serde::de::value::U32Deserializer::<serde::de::value::Error>::new(value);
        [
            crate::location_line::LocationLine::try_from(value).map(|location_line| location_line.to_string()).map_err(|error| error.to_string()),
            crate::location_column::LocationColumn::try_from(value).map(|location_column| location_column.to_string()).map_err(|error| error.to_string()),
            <crate::location_line::LocationLine as serde::Deserialize>::deserialize(deserializer()).map(|location_line| location_line.to_string()).map_err(|error| error.to_string()),
            <crate::location_column::LocationColumn as serde::Deserialize>::deserialize(deserializer()).map(|location_column| location_column.to_string()).map_err(|error| error.to_string()),
        ].into_iter().all(|result| {
            if value == 0u32 {
                result.is_err_and(|error| error == crate::location_coordinate_try_from_u32_error::LocationCoordinateTryFromU32Error::OutOfRange.to_string())
            } else {
                result.is_ok_and(|text| text == value.to_string())
            }
        })
    }));
}

#[test]
fn test_location_duration_deserialization_preserves_normalization_and_rejects_overflow() {
    fn duration_wire_error_matches_native<Factory, Deserializer>(factory: Factory)
    where
        Factory: Fn() -> Deserializer,
        Deserializer: serde::Deserializer<'static>,
    {
        let native_result = <std::time::Duration as serde::Deserialize>::deserialize(factory())
            .map_err(|error| error.to_string());
        let wrapper_result =
            <crate::location_duration::LocationDuration as serde::Deserialize>::deserialize(
                factory(),
            )
            .map(std::time::Duration::from)
            .map_err(|error| error.to_string());
        assert!(native_result.as_ref().is_err_and(|error| !error.is_empty()));
        assert_eq!(wrapper_result, native_result);
    }

    [
        (0u64, 0u64, std::time::Duration::ZERO),
        (
            0u64,
            999_999_999u64,
            std::time::Duration::new(0u64, 999_999_999u32),
        ),
        (1u64, 1_000_000_000u64, std::time::Duration::from_secs(2u64)),
        (
            0u64,
            u64::from(u32::MAX),
            std::time::Duration::new(4u64, 294_967_295u32),
        ),
        (u64::MAX, 999_999_999u64, std::time::Duration::MAX),
    ]
    .into_iter()
    .fold((), |(), (seconds, nanoseconds, expected)| {
        let actual =
            <crate::location_duration::LocationDuration as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    [
                        (constants_str::SECS, seconds),
                        (constants_str::NANOS, nanoseconds),
                    ]
                    .into_iter(),
                ),
            )
            .unwrap_or_else(|error| std::panic::panic_any(error));
        assert_eq!(std::time::Duration::from(actual), expected);
        assert_eq!(
            crate::location_duration::LocationDuration::from(expected),
            actual
        );
    });
    [
        (u64::MAX, 1_000_000_000u64),
        (0u64, u64::from(u32::MAX).saturating_add(1u64)),
        (0u64, u64::MAX),
    ]
    .into_iter()
    .fold((), |(), (seconds, nanoseconds)| {
        duration_wire_error_matches_native(|| {
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                [
                    (constants_str::SECS, seconds),
                    (constants_str::NANOS, nanoseconds),
                ]
                .into_iter(),
            )
        });
    });
    [constants_str::SECS, constants_str::NANOS]
        .into_iter()
        .fold((), |(), field| {
            duration_wire_error_matches_native(|| {
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    [(field, 0u64)].into_iter(),
                )
            });
            duration_wire_error_matches_native(|| {
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                    [
                        (constants_str::SECS, 0u64),
                        (constants_str::NANOS, 0u64),
                        (field, 0u64),
                    ]
                    .into_iter(),
                )
            });
        });
    duration_wire_error_matches_native(|| {
        serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
            [(constants_str::SECS, -1i64), (constants_str::NANOS, 0i64)].into_iter(),
        )
    });
    duration_wire_error_matches_native(|| {
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(constants_str::X)
    });
}

#[test]
fn test_location_text_wire_boundaries_preserve_unicode_byte_limits() {
    fn location_text_wire_boundaries<Text, const MAXIMUM: usize>()
    where
        Text: serde::de::DeserializeOwned + TryFrom<String> + AsRef<str>,
        <Text as TryFrom<String>>::Error: Send + 'static,
    {
        [
            constants_str::EMPTY.to_owned(),
            constants_str::X.to_owned(),
            constants_str::X.repeat(MAXIMUM),
            [
                constants_str::X
                    .repeat(MAXIMUM.saturating_sub(constants_str::U_1F496.len()))
                    .as_str(),
                constants_str::U_1F496,
            ]
            .concat(),
        ]
        .into_iter()
        .fold((), |(), raw_value| {
            let parsed = Text::deserialize(serde::de::value::StrDeserializer::<
                serde::de::value::Error,
            >::new(&raw_value))
            .unwrap_or_else(|error| std::panic::panic_any(error));
            assert_eq!(parsed.as_ref(), raw_value.as_str());
            let validated =
                Text::try_from(raw_value).unwrap_or_else(|error| std::panic::panic_any(error));
            assert_eq!(validated.as_ref(), parsed.as_ref());
        });
        [
            constants_str::X.repeat(MAXIMUM.saturating_add(1usize)),
            [
                constants_str::X
                    .repeat(
                        MAXIMUM
                            .saturating_sub(constants_str::U_1F496.len())
                            .saturating_add(1usize),
                    )
                    .as_str(),
                constants_str::U_1F496,
            ]
            .concat(),
        ]
        .into_iter()
        .fold((), |(), raw_value| {
            assert!(
                Text::deserialize(
                    serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&raw_value)
                )
                .is_err()
            );
            assert!(Text::try_from(raw_value).is_err());
        });
        assert!(
            Text::deserialize(
                serde::de::value::BoolDeserializer::<serde::de::value::Error>::new(false)
            )
            .is_err()
        );
        assert!(
            Text::deserialize(
                serde::de::value::U64Deserializer::<serde::de::value::Error>::new(0u64)
            )
            .is_err()
        );
        assert!(
            Text::deserialize(serde::de::value::SeqDeserializer::<
                _,
                serde::de::value::Error,
            >::new(std::iter::empty::<u8>()))
            .is_err()
        );
    }
    location_text_wire_boundaries::<
        crate::location_file::LocationFile,
        { crate::domain_types::LOC_FILE_MAX_LEN },
    >();
    location_text_wire_boundaries::<
        crate::location_commit::LocationCommit,
        { crate::domain_types::LOC_COMMIT_MAX_LEN },
    >();
}

#[test]
fn test_location_file_truncation_preserves_complete_unicode_characters() {
    let maximum = crate::domain_types::LOC_FILE_MAX_LEN;
    let split_character_prefix = constants_str::X.repeat(maximum.saturating_sub(1usize));
    let split_character_input = [split_character_prefix.as_str(), constants_str::U_1F496].concat();
    let split_character_file = crate::location_file::LocationFile::from(
        crate::location_file_ref::LocationFileRef::from(split_character_input.as_str()),
    );
    assert_eq!(
        split_character_file.as_ref(),
        split_character_prefix.as_str()
    );
    let complete_character_prefix = [
        constants_str::X
            .repeat(maximum.saturating_sub(constants_str::U_1F496.len()))
            .as_str(),
        constants_str::U_1F496,
    ]
    .concat();
    let complete_character_input = [complete_character_prefix.as_str(), constants_str::X].concat();
    let complete_character_file = crate::location_file::LocationFile::from(
        crate::location_file_ref::LocationFileRef::from(complete_character_input.as_str()),
    );
    assert_eq!(
        complete_character_file.as_ref(),
        complete_character_prefix.as_str()
    );
    assert_eq!(complete_character_file.as_ref().len(), maximum);
}

#[cfg(target_os = "linux")]
#[test]
fn test_location_datetime_outside_chrono_range_preserves_native_failure_behavior() {
    let duration = std::time::Duration::from_secs(10_000_000_000_000u64);
    let epoch = std::time::UNIX_EPOCH.checked_add(duration);
    assert!(epoch.is_some());
    let location = test_location(duration, None);
    assert!(
        [
            std::panic::catch_unwind(|| epoch.map(chrono::DateTime::<chrono::Utc>::from).is_some()),
            std::panic::catch_unwind(|| location.datetime_with_tz().is_some()),
            std::panic::catch_unwind(|| !format!(
                "{}",
                DatetimeFmt {
                    location: &location
                }
            )
            .is_empty()),
        ]
        .into_iter()
        .all(|result| result.is_err_and(|panic_payload| {
            panic_payload
                .downcast_ref::<&str>()
                .is_some_and(|text| !text.is_empty())
                || panic_payload
                    .downcast_ref::<String>()
                    .is_some_and(|text| !text.is_empty())
        }))
    );
}

#[cfg(target_os = "linux")]
#[test]
fn test_location_datetime_maximum_utc_preserves_timestamp_and_offset_formatting() {
    let maximum_utc = chrono::DateTime::<chrono::Utc>::MAX_UTC;
    let seconds =
        u64::try_from(maximum_utc.timestamp()).unwrap_or_else(|error| std::panic::panic_any(error));
    let location = test_location(
        std::time::Duration::new(seconds, maximum_utc.timestamp_subsec_nanos()),
        None,
    );
    let converted = location.datetime_with_tz();
    assert!(converted.is_some());
    let converted_datetime = converted
        .unwrap_or_else(|| std::panic::panic_any(constants_str::LOCATION_INCORRECT_DATETIME_MSG));
    let native_datetime = chrono::DateTime::<chrono::FixedOffset>::from(converted_datetime);
    assert_eq!(native_datetime.timestamp(), maximum_utc.timestamp());
    assert_eq!(
        native_datetime.timestamp_subsec_nanos(),
        maximum_utc.timestamp_subsec_nanos()
    );
    let expected_offset =
        chrono::FixedOffset::east_opt(crate::domain_types::LOC_DISPLAY_UTC_OFFSET_SECS)
            .unwrap_or_else(|| {
                std::panic::panic_any(constants_str::LOCATION_INCORRECT_DATETIME_MSG)
            });
    assert_eq!(
        format!(
            "{}",
            DatetimeFmt {
                location: &location
            }
        ),
        maximum_utc
            .with_timezone(&expected_offset)
            .format(constants_str::VALUE_34A18516)
            .to_string()
    );
}
