#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed(std::path::PathBuf),
    Unchanged(std::path::PathBuf),
}
impl WritePathOutcome {
    #[allow(clippy::single_call_fn)] // named conversion keeps enum->path mapping centralized for callers and tests
    #[must_use]
    pub fn into_path(self) -> std::path::PathBuf {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path,
        }
    }
    #[must_use]
    pub const fn is_changed(&self) -> bool {
        matches!(self, Self::Changed(_))
    }
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path.as_path(),
        }
    }
}
#[allow(clippy::single_call_fn)] // write-decision logic is split out to keep file write path minimal and focused
fn should_write_string_into_file(
    path: &std::path::Path,
    string_cnt: &str,
) -> std::io::Result<bool> {
    match std::fs::metadata(path) {
        Ok(v) => {
            let new_len_u64 = u64::try_from(string_cnt.len())
                .map_err(|_er| std::io::Error::other("2f4d7a8c failed converting string length"))?;
            if v.len() != new_len_u64 {
                return Ok(true);
            }
            std::fs::read_to_string(path).map(|old_cnt| old_cnt != string_cnt)
        }
        Err(er) if er.kind() == std::io::ErrorKind::NotFound => Ok(true),
        Err(er) => Err(er),
    }
}
#[allow(clippy::single_call_fn)]
// shared test helper checks unchanged-length diff content path to ensure content comparison still runs
#[cfg(test)]
fn should_write_with_same_len_diff(
    path: &std::path::Path,
    old: &str,
    new: &str,
) -> std::io::Result<bool> {
    if old.len() != new.len() {
        return Err(std::io::Error::other(
            "f4c1d7a9 same-len helper requires equal lengths",
        ));
    }
    std::fs::write(path, old)?;
    should_write_string_into_file(path, new)
}
#[allow(clippy::single_call_fn)] // shared test helper checks changed-length short-circuit path via metadata length comparison
#[cfg(test)]
fn should_write_with_diff_len(
    path: &std::path::Path,
    old: &str,
    new: &str,
) -> std::io::Result<bool> {
    if old.len() == new.len() {
        return Err(std::io::Error::other(
            "9a6d2c1b diff-len helper requires different lengths",
        ));
    }
    std::fs::write(path, old)?;
    should_write_string_into_file(path, new)
}
#[allow(clippy::single_call_fn)] // central mapping keeps changed/unchanged outcome construction consistent
const fn mk_write_path_outcome(path: std::path::PathBuf, is_changed: bool) -> WritePathOutcome {
    if is_changed {
        WritePathOutcome::Changed(path)
    } else {
        WritePathOutcome::Unchanged(path)
    }
}
#[allow(clippy::single_call_fn)] // extracted side-effect helper keeps write/no-write branching reusable and test-focused
fn write_string_if_needed(path: &std::path::Path, string_cnt: &str) -> std::io::Result<bool> {
    let should_write = should_write_string_into_file(path, string_cnt)?;
    if should_write {
        std::fs::write(path, string_cnt)?;
    }
    Ok(should_write)
}
#[allow(clippy::single_call_fn)] // preserves write/no-write state so callers can skip extra work (e.g. formatting) on unchanged files
pub(crate) fn try_write_string_into_path_with_outcome(
    path: impl AsRef<std::path::Path>,
    string_cnt: &str,
) -> std::io::Result<WritePathOutcome> {
    let path_ref = path.as_ref();
    let should_write = write_string_if_needed(path_ref, string_cnt)?;
    let path_buf = path_ref.to_path_buf();
    Ok(mk_write_path_outcome(path_buf, should_write))
}
#[allow(clippy::single_call_fn)] // shared write helper keeps change-outcome API reusable for extension-based write callers
pub fn try_write_string_into_file_with_outcome<P>(
    file_name: P,
    string_cnt: &str,
) -> std::io::Result<WritePathOutcome>
where
    P: AsRef<std::path::Path>,
{
    try_write_string_into_path_with_outcome(
        crate::rs_file_path::rs_file_path(file_name),
        string_cnt,
    )
}
#[allow(clippy::single_call_fn)] // shared write helper retains simple path-returning API used by file-level wrappers and tests
#[cfg(test)]
pub(crate) fn try_write_string_into_path(
    path: impl AsRef<std::path::Path>,
    string_cnt: &str,
) -> std::io::Result<std::path::PathBuf> {
    try_write_string_into_path_with_outcome(path, string_cnt).map(WritePathOutcome::into_path)
}
pub fn try_write_string_into_file<P>(
    file_name: P,
    string_cnt: &str,
) -> std::io::Result<std::path::PathBuf>
where
    P: AsRef<std::path::Path>,
{
    try_write_string_into_file_with_outcome(file_name, string_cnt).map(WritePathOutcome::into_path)
}
pub fn write_string_into_file<P>(file_name: P, string_cnt: &str)
where
    P: AsRef<std::path::Path>,
{
    let _pth = crate::panic_if_err::panic_if_err(
        try_write_string_into_file(file_name, string_cnt),
        |er| format!("4f3094e1:{er}"),
    );
}
#[cfg(test)]
mod tests {
    fn txt_path(name: &str) -> std::path::PathBuf {
        crate::test_hlp::test_path(name).with_extension("txt")
    }
    fn cleanup(path: &std::path::Path) {
        crate::test_hlp::cleanup_test_file(path);
    }
    fn assert_content_and_cleanup(path: &std::path::Path, expected: &str) {
        crate::test_hlp::assert_file_content(path, expected);
        cleanup(path);
    }
    fn assert_outcome_and_cleanup(
        path: &std::path::Path,
        outcome: &super::WritePathOutcome,
        expected_changed: bool,
    ) {
        assert_eq!(outcome.path(), path);
        assert_eq!(outcome.is_changed(), expected_changed);
        cleanup(path);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_content() {
        let path = txt_path("macros_helpers_write_path");
        let result_path = super::try_write_string_into_path(&path, "abc").expect("dcb22948");
        assert_eq!(result_path, path);
        assert_content_and_cleanup(path.as_path(), "abc");
    }
    #[test]
    fn write_string_into_file_adds_rs_extension() {
        let base = crate::test_hlp::test_path("macros_helpers_write_file");
        let path = crate::rs_file_path::rs_file_path(&base);
        super::write_string_into_file(&base, "xyz");
        assert_content_and_cleanup(path.as_path(), "xyz");
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = crate::test_hlp::test_path("macros_helpers_try_write_file");
        let path = super::try_write_string_into_file(&base, "qwe").expect("6676e082");
        assert_content_and_cleanup(path.as_path(), "qwe");
    }
    #[test]
    fn try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = txt_path("macros_helpers_try_write_path_passthrough");
        let result_path = super::try_write_string_into_path(&path, "abc").expect("b6b47a2c");
        assert_eq!(result_path, path);
        assert_content_and_cleanup(path.as_path(), "abc");
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_missing_file() {
        let path = txt_path("macros_helpers_should_write_missing");
        let should_write = super::should_write_string_into_file(&path, "abc").expect("f5d2cb68");
        assert!(should_write);
    }
    #[test]
    fn should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = txt_path("macros_helpers_should_write_same");
        std::fs::write(&path, "same").expect("68e4f52d");
        let should_write = super::should_write_string_into_file(&path, "same").expect("3e7adf2f");
        assert!(!should_write);
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_when_content_differs() {
        let path = txt_path("macros_helpers_should_write_diff");
        std::fs::write(&path, "old").expect("a2fd8473");
        let should_write = super::should_write_string_into_file(&path, "new").expect("52c9a1db");
        assert!(should_write);
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_same_len_diff_content() {
        let path = txt_path("macros_helpers_should_write_same_len_diff");
        let should_write =
            super::should_write_with_same_len_diff(&path, "abc", "xyz").expect("517fd0c9");
        assert!(should_write);
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_diff_len_content() {
        let path = txt_path("macros_helpers_should_write_diff_len");
        let should_write = super::should_write_with_diff_len(&path, "abcd", "a").expect("e2d99b73");
        assert!(should_write);
        cleanup(path.as_path());
    }
    #[test]
    fn write_string_if_needed_returns_false_without_rewrite_for_eq_content() {
        let path = txt_path("macros_helpers_write_if_needed_eq");
        std::fs::write(&path, "same").expect("924bdc58");
        let wrote = super::write_string_if_needed(&path, "same").expect("9f27b9cb");
        assert!(!wrote);
        assert_content_and_cleanup(path.as_path(), "same");
    }
    #[test]
    fn write_string_if_needed_returns_true_and_writes_for_diff_content() {
        let path = txt_path("macros_helpers_write_if_needed_diff");
        std::fs::write(&path, "old").expect("9b4ab8ad");
        let wrote = super::write_string_if_needed(&path, "new").expect("4e4ce16d");
        assert!(wrote);
        assert_content_and_cleanup(path.as_path(), "new");
    }
    #[test]
    fn path_with_rs_extension_accepts_path_input() {
        let path = crate::rs_file_path::rs_file_path(crate::test_hlp::test_path(
            "macros_helpers_rs_ext_path",
        ));
        assert_eq!(path.extension().and_then(|v| v.to_str()), Some("rs"));
    }
    #[test]
    fn try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = crate::test_hlp::test_path("macros_helpers_write_if_changed");
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, "same").expect("0242e1a9");
        let metadata_before = std::fs::metadata(&path).expect("974bc327");
        let _path = super::try_write_string_into_file(&base, "same").expect("07d9fd90");
        let metadata_after = std::fs::metadata(&path).expect("83087942");
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_content_and_cleanup(path.as_path(), "same");
    }
    #[test]
    fn try_write_string_into_file_writes_when_cnt_differs() {
        let base = crate::test_hlp::test_path("macros_helpers_write_if_changed_diff");
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, "old").expect("d870b82e");
        let _path = super::try_write_string_into_file(&base, "new").expect("c6fd2bc8");
        assert_content_and_cleanup(path.as_path(), "new");
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = txt_path("macros_helpers_write_outcome_changed");
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, "abc").expect("947faed1");
        crate::test_hlp::assert_file_content(&path, "abc");
        assert_outcome_and_cleanup(path.as_path(), &outcome, true);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = txt_path("macros_helpers_write_outcome_unchanged");
        std::fs::write(&path, "abc").expect("d293f783");
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, "abc").expect("b8f8eaf1");
        assert_outcome_and_cleanup(path.as_path(), &outcome, false);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_changed_and_rs_path() {
        let base = crate::test_hlp::test_path("macros_helpers_write_file_outcome_changed");
        let path = crate::rs_file_path::rs_file_path(&base);
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, "abc").expect("57cf209a");
        assert_eq!(outcome.path(), path.as_path());
        assert!(outcome.is_changed());
        assert_content_and_cleanup(path.as_path(), "abc");
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_unchanged_for_same_content() {
        let base = crate::test_hlp::test_path("macros_helpers_write_file_outcome_unchanged");
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, "abc").expect("2199f0a7");
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, "abc").expect("f60721a2");
        assert_eq!(outcome.path(), path.as_path());
        assert!(!outcome.is_changed());
        cleanup(path.as_path());
    }
    #[test]
    fn write_path_outcome_into_path_returns_owned_path() {
        let changed_path = txt_path("macros_helpers_write_outcome_into_path_changed");
        let changed = super::WritePathOutcome::Changed(changed_path.clone());
        assert_eq!(changed.into_path(), changed_path);
        let unchanged_path = txt_path("macros_helpers_write_outcome_into_path_unchanged");
        let unchanged = super::WritePathOutcome::Unchanged(unchanged_path.clone());
        assert_eq!(unchanged.into_path(), unchanged_path);
    }
}
