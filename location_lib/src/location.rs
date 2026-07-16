#![allow(clippy::module_name_repetitions)]
const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const LOC_FILE_MAX_LEN: usize = 1_048_576;
const LOC_COMMIT_MAX_LEN: usize = 1_048_576;
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = LOC_FILE_MAX_LEN )]
pub struct LocationFile(String);
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
)]
pub struct LocationLine(u32);
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
)]
pub struct LocationColumn(u32);
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = LOC_COMMIT_MAX_LEN )]
pub struct LocationCommit(String);
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::FromInner,
)]
pub struct StdLocationDuration(std::time::Duration);
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for StdLocationDuration {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::STDLOCATIONDURATION,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::SECS,
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int64,
                        ))),
                )
                .property(
                    str_constants::NANOS,
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int32,
                        )))
                        .minimum(Some(0.0))
                        .maximum(Some(999_999_999.0)),
                )
                .required(str_constants::SECS)
                .required(str_constants::NANOS)
                .build()
                .into(),
        )
    }
}
#[derive(Debug, Clone, Copy)]
struct LocationFileRef<'file_lt>(pub &'file_lt str);
struct StdFmtRefMut<'fmt_ref_lt, 'fmt_lt>(pub &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>);
#[derive(Debug, Clone, Copy)]
struct ChronoLocationDisplayTimezone(pub chrono::FixedOffset);
#[derive(Debug, Clone, Copy)]
struct ChronoLocationDateTime(pub chrono::DateTime<chrono::FixedOffset>);
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct Occr {
    pub file: LocationFile,
    pub line: LocationLine,
    pub column: LocationColumn,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct Location {
    #[allow(clippy::arbitrary_source_item_ordering)]
    file: LocationFile,
    commit: LocationCommit,
    duration: StdLocationDuration,
    occr: Option<Occr>,
    line: LocationLine,
    column: LocationColumn,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_pass_by_value)]
impl Location {
    #[allow(clippy::single_call_fn)] // shared offset accessor is reused by formatter and tests
    fn location_display_timezone() -> Option<ChronoLocationDisplayTimezone> {
        chrono::FixedOffset::east_opt(LOC_DISPLAY_UTC_OFFSET_SECS)
            .map(ChronoLocationDisplayTimezone)
    }
    fn fmt_github_location(
        &self,
        f: StdFmtRefMut<'_, '_>,
        file: LocationFileRef<'_>,
        line: LocationLine,
    ) -> std::fmt::Result {
        write!(
            f.0,
            "{}/blob/{}/{}#L{}",
            str_constants::NAMING_GITHUB_URL,
            self.commit.as_ref(),
            file.0,
            line
        )
    }
    fn fmt_src_location(
        f: StdFmtRefMut<'_, '_>,
        file: LocationFileRef<'_>,
        line: LocationLine,
        column: LocationColumn,
    ) -> std::fmt::Result {
        write!(f.0, "{}:{line}:{column}", file.0)
    }
    #[allow(clippy::single_call_fn)] // centralizes datetime + timezone composition so formatting can stay branch-light and tests can target conversion separately
    fn datetime_with_tz(&self) -> Option<ChronoLocationDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration.0)?;
        let offset = Self::location_display_timezone()?;
        Some(ChronoLocationDateTime(
            chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset.0),
        ))
    }
    fn fmt_datetime(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f.0, "{}", v.0.format("%Y-%m-%d %H:%M:%S")),
            None => {
                f.0.write_str(str_constants::LOCATION_INCORRECT_DATETIME_MSG)
            }
        }
    }
    fn fmt_github_place(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        self.fmt_github_location(
            StdFmtRefMut(f.0),
            LocationFileRef(self.file.as_ref()),
            self.line,
        )?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(str_constants::TEXT)?;
            self.fmt_github_location(StdFmtRefMut(f.0), LocationFileRef(v.file.as_ref()), v.line)?;
            f.0.write_str(str_constants::TEXT_ALT_5)
        } else {
            Ok(())
        }
    }
    fn fmt_place(
        &self,
        src_place_type: config_lib::types::SrcPlaceType,
        f: StdFmtRefMut<'_, '_>,
    ) -> std::fmt::Result {
        match src_place_type {
            config_lib::types::SrcPlaceType::Src => self.fmt_src_place(f),
            config_lib::types::SrcPlaceType::Github => self.fmt_github_place(f),
        }
    }
    fn fmt_src_place(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        Self::fmt_src_location(
            StdFmtRefMut(f.0),
            LocationFileRef(self.file.as_ref()),
            self.line,
            self.column,
        )?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(str_constants::TEXT)?;
            Self::fmt_src_location(
                StdFmtRefMut(f.0),
                LocationFileRef(v.file.as_ref()),
                v.line,
                v.column,
            )?;
            f.0.write_str(str_constants::TEXT_ALT_5)
        } else {
            Ok(())
        }
    }
    #[must_use]
    pub fn new<FileTy, LineTy, ColumnTy>(
        file: FileTy,
        line: LineTy,
        column: ColumnTy,
        occr: Option<Occr>,
    ) -> Self
    where
        FileTy: AsRef<str>,
        LineTy: Into<LocationLine>,
        ColumnTy: Into<LocationColumn>,
    {
        Self {
            file: LocationFile::try_from(file.as_ref().to_owned())
                .unwrap_or_else(LocationFile::from),
            line: line.into(),
            column: column.into(),
            commit: LocationCommit::try_from(git_info::PROJECT_GIT_INFO.commit.as_ref().to_owned())
                .unwrap_or_else(LocationCommit::from),
            duration: StdLocationDuration(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default(),
            ),
            occr,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, utoipa::ToSchema, optml::Optml)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    pub secs: StdTimeDurationSecs,
    pub nanos: StdTimeDurationNanos,
}
#[derive(
    Debug, Clone, Copy, utoipa::ToSchema, optml::Optml, newtype::DerefInner, newtype::FromInner,
)]
pub struct StdTimeDurationSecs(u64);
#[derive(
    Debug, Clone, Copy, utoipa::ToSchema, optml::Optml, newtype::DerefInner, newtype::FromInner,
)]
pub struct StdTimeDurationNanos(u32);
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(
            config_lib::types::SrcPlaceType::from_env_or_default(),
            StdFmtRefMut(f),
        )?;
        f.write_str(str_constants::SPACE)?;
        self.fmt_datetime(StdFmtRefMut(f))
    }
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)]
mod tests {
    struct DatetimeFmt<'location_lt> {
        location: &'location_lt super::Location,
    }
    impl std::fmt::Display for DatetimeFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.location.fmt_datetime(super::StdFmtRefMut(f))
        }
    }
    struct PlaceFmt<'location_lt> {
        location: &'location_lt super::Location,
        src_place_type: config_lib::types::SrcPlaceType,
    }
    impl std::fmt::Display for PlaceFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.location
                .fmt_place(self.src_place_type, super::StdFmtRefMut(f))
        }
    }
    fn test_location(duration: std::time::Duration, occr: Option<super::Occr>) -> super::Location {
        super::Location {
            file: super::LocationFile::try_from(String::from(str_constants::SRC_LIB_RS))
                .unwrap_or_else(super::LocationFile::from),
            commit: super::LocationCommit::try_from(String::from(
                str_constants::TEST_VALUES_COMMIT,
            ))
            .unwrap_or_else(super::LocationCommit::from),
            duration: super::StdLocationDuration(duration),
            occr,
            line: super::LocationLine(10),
            column: super::LocationColumn(20),
        }
    }
    fn test_occr() -> super::Occr {
        super::Occr {
            file: super::LocationFile::try_from(String::from(str_constants::SRC_ERROR_RS))
                .unwrap_or_else(super::LocationFile::from),
            line: super::LocationLine(30),
            column: super::LocationColumn(40),
        }
    }
    fn fmt_place(
        location: &super::Location,
        src_place_type: config_lib::types::SrcPlaceType,
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
            fmt_place(&location, config_lib::types::SrcPlaceType::Src),
            "src/lib.rs:10:20"
        );
    }
    #[test]
    fn fmt_place_src_with_occr() {
        let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&location, config_lib::types::SrcPlaceType::Src),
            "src/lib.rs:10:20 (src/error.rs:30:40)"
        );
    }
    #[test]
    fn fmt_place_github_without_occr() {
        let location = test_location(std::time::Duration::from_secs(0), None);
        assert_eq!(
            fmt_place(&location, config_lib::types::SrcPlaceType::Github),
            format!(
                "{}/blob/abc123/src/lib.rs#L10",
                str_constants::NAMING_GITHUB_URL
            )
        );
    }
    #[test]
    fn fmt_place_github_with_occr() {
        let location = test_location(std::time::Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&location, config_lib::types::SrcPlaceType::Github),
            format!(
                "{}/blob/abc123/src/lib.rs#L10 ({}/blob/abc123/src/error.rs#L30)",
                str_constants::NAMING_GITHUB_URL,
                str_constants::NAMING_GITHUB_URL
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
            str_constants::LOCATION_INCORRECT_DATETIME_MSG
        );
    }
    #[test]
    fn datetime_with_tz_returns_expected_epoch_time_for_zero_duration() {
        let location = test_location(std::time::Duration::from_secs(0), None);
        let date_time = location.datetime_with_tz().expect("f5c41dd8");
        assert_eq!(
            date_time.0.format("%Y-%m-%d %H:%M:%S").to_string(),
            "1970-01-01 03:00:00"
        );
    }
    #[test]
    fn location_display_timezone_uses_expected_offset() {
        let offset = super::Location::location_display_timezone().expect("5c53d969");
        assert_eq!(
            offset.0.local_minus_utc(),
            super::LOC_DISPLAY_UTC_OFFSET_SECS
        );
    }
}
