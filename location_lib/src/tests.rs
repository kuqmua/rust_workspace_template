#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct DatetimeFmt<'location_lt> {
    location: &'location_lt crate::location::Location,
}
impl std::fmt::Display for DatetimeFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location
            .fmt_datetime(crate::formatter_ref_mut::FormatterRefMut::from(f))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct PlaceFmt<'location_lt> {
    location: &'location_lt crate::location::Location,
    src_place_type: config_lib::src_place_type::SrcPlaceType,
}
impl std::fmt::Display for PlaceFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.location.fmt_place(
            self.src_place_type,
            crate::formatter_ref_mut::FormatterRefMut::from(f),
        )
    }
}
fn test_location(
    duration: std::time::Duration,
    occr: Option<crate::occr::Occr>,
) -> crate::location::Location {
    crate::location::Location {
        file: crate::location_file::LocationFile::try_from(String::from(
            constants_str::catalog::SRC_LIB_RS,
        ))
        .unwrap_or_else(crate::location_file::LocationFile::from),
        commit: crate::location_commit::LocationCommit::try_from(String::from(
            constants_str::catalog::TEST_VALUES_COMMIT,
        ))
        .unwrap_or_else(crate::location_commit::LocationCommit::from),
        duration: crate::location_duration::LocationDuration::from(duration),
        occr,
        line: crate::location_line::LocationLine::try_from(10)
            .expect("fc5a52e8 test_location invariant must hold"),
        column: crate::location_column::LocationColumn::try_from(20)
            .expect("8a180198 test_location invariant must hold"),
    }
}
fn test_occr() -> crate::occr::Occr {
    crate::occr::Occr {
        file: crate::location_file::LocationFile::try_from(String::from(
            constants_str::catalog::SRC_ERROR_RS,
        ))
        .unwrap_or_else(crate::location_file::LocationFile::from),
        line: crate::location_line::LocationLine::try_from(30)
            .expect("1fbd3424 test_occr invariant must hold"),
        column: crate::location_column::LocationColumn::try_from(40)
            .expect("44a1f8ca test_occr invariant must hold"),
    }
}
fn fmt_place(
    location: &crate::location::Location,
    src_place_type: config_lib::src_place_type::SrcPlaceType,
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
        fmt_place(&location, config_lib::src_place_type::SrcPlaceType::Src),
        "src/lib.rs:10:20"
    );
}
#[test]
fn fmt_place_src_with_occr() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
    assert_eq!(
        fmt_place(&location, config_lib::src_place_type::SrcPlaceType::Src),
        "src/lib.rs:10:20 (src/error.rs:30:40)"
    );
}
#[test]
fn fmt_place_github_without_occr() {
    let location = test_location(std::time::Duration::from_secs(0), None);
    assert_eq!(
        fmt_place(&location, config_lib::src_place_type::SrcPlaceType::Github),
        format!(
            "{}/blob/abc123/src/lib.rs#L10",
            constants_str::catalog::NAMING_GITHUB_URL
        )
    );
}
#[test]
fn fmt_place_github_with_occr() {
    let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
    assert_eq!(
        fmt_place(&location, config_lib::src_place_type::SrcPlaceType::Github),
        format!(
            "{}/blob/abc123/src/lib.rs#L10 ({}/blob/abc123/src/error.rs#L30)",
            constants_str::catalog::NAMING_GITHUB_URL,
            constants_str::catalog::NAMING_GITHUB_URL
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
        constants_str::catalog::LOCATION_INCORRECT_DATETIME_MSG
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
fn location_text_deserialization_uses_bounded_try_from() {
    let oversized = constants_str::catalog::X
        .repeat(crate::domain_types::LOC_FILE_MAX_LEN + constants_usize::ONE);
    let _file_error = <crate::location_file::LocationFile as serde::Deserialize>::deserialize(
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized.clone()),
    )
    .expect_err(constants_str::test_fixtures::VALUE_AC9468A7);
    let _commit_error =
        <crate::location_commit::LocationCommit as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized),
        )
        .expect_err(constants_str::test_fixtures::VALUE_1E61B1AF);
}
#[test]
fn coordinates_and_nanoseconds_reject_zero_based_or_overflowing_values() {
    let _line_error = crate::location_line::LocationLine::try_from(constants_u32::ZERO)
        .expect_err(constants_str::test_fixtures::VALUE_3AF5C47B);
    let _column_error = crate::location_column::LocationColumn::try_from(constants_u32::ZERO)
        .expect_err(constants_str::test_fixtures::VALUE_B0E3542F);
    let _nanos_error =
        crate::std_time_duration_nanos::StdTimeDurationNanos::try_from(1_000_000_000u32)
            .expect_err(constants_str::test_fixtures::VALUE_EB22AFCB);
}
