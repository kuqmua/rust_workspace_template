#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct DatetimeFmt<'location_lt> {
    location: &'location_lt super::Location,
}
impl std::fmt::Display for DatetimeFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location.fmt_datetime(super::FormatterRefMut::from(f))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct PlaceFmt<'location_lt> {
    location: &'location_lt super::Location,
    src_place_type: config_lib::domain_types::types::SrcPlaceType,
}
impl std::fmt::Display for PlaceFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location
            .fmt_place(self.src_place_type, super::FormatterRefMut::from(f))
    }
}
fn test_location(duration: std::time::Duration, occr: Option<super::Occr>) -> super::Location {
    super::Location {
        file: super::LocationFile::try_from(String::from(constants_str::SRC_LIB_RS))
            .unwrap_or_else(super::LocationFile::from),
        commit: super::LocationCommit::try_from(String::from(constants_str::TEST_VALUES_COMMIT))
            .unwrap_or_else(super::LocationCommit::from),
        duration: super::LocationDuration::from(duration),
        occr,
        line: super::LocationLine::try_from(10)
            .expect("fc5a52e8 test_location invariant must hold"),
        column: super::LocationColumn::try_from(20)
            .expect("8a180198 test_location invariant must hold"),
    }
}
fn test_occr() -> super::Occr {
    super::Occr {
        file: super::LocationFile::try_from(String::from(constants_str::SRC_ERROR_RS))
            .unwrap_or_else(super::LocationFile::from),
        line: super::LocationLine::try_from(30).expect("1fbd3424 test_occr invariant must hold"),
        column: super::LocationColumn::try_from(40)
            .expect("44a1f8ca test_occr invariant must hold"),
    }
}
fn fmt_place(
    location: &super::Location,
    src_place_type: config_lib::domain_types::types::SrcPlaceType,
) -> String {
    format!(
        "{:}",
        PlaceFmt {
            location,
            src_place_type
        }
    )
}
#[test]
fn fmt_place_src_without_occr() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        fmt_place(
            &location,
            config_lib::domain_types::types::SrcPlaceType::Src
        ),
        "src/lib.rs:10:20"
    );
}
#[test]
fn fmt_place_src_with_occr() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
    assert_eq!(
        fmt_place(
            &location,
            config_lib::domain_types::types::SrcPlaceType::Src
        ),
        "src/lib.rs:10:20 (src/error.rs:30:40)"
    );
}
#[test]
fn fmt_place_github_without_occr() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        fmt_place(
            &location,
            config_lib::domain_types::types::SrcPlaceType::Github
        ),
        format!(
            "{}/blob/abc123/src/lib.rs#L10",
            constants_str::NAMING_GITHUB_URL
        )
    );
}
#[test]
fn fmt_place_github_with_occr() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
    assert_eq!(
        fmt_place(
            &location,
            config_lib::domain_types::types::SrcPlaceType::Github
        ),
        format!(
            "{}/blob/abc123/src/lib.rs#L10 ({}/blob/abc123/src/error.rs#L30)",
            constants_str::NAMING_GITHUB_URL,
            constants_str::NAMING_GITHUB_URL
        )
    );
}
#[test]
fn fmt_datetime_returns_fallback_for_overflowed_duration() {
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
fn datetime_with_tz_returns_expected_epoch_time_for_zero_duration() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    let date_time = location.datetime_with_tz().expect("f5c41dd8 datetime_with_tz_returns_expected_epoch_time_for_zero_duration invariant must hold");
    assert_eq!(
        date_time.0.format("%Y-%m-%d %H:%M:%S").to_string(),
        "1970-01-01 03:00:00"
    );
}
#[test]
fn location_display_timezone_uses_expected_offset() {
    let offset = super::Location::location_display_timezone()
        .expect("5c53d969 location_display_timezone_uses_expected_offset invariant must hold");
    assert_eq!(
        offset.0.local_minus_utc(),
        super::LOC_DISPLAY_UTC_OFFSET_SECS
    );
}
#[test]
fn location_text_deserialization_uses_bounded_try_from() {
    let oversized = constants_str::X.repeat(super::LOC_FILE_MAX_LEN + constants_usize::ONE);
    let _file_error = <super::LocationFile as serde::Deserialize>::deserialize(
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized.clone()),
    )
    .expect_err(constants_str::VALUE_AC9468A7);
    let _commit_error = <super::LocationCommit as serde::Deserialize>::deserialize(
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized),
    )
    .expect_err(constants_str::VALUE_1E61B1AF);
}
#[test]
fn coordinates_and_nanoseconds_reject_zero_based_or_overflowing_values() {
    let _line_error = super::LocationLine::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_3AF5C47B);
    let _column_error = super::LocationColumn::try_from(constants_u32::ZERO)
        .expect_err(constants_str::VALUE_B0E3542F);
    let _nanos_error = super::StdTimeDurationNanos::try_from(1_000_000_000u32)
        .expect_err(constants_str::VALUE_EB22AFCB);
}
