const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const INCORRECT_DATETIME_MSG: &str = "incorrect datetime";
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
    pub file: String,
    pub line: u32,
    pub col: u32,
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
pub struct Loc {
    #[allow(clippy::arbitrary_source_item_ordering)]
    file: String,
    commit: String,
    duration: std::time::Duration,
    occr: Option<Occr>,
    line: u32,
    col: u32,
}
#[allow(clippy::arbitrary_source_item_ordering)]
impl Loc {
    #[allow(clippy::single_call_fn)] // shared offset accessor is reused by formatter and tests
    const fn loc_display_timezone() -> Option<chrono::FixedOffset> {
        chrono::FixedOffset::east_opt(LOC_DISPLAY_UTC_OFFSET_SECS)
    }
    fn fmt_with_occr(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        mut fmt_primary: impl FnMut(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
        mut fmt_occr: impl FnMut(&mut std::fmt::Formatter<'_>, &Occr) -> std::fmt::Result,
    ) -> std::fmt::Result {
        fmt_primary(f)?;
        if let Some(v) = self.occr.as_ref() {
            f.write_str(" (")?;
            fmt_occr(f, v)?;
            f.write_str(")")
        } else {
            Ok(())
        }
    }
    fn fmt_github_loc(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        file: &str,
        line: u32,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}/blob/{}/{}#L{}",
            naming::GITHUB_URL,
            self.commit,
            file,
            line
        )
    }
    fn fmt_src_loc(
        f: &mut std::fmt::Formatter<'_>,
        file: &str,
        line: u32,
        col: u32,
    ) -> std::fmt::Result {
        write!(f, "{file}:{line}:{col}")
    }
    #[allow(clippy::single_call_fn)] // centralizes datetime + timezone composition so formatting can stay branch-light and tests can target conversion separately
    fn datetime_with_tz(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration)?;
        let offset = Self::loc_display_timezone()?;
        Some(chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset))
    }
    fn fmt_datetime(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f, "{}", v.format("%Y-%m-%d %H:%M:%S")),
            None => f.write_str(INCORRECT_DATETIME_MSG),
        }
    }
    fn fmt_github_place(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_occr(
            f,
            |fmtr| self.fmt_github_loc(fmtr, &self.file, self.line),
            |fmtr, v| self.fmt_github_loc(fmtr, &v.file, v.line),
        )
    }
    fn fmt_place(
        &self,
        src_place_type: app_state::SrcPlaceType,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match src_place_type {
            app_state::SrcPlaceType::Src => self.fmt_src_place(f),
            app_state::SrcPlaceType::Github => self.fmt_github_place(f),
        }
    }
    fn fmt_src_place(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_occr(
            f,
            |fmtr| Self::fmt_src_loc(fmtr, &self.file, self.line, self.col),
            |fmtr, v| Self::fmt_src_loc(fmtr, &v.file, v.line, v.col),
        )
    }
    #[must_use]
    pub fn new(file: String, line: u32, col: u32, occr: Option<Occr>) -> Self {
        Self {
            file,
            line,
            col,
            commit: git_info::PROJECT_GIT_INFO.commit.to_owned(),
            duration: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default(),
            occr,
        }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, utoipa::ToSchema, optml::Optml)] //todo check somehow what its eq to std::time::Duration
pub struct StdTimeDuration {
    pub secs: u64,
    pub nanos: u32,
}
impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(app_state::SrcPlaceType::from_env_or_dflt(), f)?;
        f.write_str(" ")?;
        self.fmt_datetime(f)
    }
}
#[cfg(test)]
#[allow(clippy::arbitrary_source_item_ordering)]
mod tests {
    struct DatetimeFmt<'loc_lt> {
        loc: &'loc_lt super::Loc,
    }
    impl std::fmt::Display for DatetimeFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.loc.fmt_datetime(f)
        }
    }
    struct PlaceFmt<'loc_lt> {
        loc: &'loc_lt super::Loc,
        src_place_type: app_state::SrcPlaceType,
    }
    impl std::fmt::Display for PlaceFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.loc.fmt_place(self.src_place_type, f)
        }
    }
    fn test_loc(duration: std::time::Duration, occr: Option<super::Occr>) -> super::Loc {
        super::Loc {
            file: String::from("src/lib.rs"),
            commit: String::from("abc123"),
            duration,
            occr,
            line: 10,
            col: 20,
        }
    }
    fn test_occr() -> super::Occr {
        super::Occr {
            file: String::from("src/er.rs"),
            line: 30,
            col: 40,
        }
    }
    fn fmt_place(loc: &super::Loc, src_place_type: app_state::SrcPlaceType) -> String {
        format!(
            "{:}",
            PlaceFmt {
                loc,
                src_place_type
            }
        )
    }
    #[test]
    fn fmt_place_src_without_occr() {
        let loc = test_loc(std::time::Duration::from_secs(0), None);
        assert_eq!(
            fmt_place(&loc, app_state::SrcPlaceType::Src),
            "src/lib.rs:10:20"
        );
    }
    #[test]
    fn fmt_place_src_with_occr() {
        let loc = test_loc(std::time::Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&loc, app_state::SrcPlaceType::Src),
            "src/lib.rs:10:20 (src/er.rs:30:40)"
        );
    }
    #[test]
    fn fmt_place_github_without_occr() {
        let loc = test_loc(std::time::Duration::from_secs(0), None);
        assert_eq!(
            fmt_place(&loc, app_state::SrcPlaceType::Github),
            format!("{}/blob/abc123/src/lib.rs#L10", naming::GITHUB_URL)
        );
    }
    #[test]
    fn fmt_place_github_with_occr() {
        let loc = test_loc(std::time::Duration::from_secs(0), Some(test_occr()));
        assert_eq!(
            fmt_place(&loc, app_state::SrcPlaceType::Github),
            format!(
                "{}/blob/abc123/src/lib.rs#L10 ({}/blob/abc123/src/er.rs#L30)",
                naming::GITHUB_URL,
                naming::GITHUB_URL
            )
        );
    }
    #[test]
    fn fmt_datetime_returns_fallback_for_overflowed_duration() {
        let loc = test_loc(std::time::Duration::MAX, None);
        assert_eq!(
            format!("{}", DatetimeFmt { loc: &loc }),
            super::INCORRECT_DATETIME_MSG
        );
    }
    #[test]
    fn datetime_with_tz_returns_expected_epoch_time_for_zero_duration() {
        let loc = test_loc(std::time::Duration::from_secs(0), None);
        let date_time = loc.datetime_with_tz().expect("f5c41dd8");
        assert_eq!(
            date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            "1970-01-01 03:00:00"
        );
    }
    #[test]
    fn loc_display_timezone_uses_expected_offset() {
        let offset = super::Loc::loc_display_timezone().expect("5c53d969");
        assert_eq!(offset.local_minus_utc(), super::LOC_DISPLAY_UTC_OFFSET_SECS);
    }
}
