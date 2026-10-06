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
