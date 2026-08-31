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
    file: crate::location_file::LocationFile,
    commit: crate::location_commit::LocationCommit,
    duration: crate::location_duration::LocationDuration,
    occr: Option<crate::occr::Occr>,
    line: crate::location_line::LocationLine,
    column: crate::location_column::LocationColumn,
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
        let formatter: &mut std::fmt::Formatter<'_> = f.into();
        write!(
            formatter,
            "{}/blob/{}/{}#L{}",
            constants_str::NAMING_GITHUB_URL,
            self.commit.as_ref(),
            <&str>::from(file),
            line
        )
    }
    fn fmt_src_location(
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
        file: crate::location_file_ref::LocationFileRef<'_>,
        line: crate::location_line::LocationLine,
        column: crate::location_column::LocationColumn,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = f.into();
        write!(formatter, "{}:{line}:{column}", <&str>::from(file))
    }
    pub(super) fn datetime_with_tz(
        &self,
    ) -> Option<crate::chrono_location_date_time::ChronoLocationDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(std::time::Duration::from(self.duration))?;
        let offset = chrono::FixedOffset::east_opt(
            crate::domain_types::LOC_DISPLAY_UTC_OFFSET_SECS,
        )
        .map(crate::chrono_location_display_timezone::ChronoLocationDisplayTimezone::from)?;
        Some(
            crate::chrono_location_date_time::ChronoLocationDateTime::from(
                chrono::DateTime::<chrono::Utc>::from(epoch)
                    .with_timezone(&chrono::FixedOffset::from(offset)),
            ),
        )
    }
    pub(super) fn fmt_datetime(
        &self,
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = f.into();
        match self.datetime_with_tz() {
            Some(v) => write!(
                formatter,
                "{}",
                chrono::DateTime::<chrono::FixedOffset>::from(v).format("%Y-%m-%d %H:%M:%S")
            ),
            None => formatter.write_str(constants_str::LOCATION_INCORRECT_DATETIME_MSG),
        }
    }
    pub(super) fn fmt_place(
        &self,
        src_place_type: config_lib::src_place_type::SrcPlaceType,
        f: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = f.into();
        match src_place_type {
            config_lib::src_place_type::SrcPlaceType::Src => {
                Self::fmt_src_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                    self.column,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    formatter.write_str(constants_str::TEXT)?;
                    Self::fmt_src_location(
                        crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                        crate::location_file_ref::LocationFileRef::from(v.get_file().as_ref()),
                        *v.get_line(),
                        *v.get_column(),
                    )?;
                    formatter.write_str(constants_str::TEXT_ALT_5)
                } else {
                    Ok(())
                }
            }
            config_lib::src_place_type::SrcPlaceType::Github => {
                self.fmt_github_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                )?;
                if let Some(v) = self.occr.as_ref() {
                    formatter.write_str(constants_str::TEXT)?;
                    self.fmt_github_location(
                        crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                        crate::location_file_ref::LocationFileRef::from(v.get_file().as_ref()),
                        *v.get_line(),
                    )?;
                    formatter.write_str(constants_str::TEXT_ALT_5)
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
#[cfg(test)]
impl
    From<(
        crate::location_file::LocationFile,
        crate::location_commit::LocationCommit,
        crate::location_duration::LocationDuration,
        Option<crate::occr::Occr>,
        crate::location_line::LocationLine,
        crate::location_column::LocationColumn,
    )> for Location
{
    fn from(
        value: (
            crate::location_file::LocationFile,
            crate::location_commit::LocationCommit,
            crate::location_duration::LocationDuration,
            Option<crate::occr::Occr>,
            crate::location_line::LocationLine,
            crate::location_column::LocationColumn,
        ),
    ) -> Self {
        Self {
            file: value.0,
            commit: value.1,
            duration: value.2,
            occr: value.3,
            line: value.4,
            column: value.5,
        }
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(
            config_lib::src_place_type::SrcPlaceType::from_env_or_default(),
            crate::formatter_ref_mut::FormatterRefMut::from(&mut *f),
        )?;
        f.write_str(constants_str::SPACE)?;
        self.fmt_datetime(crate::formatter_ref_mut::FormatterRefMut::from(&mut *f))
    }
}
