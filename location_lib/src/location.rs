#![allow(clippy::module_name_repetitions)]
const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const LOC_FILE_MAX_LEN: usize = 1_048_576;
const LOC_COMMIT_MAX_LEN: usize = 1_048_576;
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = LOC_FILE_MAX_LEN )]
#[serde(try_from = "String")]
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
    newtype::TryFrom,
)]
#[serde(try_from = "u32")]
#[try_from(
    error = LocationCoordinateTryFromU32Error,
    validator = LocationLine::validate
)]
pub struct LocationLine(u32);
impl From<std::num::NonZeroU32> for LocationLine {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(value.get())
    }
}
impl LocationLine {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), LocationCoordinateTryFromU32Error> {
        if *value == 0u32 {
            Err(LocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
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
    newtype::TryFrom,
)]
#[serde(try_from = "u32")]
#[try_from(
    error = LocationCoordinateTryFromU32Error,
    validator = LocationColumn::validate
)]
pub struct LocationColumn(u32);
impl From<std::num::NonZeroU32> for LocationColumn {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(value.get())
    }
}
impl LocationColumn {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), LocationCoordinateTryFromU32Error> {
        if *value == 0u32 {
            Err(LocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct LocationCoordinateTryFromU32Error;
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = LOC_COMMIT_MAX_LEN )]
#[serde(try_from = "String")]
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
#[serde(from = "std::time::Duration")]
pub struct StdLocationDuration(std::time::Duration);
impl utoipa::PartialSchema for StdLocationDuration {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                str_constants::SECS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int64,
                    ))),
            )
            .property(
                str_constants::NANOS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    )))
                    .minimum(Some(0.0))
                    .maximum(Some(999_999_999.0)),
            )
            .required(str_constants::SECS)
            .required(str_constants::NANOS)
            .build()
            .into()
    }
}
impl utoipa::ToSchema for StdLocationDuration {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(str_constants::STDLOCATIONDURATION)
    }
}
#[derive(optml::Optml, Debug, Clone, Copy, newtype::FromInner)]
struct LocationFileRef<'file_lt>(&'file_lt str);
#[derive(optml::Optml, newtype::FromInner)]
struct StdFmtRefMut<'fmt_ref_lt, 'fmt_lt>(&'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>);
#[derive(optml::Optml, Debug, Clone, Copy, newtype::FromInner)]
struct ChronoLocationDisplayTimezone(chrono::FixedOffset);
#[derive(optml::Optml, Debug, Clone, Copy, newtype::FromInner)]
struct ChronoLocationDateTime(chrono::DateTime<chrono::FixedOffset>);
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
    fn datetime_with_tz(&self) -> Option<ChronoLocationDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration.0)?;
        let offset = Self::location_display_timezone()?;
        Some(ChronoLocationDateTime::from(
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
            StdFmtRefMut::from(&mut *f.0),
            LocationFileRef::from(self.file.as_ref()),
            self.line,
        )?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(str_constants::TEXT)?;
            self.fmt_github_location(
                StdFmtRefMut::from(&mut *f.0),
                LocationFileRef::from(v.file.as_ref()),
                v.line,
            )?;
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
            StdFmtRefMut::from(&mut *f.0),
            LocationFileRef::from(self.file.as_ref()),
            self.line,
            self.column,
        )?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(str_constants::TEXT)?;
            Self::fmt_src_location(
                StdFmtRefMut::from(&mut *f.0),
                LocationFileRef::from(v.file.as_ref()),
                v.line,
                v.column,
            )?;
            f.0.write_str(str_constants::TEXT_ALT_5)
        } else {
            Ok(())
        }
    }
    #[must_use]
    pub fn new<FileTy>(
        file: FileTy,
        line: LocationLine,
        column: LocationColumn,
        occr: Option<Occr>,
    ) -> Self
    where
        FileTy: AsRef<str>,
    {
        Self {
            file: LocationFile::try_from(file.as_ref().to_owned())
                .unwrap_or_else(LocationFile::from),
            line,
            column,
            commit: LocationCommit::try_from(
                git_info::project_git_info().commit().as_ref().to_owned(),
            )
            .unwrap_or_else(LocationCommit::from),
            duration: StdLocationDuration::from(
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
    Debug, Clone, Copy, utoipa::ToSchema, optml::Optml, newtype::DerefInner, newtype::TryFrom,
)]
#[try_from(
    error = StdTimeDurationNanosTryFromU32Error,
    validator = StdTimeDurationNanos::validate
)]
pub struct StdTimeDurationNanos(u32);
impl StdTimeDurationNanos {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), StdTimeDurationNanosTryFromU32Error> {
        if *value < 1_000_000_000u32 {
            Ok(())
        } else {
            Err(StdTimeDurationNanosTryFromU32Error)
        }
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct StdTimeDurationNanosTryFromU32Error;
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(
            config_lib::types::SrcPlaceType::from_env_or_default(),
            StdFmtRefMut::from(&mut *f),
        )?;
        f.write_str(str_constants::SPACE)?;
        self.fmt_datetime(StdFmtRefMut::from(&mut *f))
    }
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)]
mod tests {
    #[derive(optml::Optml)]
    struct DatetimeFmt<'location_lt> {
        location: &'location_lt super::Location,
    }
    impl std::fmt::Display for DatetimeFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.location.fmt_datetime(super::StdFmtRefMut::from(f))
        }
    }
    #[derive(optml::Optml)]
    struct PlaceFmt<'location_lt> {
        location: &'location_lt super::Location,
        src_place_type: config_lib::types::SrcPlaceType,
    }
    impl std::fmt::Display for PlaceFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.location
                .fmt_place(self.src_place_type, super::StdFmtRefMut::from(f))
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
            duration: super::StdLocationDuration::from(duration),
            occr,
            line: super::LocationLine::try_from(10)
                .expect("fc5a52e8 test_location invariant must hold"),
            column: super::LocationColumn::try_from(20)
                .expect("8a180198 test_location invariant must hold"),
        }
    }
    fn test_occr() -> super::Occr {
        super::Occr {
            file: super::LocationFile::try_from(String::from(str_constants::SRC_ERROR_RS))
                .unwrap_or_else(super::LocationFile::from),
            line: super::LocationLine::try_from(30)
                .expect("1fbd3424 test_occr invariant must hold"),
            column: super::LocationColumn::try_from(40)
                .expect("44a1f8ca test_occr invariant must hold"),
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
        let oversized = "x".repeat(super::LOC_FILE_MAX_LEN + 1usize);
        let _file_error = <super::LocationFile as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized.clone()),
        )
        .expect_err("845c5b02");
        let _commit_error = <super::LocationCommit as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(oversized),
        )
        .expect_err("7e50ddbb");
    }
    #[test]
    fn coordinates_and_nanoseconds_reject_zero_based_or_overflowing_values() {
        let _line_error = super::LocationLine::try_from(0u32).expect_err("f4dfc0b1");
        let _column_error = super::LocationColumn::try_from(0u32).expect_err("86102562");
        let _nanos_error =
            super::StdTimeDurationNanos::try_from(1_000_000_000u32).expect_err("c342a3f2");
    }
}
