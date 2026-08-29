#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

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
    pub(super) file: crate::location_file::LocationFile,
    pub(super) commit: crate::location_commit::LocationCommit,
    pub(super) duration: crate::location_duration::LocationDuration,
    pub(super) occr: Option<crate::occr::Occr>,
    pub(super) line: crate::location_line::LocationLine,
    pub(super) column: crate::location_column::LocationColumn,
}
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_pass_by_value)]
impl Location {
    fn fmt_github_location(
        &self,
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
        file: crate::location_file_ref::LocationFileRef<'_>,
        line: crate::location_line::LocationLine,
    ) -> std::fmt::Result {
        write!(
            f.0,
            "{}/blob/{}/{}#L{}",
            constants_str::catalog::NAMING_GITHUB_URL,
            self.commit.as_ref(),
            file.0,
            line
        )
    }
    fn fmt_src_location(
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
        file: crate::location_file_ref::LocationFileRef<'_>,
        line: crate::location_line::LocationLine,
        column: crate::location_column::LocationColumn,
    ) -> std::fmt::Result {
        write!(f.0, "{}:{line}:{column}", file.0)
    }
    pub(super) fn datetime_with_tz(
        &self,
    ) -> Option<crate::chrono_location_date_time::ChronoLocationDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration.0)?;
        let offset =
            chrono::FixedOffset::east_opt(crate::domain_types::LOC_DISPLAY_UTC_OFFSET_SECS)
                .map(crate::chrono_location_display_timezone::ChronoLocationDisplayTimezone)?;
        Some(
            crate::chrono_location_date_time::ChronoLocationDateTime::from(
                chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset.0),
            ),
        )
    }
    pub(super) fn fmt_datetime(
        &self,
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f.0, "{}", v.0.format("%Y-%m-%d %H:%M:%S")),
            None => {
                f.0.write_str(constants_str::catalog::LOCATION_INCORRECT_DATETIME_MSG)
            }
        }
    }
    pub(super) fn fmt_place(
        &self,
        src_place_type: config_lib::src_place_type::SrcPlaceType,
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        match src_place_type {
            config_lib::src_place_type::SrcPlaceType::Src => {
                Self::fmt_src_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *f.0),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                    self.column,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    f.0.write_str(constants_str::catalog::TEXT)?;
                    Self::fmt_src_location(
                        crate::formatter_ref_mut::FormatterRefMut::from(&mut *f.0),
                        crate::location_file_ref::LocationFileRef::from(v.file.as_ref()),
                        v.line,
                        v.column,
                    )?;
                    f.0.write_str(constants_str::catalog::TEXT_ALT_5)
                } else {
                    Ok(())
                }
            }
            config_lib::src_place_type::SrcPlaceType::Github => {
                self.fmt_github_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *f.0),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    f.0.write_str(constants_str::catalog::TEXT)?;
                    self.fmt_github_location(
                        crate::formatter_ref_mut::FormatterRefMut::from(&mut *f.0),
                        crate::location_file_ref::LocationFileRef::from(v.file.as_ref()),
                        v.line,
                    )?;
                    f.0.write_str(constants_str::catalog::TEXT_ALT_5)
                } else {
                    Ok(())
                }
            }
        }
    }
    #[must_use]
    pub fn new<FileTy>(
        file: FileTy,
        line: crate::location_line::LocationLine,
        column: crate::location_column::LocationColumn,
        occr: Option<crate::occr::Occr>,
    ) -> Self
    where
        FileTy: AsRef<str>,
    {
        Self {
            file: crate::location_file::LocationFile::try_from(file.as_ref().to_owned())
                .unwrap_or_else(crate::location_file::LocationFile::from),
            line,
            column,
            commit: crate::location_commit::LocationCommit::try_from(
                git_info::project_git_info_value::project_git_info_value()
                    .commit()
                    .as_ref()
                    .to_owned(),
            )
            .unwrap_or_else(crate::location_commit::LocationCommit::from),
            duration: crate::location_duration::LocationDuration::from(
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
            config_lib::src_place_type::SrcPlaceType::from_env_or_default(),
            crate::formatter_ref_mut::FormatterRefMut::from(&mut *f),
        )?;
        f.write_str(constants_str::catalog::SPACE)?;
        self.fmt_datetime(crate::formatter_ref_mut::FormatterRefMut::from(&mut *f))
    }
}
