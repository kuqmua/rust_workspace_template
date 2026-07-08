#![allow(clippy::module_name_repetitions)]
const LOC_DISPLAY_UTC_OFFSET_SECS: i32 = 10_800;
const INCORRECT_DATETIME_MSG: &str = "incorrect datetime";
const LOC_FILE_MAX_LEN: usize = 1_048_576;
const LOC_COMMIT_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocFileTryFromStringEr {
    TooLong { len: usize, max: usize },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocCommitTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for LocFileTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "loc file length {len} exceeds maximum {max}")
            }
        }
    }
}
impl std::fmt::Display for LocCommitTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "loc commit length {len} exceeds maximum {max}")
            }
        }
    }
}
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
    newtype::Newtype,
)]
#[newtype(display)]
pub struct LocFile(String);
impl AsRef<str> for LocFile {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<LocFileTryFromStringEr> for LocFile {
    fn from(value: LocFileTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for LocFile {
    type Error = LocFileTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > LOC_FILE_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: LOC_FILE_MAX_LEN,
            });
        }
        Ok(Self(value))
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
    newtype::Newtype,
)]
#[newtype(display, from)]
pub struct LocLine(u32);
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
    newtype::Newtype,
)]
#[newtype(display, from)]
pub struct LocCol(u32);
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
pub struct LocCommit(String);
impl AsRef<str> for LocCommit {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<LocCommitTryFromStringEr> for LocCommit {
    fn from(value: LocCommitTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for LocCommit {
    type Error = LocCommitTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > LOC_COMMIT_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: LOC_COMMIT_MAX_LEN,
            });
        }
        Ok(Self(value))
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
    newtype::Newtype,
)]
#[newtype(from)]
pub struct StdLocDuration(std::time::Duration);
#[derive(Debug, Clone, Copy)]
struct LocFileRef<'file_lt>(pub &'file_lt str);
struct StdFmtRefMut<'fmt_ref_lt, 'fmt_lt>(pub &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>);
#[derive(Debug, Clone, Copy)]
struct ChronoLocDisplayTimezone(pub chrono::FixedOffset);
#[derive(Debug, Clone, Copy)]
struct ChronoLocDateTime(pub chrono::DateTime<chrono::FixedOffset>);
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
    pub file: LocFile,
    pub line: LocLine,
    pub col: LocCol,
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
    file: LocFile,
    commit: LocCommit,
    duration: StdLocDuration,
    occr: Option<Occr>,
    line: LocLine,
    col: LocCol,
}
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_pass_by_value)]
impl Loc {
    #[allow(clippy::single_call_fn)] // shared offset accessor is reused by formatter and tests
    fn loc_display_timezone() -> Option<ChronoLocDisplayTimezone> {
        chrono::FixedOffset::east_opt(LOC_DISPLAY_UTC_OFFSET_SECS).map(ChronoLocDisplayTimezone)
    }
    fn fmt_github_loc(
        &self,
        f: StdFmtRefMut<'_, '_>,
        file: LocFileRef<'_>,
        line: LocLine,
    ) -> std::fmt::Result {
        write!(
            f.0,
            "{}/blob/{}/{}#L{}",
            naming::GITHUB_URL,
            self.commit.as_ref(),
            file.0,
            line
        )
    }
    fn fmt_src_loc(
        f: StdFmtRefMut<'_, '_>,
        file: LocFileRef<'_>,
        line: LocLine,
        col: LocCol,
    ) -> std::fmt::Result {
        write!(f.0, "{}:{line}:{col}", file.0)
    }
    #[allow(clippy::single_call_fn)] // centralizes datetime + timezone composition so formatting can stay branch-light and tests can target conversion separately
    fn datetime_with_tz(&self) -> Option<ChronoLocDateTime> {
        let epoch = std::time::UNIX_EPOCH.checked_add(self.duration.0)?;
        let offset = Self::loc_display_timezone()?;
        Some(ChronoLocDateTime(
            chrono::DateTime::<chrono::Utc>::from(epoch).with_timezone(&offset.0),
        ))
    }
    fn fmt_datetime(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        match self.datetime_with_tz() {
            Some(v) => write!(f.0, "{}", v.0.format("%Y-%m-%d %H:%M:%S")),
            None => f.0.write_str(INCORRECT_DATETIME_MSG),
        }
    }
    fn fmt_github_place(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        self.fmt_github_loc(StdFmtRefMut(f.0), LocFileRef(self.file.as_ref()), self.line)?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(" (")?;
            self.fmt_github_loc(StdFmtRefMut(f.0), LocFileRef(v.file.as_ref()), v.line)?;
            f.0.write_str(")")
        } else {
            Ok(())
        }
    }
    fn fmt_place(
        &self,
        src_place_type: app_state::SrcPlaceType,
        f: StdFmtRefMut<'_, '_>,
    ) -> std::fmt::Result {
        match src_place_type {
            app_state::SrcPlaceType::Src => self.fmt_src_place(f),
            app_state::SrcPlaceType::Github => self.fmt_github_place(f),
        }
    }
    fn fmt_src_place(&self, f: StdFmtRefMut<'_, '_>) -> std::fmt::Result {
        Self::fmt_src_loc(
            StdFmtRefMut(f.0),
            LocFileRef(self.file.as_ref()),
            self.line,
            self.col,
        )?;
        if let Some(v) = self.occr.as_ref() {
            f.0.write_str(" (")?;
            Self::fmt_src_loc(
                StdFmtRefMut(f.0),
                LocFileRef(v.file.as_ref()),
                v.line,
                v.col,
            )?;
            f.0.write_str(")")
        } else {
            Ok(())
        }
    }
    #[must_use]
    pub fn new<FileTy, LineTy, ColTy>(
        file: FileTy,
        line: LineTy,
        col: ColTy,
        occr: Option<Occr>,
    ) -> Self
    where
        FileTy: AsRef<str>,
        LineTy: Into<LocLine>,
        ColTy: Into<LocCol>,
    {
        Self {
            file: LocFile::try_from(file.as_ref().to_owned()).unwrap_or_else(LocFile::from),
            line: line.into(),
            col: col.into(),
            commit: LocCommit::try_from(git_info::PROJECT_GIT_INFO.commit.as_ref().to_owned())
                .unwrap_or_else(LocCommit::from),
            duration: StdLocDuration(
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
#[derive(Debug, Clone, Copy, utoipa::ToSchema, optml::Optml)]
pub struct StdTimeDurationSecs(u64);
impl From<u64> for StdTimeDurationSecs {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for StdTimeDurationSecs {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, utoipa::ToSchema, optml::Optml)]
pub struct StdTimeDurationNanos(u32);
impl From<u32> for StdTimeDurationNanos {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for StdTimeDurationNanos {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_place(app_state::SrcPlaceType::from_env_or_dflt(), StdFmtRefMut(f))?;
        f.write_str(" ")?;
        self.fmt_datetime(StdFmtRefMut(f))
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
            self.loc.fmt_datetime(super::StdFmtRefMut(f))
        }
    }
    struct PlaceFmt<'loc_lt> {
        loc: &'loc_lt super::Loc,
        src_place_type: app_state::SrcPlaceType,
    }
    impl std::fmt::Display for PlaceFmt<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.loc
                .fmt_place(self.src_place_type, super::StdFmtRefMut(f))
        }
    }
    fn test_loc(duration: std::time::Duration, occr: Option<super::Occr>) -> super::Loc {
        super::Loc {
            file: super::LocFile::try_from(String::from("src/lib.rs"))
                .unwrap_or_else(super::LocFile::from),
            commit: super::LocCommit::try_from(String::from("abc123"))
                .unwrap_or_else(super::LocCommit::from),
            duration: super::StdLocDuration(duration),
            occr,
            line: super::LocLine(10),
            col: super::LocCol(20),
        }
    }
    fn test_occr() -> super::Occr {
        super::Occr {
            file: super::LocFile::try_from(String::from("src/er.rs"))
                .unwrap_or_else(super::LocFile::from),
            line: super::LocLine(30),
            col: super::LocCol(40),
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
            date_time.0.format("%Y-%m-%d %H:%M:%S").to_string(),
            "1970-01-01 03:00:00"
        );
    }
    #[test]
    fn loc_display_timezone_uses_expected_offset() {
        let offset = super::Loc::loc_display_timezone().expect("5c53d969");
        assert_eq!(
            offset.0.local_minus_utc(),
            super::LOC_DISPLAY_UTC_OFFSET_SECS
        );
    }
}
