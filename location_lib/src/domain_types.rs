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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
        if *value == constants_u32::ZERO {
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
    optimal_memory_layout::OptimalMemoryLayout,
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
        if *value == constants_u32::ZERO {
            Err(LocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
#[serde(from = "std::time::Duration")]
pub struct LocationDuration(std::time::Duration);
impl utoipa::PartialSchema for LocationDuration {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::SECS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int64,
                    ))),
            )
            .property(
                constants_str::NANOS,
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                        utoipa::openapi::KnownFormat::Int32,
                    )))
                    .minimum(Some(0.0))
                    .maximum(Some(999_999_999.0)),
            )
            .required(constants_str::SECS)
            .required(constants_str::NANOS)
            .build()
            .into()
    }
}
impl utoipa::ToSchema for LocationDuration {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::STDLOCATIONDURATION)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct LocationFileRef<'file_lt>(&'file_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct FormatterRefMut<'fmt_ref_lt, 'fmt_lt>(&'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct ChronoLocationDisplayTimezone(chrono::FixedOffset);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
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
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct Location {
    #[allow(clippy::arbitrary_source_item_ordering)]
    file: LocationFile,
    commit: LocationCommit,
    duration: LocationDuration,
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
        f: FormatterRefMut<'_, '_>,
        file: LocationFileRef<'_>,
        line: LocationLine,
    ) -> std::fmt::Result {
        write!(
            f.0,
            "{}/blob/{}/{}#L{}",
            constants_str::NAMING_GITHUB_URL,
            self.commit.as_ref(),
            file.0,
            line
        )
    }
    fn fmt_src_location(
        f: FormatterRefMut<'_, '_>,
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
    fn fmt_datetime(&self, f: FormatterRefMut<'_, '_>) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f.0, "{}", v.0.format("%Y-%m-%d %H:%M:%S")),
            None => {
                f.0.write_str(constants_str::LOCATION_INCORRECT_DATETIME_MSG)
            }
        }
    }
    fn fmt_place(
        &self,
        src_place_type: config_lib::domain_types::types::SrcPlaceType,
        f: FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        match src_place_type {
            config_lib::domain_types::types::SrcPlaceType::Src => {
                Self::fmt_src_location(
                    FormatterRefMut::from(&mut *f.0),
                    LocationFileRef::from(self.file.as_ref()),
                    self.line,
                    self.column,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    f.0.write_str(constants_str::TEXT)?;
                    Self::fmt_src_location(
                        FormatterRefMut::from(&mut *f.0),
                        LocationFileRef::from(v.file.as_ref()),
                        v.line,
                        v.column,
                    )?;
                    f.0.write_str(constants_str::TEXT_ALT_5)
                } else {
                    Ok(())
                }
            }
            config_lib::domain_types::types::SrcPlaceType::Github => {
                self.fmt_github_location(
                    FormatterRefMut::from(&mut *f.0),
                    LocationFileRef::from(self.file.as_ref()),
                    self.line,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    f.0.write_str(constants_str::TEXT)?;
                    self.fmt_github_location(
                        FormatterRefMut::from(&mut *f.0),
                        LocationFileRef::from(v.file.as_ref()),
                        v.line,
                    )?;
                    f.0.write_str(constants_str::TEXT_ALT_5)
                } else {
                    Ok(())
                }
            }
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
                git_info::domain_types::project_git_info()
                    .commit()
                    .as_ref()
                    .to_owned(),
            )
            .unwrap_or_else(LocationCommit::from),
            duration: LocationDuration::from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default(),
            ),
            occr,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, utoipa::ToSchema, optimal_memory_layout::OptimalMemoryLayout)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    pub secs: StdTimeDurationSecs,
    pub nanos: StdTimeDurationNanos,
}
#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct StdTimeDurationSecs(u64);
#[derive(
    Debug,
    Clone,
    Copy,
    utoipa::ToSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::TryFrom,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct StdTimeDurationNanosTryFromU32Error;
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(
            config_lib::domain_types::types::SrcPlaceType::from_env_or_default(),
            FormatterRefMut::from(&mut *f),
        )?;
        f.write_str(constants_str::SPACE)?;
        self.fmt_datetime(FormatterRefMut::from(&mut *f))
    }
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)]
#[path = "tests.rs"]
mod tests;
