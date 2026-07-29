#[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct StdWrittenFilePath(std::path::PathBuf);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct StdWrittenFilePathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct StringFileContentRef<'cnt_lt>(&'cnt_lt str);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
struct GeneratedFileMaximumBytes(usize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner, newtype::IntoInnerFrom, newtype::NotInner,
)]
pub struct ShouldWriteString(bool);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed(StdWrittenFilePath),
    Unchanged(StdWrittenFilePath),
}
impl WritePathOutcome {
    #[must_use]
    pub fn into_path(self) -> StdWrittenFilePath {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path,
        }
    }
    #[must_use]
    pub fn is_changed(&self) -> ShouldWriteString {
        ShouldWriteString::from(matches!(self, Self::Changed(_)))
    }
    #[must_use]
    pub fn path(&self) -> StdWrittenFilePathRef<'_> {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => {
                StdWrittenFilePathRef::from(path.as_ref())
            }
        }
    }
}
fn validate_existing_file_text(
    path: StdWrittenFilePathRef<'_>,
    maximum_bytes: GeneratedFileMaximumBytes,
) -> std::io::Result<()> {
    server_runtime_http::read_bounded_file(
        server_runtime_http::StdPathRef::from(path.as_ref()),
        server_runtime_http::BoundedReadMaximumBytes::from(maximum_bytes.0),
    )
    .and_then(server_runtime_http::BoundedText::try_from)
    .map(|_text| ())
    .map_err(std::io::Error::other)
}
#[allow(clippy::single_call_fn)] // write-decision logic is split out to keep file write path minimal and focused
fn should_write_string_into_file(
    path: StdWrittenFilePathRef<'_>,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<ShouldWriteString> {
    let path_ref = path.as_ref();
    let string_cnt_ref = string_cnt.as_ref();
    match std::fs::metadata(path_ref) {
        Ok(v) => {
            let new_len_u64 = u64::try_from(string_cnt_ref.len()).map_err(|_error| {
                std::io::Error::other(str_constants::VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH)
            })?;
            if v.len() != new_len_u64 {
                return Ok(ShouldWriteString::from(true));
            }
            let mut old_file = std::fs::File::open(path_ref)?;
            let mut offset = 0usize;
            let mut old_chunk = [0u8; 8192];
            loop {
                let read_len = std::io::Read::read(&mut old_file, &mut old_chunk)?;
                if read_len == 0usize {
                    if offset == string_cnt_ref.len() {
                        return Ok(ShouldWriteString::from(false));
                    }
                    validate_existing_file_text(
                        path,
                        GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(ShouldWriteString::from(true));
                }
                let end = offset.checked_add(read_len).ok_or_else(|| {
                    std::io::Error::other(
                        str_constants::VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW,
                    )
                })?;
                let Some(new_chunk) = string_cnt_ref.as_bytes().get(offset..end) else {
                    validate_existing_file_text(
                        path,
                        GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(ShouldWriteString::from(true));
                };
                let Some(old_chunk_read) = old_chunk.get(..read_len) else {
                    return Err(std::io::Error::other(str_constants::F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER));
                };
                if old_chunk_read != new_chunk {
                    validate_existing_file_text(
                        path,
                        GeneratedFileMaximumBytes::from(string_cnt_ref.len()),
                    )?;
                    return Ok(ShouldWriteString::from(true));
                }
                offset = end;
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(ShouldWriteString::from(true))
        }
        Err(error) => Err(error),
    }
}
#[allow(clippy::single_call_fn)] // extracted side-effect helper keeps write/no-write branching reusable and test-focused
fn write_string_if_needed(
    path: StdWrittenFilePathRef<'_>,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<ShouldWriteString> {
    let should_write = should_write_string_into_file(path, string_cnt)?;
    if bool::from(should_write) {
        let mut file = atomic_write_file::AtomicWriteFile::open(path.as_ref())?;
        std::io::Write::write_all(&mut file, string_cnt.as_ref().as_bytes())?;
        file.commit()?;
    }
    Ok(should_write)
}
#[allow(clippy::single_call_fn)] // preserves write/no-write state so callers can skip extra work (e.g. formatting) on unchanged files
pub(crate) fn try_write_string_into_path_with_outcome(
    path: impl AsRef<std::path::Path>,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<WritePathOutcome> {
    let path_ref = path.as_ref();
    let should_write = write_string_if_needed(StdWrittenFilePathRef::from(path_ref), string_cnt)?;
    let path_buf = StdWrittenFilePath::from(path_ref.to_path_buf());
    Ok(if bool::from(should_write) {
        WritePathOutcome::Changed(path_buf)
    } else {
        WritePathOutcome::Unchanged(path_buf)
    })
}
pub fn try_write_string_into_file_with_outcome<P>(
    file_name: P,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<WritePathOutcome>
where
    P: AsRef<std::path::Path>,
{
    try_write_string_into_path_with_outcome(
        crate::rs_file_path::rs_file_path(file_name),
        string_cnt,
    )
}
#[cfg(test)]
pub(crate) fn try_write_string_into_path(
    path: impl AsRef<std::path::Path>,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<StdWrittenFilePath> {
    try_write_string_into_path_with_outcome(path, string_cnt).map(WritePathOutcome::into_path)
}
pub fn try_write_string_into_file<P>(
    file_name: P,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<StdWrittenFilePath>
where
    P: AsRef<std::path::Path>,
{
    try_write_string_into_file_with_outcome(file_name, string_cnt).map(WritePathOutcome::into_path)
}
#[cfg(test)]
mod tests {
    fn cnt(v: &str) -> super::StringFileContentRef<'_> {
        super::StringFileContentRef::from(v)
    }
    fn path_ref(v: &std::path::Path) -> super::StdWrittenFilePathRef<'_> {
        super::StdWrittenFilePathRef::from(v)
    }
    fn written_path(v: std::path::PathBuf) -> super::StdWrittenFilePath {
        super::StdWrittenFilePath::from(v)
    }
    fn txt_path(name: &str) -> std::path::PathBuf {
        crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(name))
            .as_ref()
            .with_extension(str_constants::TXT)
    }
    fn cleanup(path: &std::path::Path) {
        crate::test_hlp::cleanup_test_file(path);
    }
    fn assert_content_and_cleanup(path: &std::path::Path, expected: &str) {
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(path),
            crate::test_hlp::ExpectedFileContent::new(expected),
        );
        cleanup(path);
    }
    fn assert_outcome_and_cleanup(
        path: &std::path::Path,
        outcome: &super::WritePathOutcome,
        expected_changed: bool,
    ) {
        assert_eq!(outcome.path().as_ref(), path);
        assert_eq!(bool::from(outcome.is_changed()), expected_changed);
        cleanup(path);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_WRITE_PATH);
        let result_path = super::try_write_string_into_path(&path, cnt(str_constants::ABC_ALT_3))
            .expect("dcb22948");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), str_constants::ABC_ALT_3);
    }
    #[test]
    fn write_string_into_file_adds_rs_extension() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE_FILE,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let _path =
            super::try_write_string_into_file(&base, cnt(str_constants::XYZ)).expect("4f3094e1");
        assert_content_and_cleanup(path.as_ref(), str_constants::XYZ);
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_TRY_WRITE_FILE,
        ));
        let path =
            super::try_write_string_into_file(&base, cnt(str_constants::QWE)).expect("6676e082");
        assert_content_and_cleanup(path.as_ref(), str_constants::QWE);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = txt_path(str_constants::MACROS_HELPERS_TRY_WRITE_PATH_PASSTHROUGH);
        let result_path = super::try_write_string_into_path(&path, cnt(str_constants::ABC_ALT_3))
            .expect("b6b47a2c");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), str_constants::ABC_ALT_3);
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_missing_file() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_MISSING);
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(str_constants::ABC_ALT_3))
                .expect("f5d2cb68");
        assert!(bool::from(should_write));
    }
    #[test]
    fn should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_SAME);
        std::fs::write(&path, str_constants::SAME).expect("68e4f52d");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(str_constants::SAME))
                .expect("3e7adf2f");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_compares_equal_content_in_chunks() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_LARGE_SAME);
        let content = str_constants::ABCD_ALT.repeat(4097usize);
        std::fs::write(&path, &content).expect("1d706d27");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(&content)).expect("d6619712");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_finds_diff_after_first_chunk() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_LARGE_DIFF);
        let old_content = str_constants::A_ALT.repeat(16_388usize);
        let mut new_content = old_content.clone();
        new_content.replace_range(16_387usize.., str_constants::B);
        std::fs::write(&path, old_content).expect("abfd8fbc");
        let should_write = super::should_write_string_into_file(path_ref(&path), cnt(&new_content))
            .expect("a3040fa0");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_when_content_differs() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_DIFF);
        std::fs::write(&path, str_constants::OLD).expect("a2fd8473");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(str_constants::NEW))
                .expect("52c9a1db");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_same_len_diff_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF);
        std::fs::write(&path, str_constants::ABC_ALT_3).expect("517fd0c9");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(str_constants::XYZ))
                .expect("a82c48b8");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_diff_len_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_SHOULD_WRITE_DIFF_LEN);
        std::fs::write(&path, str_constants::ABCD_ALT).expect("e2d99b73");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(str_constants::A_ALT))
                .expect("157e8cad");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn write_string_if_needed_returns_false_without_rewrite_for_eq_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_WRITE_IF_NEEDED_EQ);
        std::fs::write(&path, str_constants::SAME).expect("924bdc58");
        let wrote = super::write_string_if_needed(path_ref(&path), cnt(str_constants::SAME))
            .expect("9f27b9cb");
        assert!(!bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), str_constants::SAME);
    }
    #[test]
    fn write_string_if_needed_returns_true_and_writes_for_diff_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_WRITE_IF_NEEDED_DIFF);
        std::fs::write(&path, str_constants::OLD).expect("9b4ab8ad");
        let wrote = super::write_string_if_needed(path_ref(&path), cnt(str_constants::NEW))
            .expect("4e4ce16d");
        assert!(bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), str_constants::NEW);
    }
    #[test]
    fn path_with_rs_extension_accepts_path_input() {
        let path = crate::rs_file_path::rs_file_path(crate::test_hlp::test_path(
            crate::test_hlp::TestPathStem::new(str_constants::MACROS_HELPERS_RS_EXT_PATH),
        ));
        assert_eq!(
            path.as_ref().extension().and_then(|v| v.to_str()),
            Some("rs")
        );
    }
    #[test]
    fn try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE_IF_CHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, str_constants::SAME).expect("0242e1a9");
        let metadata_before = std::fs::metadata(&path).expect("974bc327");
        let _path =
            super::try_write_string_into_file(&base, cnt(str_constants::SAME)).expect("07d9fd90");
        let metadata_after = std::fs::metadata(&path).expect("83087942");
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_content_and_cleanup(path.as_ref(), str_constants::SAME);
    }
    #[test]
    fn try_write_string_into_file_writes_when_cnt_differs() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE_IF_CHANGED_DIFF,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, str_constants::OLD).expect("d870b82e");
        let _path =
            super::try_write_string_into_file(&base, cnt(str_constants::NEW)).expect("c6fd2bc8");
        assert_content_and_cleanup(path.as_ref(), str_constants::NEW);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_WRITE_OUTCOME_CHANGED);
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, cnt(str_constants::ABC_ALT_3))
                .expect("947faed1");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(&path),
            crate::test_hlp::ExpectedFileContent::new(str_constants::ABC_ALT_3),
        );
        assert_outcome_and_cleanup(path.as_path(), &outcome, true);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = txt_path(str_constants::MACROS_HELPERS_WRITE_OUTCOME_UNCHANGED);
        std::fs::write(&path, str_constants::ABC_ALT_3).expect("d293f783");
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, cnt(str_constants::ABC_ALT_3))
                .expect("b8f8eaf1");
        assert_outcome_and_cleanup(path.as_path(), &outcome, false);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_changed_and_rs_path() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, cnt(str_constants::ABC_ALT_3))
                .expect("57cf209a");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(bool::from(outcome.is_changed()));
        assert_content_and_cleanup(path.as_ref(), str_constants::ABC_ALT_3);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_unchanged_for_same_content() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            str_constants::MACROS_HELPERS_WRITE_FILE_OUTCOME_UNCHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, str_constants::ABC_ALT_3).expect("2199f0a7");
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, cnt(str_constants::ABC_ALT_3))
                .expect("f60721a2");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(!bool::from(outcome.is_changed()));
        cleanup(path.as_ref());
    }
    #[test]
    fn write_path_outcome_into_path_returns_owned_path() {
        let changed_path = txt_path(str_constants::MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_CHANGED);
        let changed = super::WritePathOutcome::Changed(written_path(changed_path.clone()));
        assert_eq!(changed.into_path(), written_path(changed_path));
        let unchanged_path =
            txt_path(str_constants::MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_UNCHANGED);
        let unchanged = super::WritePathOutcome::Unchanged(written_path(unchanged_path.clone()));
        assert_eq!(unchanged.into_path(), written_path(unchanged_path));
    }
}
