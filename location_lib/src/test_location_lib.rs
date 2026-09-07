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
fn test_coordinates_and_nanoseconds_reject_zero_based_or_overflowing_values() {
    let _line_error = crate::location_line::LocationLine::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_3AF5C47B);
    let _column_error = crate::location_column::LocationColumn::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_B0E3542F);
    let _nanos_error =
        crate::std_time_duration_nanos::StdTimeDurationNanos::try_from(1_000_000_000u32)
            .expect_err(constants_str::VALUE_EB22AFCB);
}
