#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "location keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct Location {
    #[allow(
        clippy::arbitrary_source_item_ordering,
        reason = "location keeps declaration order aligned with generated layout or processing flow"
    )]
    file: crate::location_file::LocationFile,
    commit: crate::location_commit::LocationCommit,
    duration: crate::location_duration::LocationDuration,
    occurrence: Option<crate::occurrence::Occurrence>,
    line: crate::location_line::LocationLine,
    column: crate::location_column::LocationColumn,
}

#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_pass_by_value,
    reason = "location keeps declaration order aligned with generated layout or processing flow"
)]
impl Location {
    fn fmt_github_location(
        &self,
        formatter_ref_mut: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
        location_file_ref: crate::location_file_ref::LocationFileRef<'_>,
        location_line: crate::location_line::LocationLine,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = formatter_ref_mut.into();
        write!(
            formatter,
            "{}/blob/{}/{}#L{}",
            constants_str::NAMING_GITHUB_URL,
            self.commit.as_ref(),
            <&str>::from(location_file_ref),
            location_line
        )
    }
    fn fmt_src_location(
        formatter_ref_mut: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
        location_file_ref: crate::location_file_ref::LocationFileRef<'_>,
        location_line: crate::location_line::LocationLine,
        location_column: crate::location_column::LocationColumn,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = formatter_ref_mut.into();
        write!(
            formatter,
            "{}:{location_line}:{location_column}",
            <&str>::from(location_file_ref)
        )
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
        formatter_ref_mut: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = formatter_ref_mut.into();
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
        source_place_type: config_lib::source_place_type::SourcePlaceType,
        formatter_ref_mut: crate::formatter_ref_mut::FormatterRefMut<'_, '_>,
    ) -> std::fmt::Result {
        let formatter: &mut std::fmt::Formatter<'_> = formatter_ref_mut.into();
        match source_place_type {
            config_lib::source_place_type::SourcePlaceType::Src => {
                Self::fmt_src_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                    self.column,
                )?;
                if let Some(v) = self.occurrence.as_ref() {
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
            config_lib::source_place_type::SourcePlaceType::Github => {
                self.fmt_github_location(
                    crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
                    crate::location_file_ref::LocationFileRef::from(self.file.as_ref()),
                    self.line,
                )?;
                if let Some(v) = self.occurrence.as_ref() {
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
        file_ty: FileTy,
        location_line: crate::location_line::LocationLine,
        location_column: crate::location_column::LocationColumn,
        option: Option<crate::occurrence::Occurrence>,
    ) -> Self
    where
        FileTy: AsRef<str>,
    {
        Self {
            file: crate::location_file::LocationFile::try_from(file_ty.as_ref().to_owned())
                .unwrap_or_else(crate::location_file::LocationFile::from),
            line: location_line,
            column: location_column,
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
            occurrence: option,
        }
    }
}
#[cfg(test)]
impl
    From<(
        crate::location_file::LocationFile,
        crate::location_commit::LocationCommit,
        crate::location_duration::LocationDuration,
        Option<crate::occurrence::Occurrence>,
        crate::location_line::LocationLine,
        crate::location_column::LocationColumn,
    )> for Location
{
    fn from(
        value: (
            crate::location_file::LocationFile,
            crate::location_commit::LocationCommit,
            crate::location_duration::LocationDuration,
            Option<crate::occurrence::Occurrence>,
            crate::location_line::LocationLine,
            crate::location_column::LocationColumn,
        ),
    ) -> Self {
        Self {
            file: value.0,
            commit: value.1,
            duration: value.2,
            occurrence: value.3,
            line: value.4,
            column: value.5,
        }
    }
}
impl std::fmt::Display for Location {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(
            config_lib::source_place_type::SourcePlaceType::from_env_or_default(),
            crate::formatter_ref_mut::FormatterRefMut::from(&mut *formatter),
        )?;
        formatter.write_str(constants_str::SPACE)?;
        self.fmt_datetime(crate::formatter_ref_mut::FormatterRefMut::from(
            &mut *formatter,
        ))
    }
}
