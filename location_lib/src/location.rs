#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    ChronoLocationDateTime, ChronoLocationDisplayTimezone, FormatterRefMut,
    LOC_DISPLAY_UTC_OFFSET_SECS, LocationColumn, LocationCommit, LocationDuration, LocationFile,
    LocationFileRef, LocationLine, Occr,
};

// The owner module retains lint-sensitive semantics from the original implementation.
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
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(clippy::arbitrary_source_item_ordering)]
    pub(super) file: LocationFile,
    pub(super) commit: LocationCommit,
    pub(super) duration: LocationDuration,
    pub(super) occr: Option<Occr>,
    pub(super) line: LocationLine,
    pub(super) column: LocationColumn,
}
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_pass_by_value)]
impl Location {
    #[allow(clippy::single_call_fn)] // shared offset accessor is reused by formatter and tests
    pub(super) fn location_display_timezone() -> Option<ChronoLocationDisplayTimezone> {
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
    pub(super) fn datetime_with_tz(&self) -> Option<ChronoLocationDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration.0)?;
        let offset = Self::location_display_timezone()?;
        Some(ChronoLocationDateTime::from(
            chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset.0),
        ))
    }
    pub(super) fn fmt_datetime(&self, f: FormatterRefMut<'_, '_>) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f.0, "{}", v.0.format("%Y-%m-%d %H:%M:%S")),
            None => {
                f.0.write_str(constants_str::LOCATION_INCORRECT_DATETIME_MSG)
            }
        }
    }
    pub(super) fn fmt_place(
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
                git_info::domain_types::project_git_info_value()
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
