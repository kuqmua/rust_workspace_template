#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct WrittenFilePathBuf(std::path::PathBuf);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct WrittenFilePathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct StringFileContentRef<'cnt_lt>(&'cnt_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
struct GeneratedFileMaximumBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::NotInner,
)]
pub struct ShouldWriteString(bool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed(WrittenFilePathBuf),
    Unchanged(WrittenFilePathBuf),
}
impl WritePathOutcome {
    #[must_use]
    pub fn into_path(self) -> WrittenFilePathBuf {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path,
        }
    }
    #[must_use]
    pub fn is_changed(&self) -> ShouldWriteString {
        ShouldWriteString::from(matches!(self, Self::Changed(_)))
    }
    #[must_use]
    pub fn path(&self) -> WrittenFilePathRef<'_> {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => WrittenFilePathRef::from(path.as_ref()),
        }
    }
}
fn validate_existing_file_text(
    path: WrittenFilePathRef<'_>,
    maximum_bytes: GeneratedFileMaximumBytes,
) -> std::io::Result<()> {
    server_runtime_http::read_bounded_file(
        server_runtime_http::PathRef::from(path.as_ref()),
        server_runtime_http::BoundedReadMaximumBytes::from(maximum_bytes.0),
    )
    .and_then(server_runtime_http::BoundedText::try_from)
    .map(|_text| ())
    .map_err(std::io::Error::other)
}
#[allow(clippy::single_call_fn)] // write-decision logic is split out to keep file write path minimal and focused
fn should_write_string_into_file(
    path: WrittenFilePathRef<'_>,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<ShouldWriteString> {
    let path_ref = path.as_ref();
    let string_cnt_ref = string_cnt.as_ref();
    match std::fs::metadata(path_ref) {
        Ok(v) => {
            let new_len_u64 = u64::try_from(string_cnt_ref.len()).map_err(|_error| {
                std::io::Error::other(constants_str::VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH)
            })?;
            if v.len() != new_len_u64 {
                return Ok(ShouldWriteString::from(true));
            }
            let mut old_file = std::fs::File::open(path_ref)?;
            let mut offset = constants_usize::ZERO;
            let mut old_chunk = [constants_u8::ZERO; 8192];
            loop {
                let read_len = std::io::Read::read(&mut old_file, &mut old_chunk)?;
                if read_len == constants_usize::ZERO {
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
                        constants_str::VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW,
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
                    return Err(std::io::Error::other(constants_str::F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER));
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
    path: WrittenFilePathRef<'_>,
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
    let should_write = write_string_if_needed(WrittenFilePathRef::from(path_ref), string_cnt)?;
    let path_buf = WrittenFilePathBuf::from(path_ref.to_path_buf());
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
) -> std::io::Result<WrittenFilePathBuf> {
    try_write_string_into_path_with_outcome(path, string_cnt).map(WritePathOutcome::into_path)
}
pub fn try_write_string_into_file<P>(
    file_name: P,
    string_cnt: StringFileContentRef<'_>,
) -> std::io::Result<WrittenFilePathBuf>
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
    fn path_ref(v: &std::path::Path) -> super::WrittenFilePathRef<'_> {
        super::WrittenFilePathRef::from(v)
    }
    fn written_path(v: std::path::PathBuf) -> super::WrittenFilePathBuf {
        super::WrittenFilePathBuf::from(v)
    }
    fn txt_path(name: &str) -> std::path::PathBuf {
        crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(name))
            .as_ref()
            .with_extension(constants_str::TXT)
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
        let path = txt_path(constants_str::MACROS_HELPERS_WRITE_PATH);
        let result_path = super::try_write_string_into_path(&path, cnt(constants_str::ABC_ALT_3))
            .expect("dcb22948 try_write_string_into_path_writes_exact_content invariant must hold");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn write_string_into_file_adds_rs_extension() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_WRITE_FILE,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let _path = super::try_write_string_into_file(&base, cnt(constants_str::XYZ))
            .expect("4f3094e1 write_string_into_file_adds_rs_extension invariant must hold");
        assert_content_and_cleanup(path.as_ref(), constants_str::XYZ);
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_TRY_WRITE_FILE,
        ));
        let path = super::try_write_string_into_file(&base, cnt(constants_str::QWE))
            .expect("6676e082 try_write_string_into_file_returns_path invariant must hold");
        assert_content_and_cleanup(path.as_ref(), constants_str::QWE);
    }
    #[test]
    fn try_write_string_into_path_writes_exact_path_without_extension_rewrite() {
        let path = txt_path(constants_str::MACROS_HELPERS_TRY_WRITE_PATH_PASSTHROUGH);
        let result_path = super::try_write_string_into_path(&path, cnt(constants_str::ABC_ALT_3))
            .expect("b6b47a2c try_write_string_into_path_writes_exact_path_without_extension_rewrite invariant must hold");
        assert_eq!(result_path, written_path(path.clone()));
        assert_content_and_cleanup(path.as_path(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_missing_file() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_MISSING);
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(constants_str::ABC_ALT_3))
                .expect("f5d2cb68 should_write_string_into_file_returns_true_for_missing_file invariant must hold");
        assert!(bool::from(should_write));
    }
    #[test]
    fn should_write_string_into_file_returns_false_when_content_is_eq() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_SAME);
        std::fs::write(&path, constants_str::SAME).expect("68e4f52d should_write_string_into_file_returns_false_when_content_is_eq invariant must hold");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(constants_str::SAME))
                .expect("3e7adf2f should_write_string_into_file_returns_false_when_content_is_eq invariant must hold");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_compares_equal_content_in_chunks() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_LARGE_SAME);
        let content = constants_str::ABCD_ALT.repeat(4097usize);
        std::fs::write(&path, &content).expect("1d706d27 should_write_string_into_file_compares_equal_content_in_chunks invariant must hold");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(&content)).expect("d6619712 should_write_string_into_file_compares_equal_content_in_chunks invariant must hold");
        assert!(!bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_finds_diff_after_first_chunk() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_LARGE_DIFF);
        let old_content = constants_str::A_ALT.repeat(16_388usize);
        let mut new_content = old_content.clone();
        new_content.replace_range(16_387usize.., constants_str::B);
        std::fs::write(&path, old_content).expect("abfd8fbc should_write_string_into_file_finds_diff_after_first_chunk invariant must hold");
        let should_write = super::should_write_string_into_file(path_ref(&path), cnt(&new_content))
            .expect("a3040fa0 should_write_string_into_file_finds_diff_after_first_chunk invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_when_content_differs() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_DIFF);
        std::fs::write(&path, constants_str::OLD).expect("a2fd8473 should_write_string_into_file_returns_true_when_content_differs invariant must hold");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(constants_str::NEW))
                .expect("52c9a1db should_write_string_into_file_returns_true_when_content_differs invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_same_len_diff_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("517fd0c9 should_write_string_into_file_returns_true_for_same_len_diff_content invariant must hold");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(constants_str::XYZ))
                .expect("a82c48b8 should_write_string_into_file_returns_true_for_same_len_diff_content invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn should_write_string_into_file_returns_true_for_diff_len_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_SHOULD_WRITE_DIFF_LEN);
        std::fs::write(&path, constants_str::ABCD_ALT).expect("e2d99b73 should_write_string_into_file_returns_true_for_diff_len_content invariant must hold");
        let should_write =
            super::should_write_string_into_file(path_ref(&path), cnt(constants_str::A_ALT))
                .expect("157e8cad should_write_string_into_file_returns_true_for_diff_len_content invariant must hold");
        assert!(bool::from(should_write));
        cleanup(path.as_path());
    }
    #[test]
    fn write_string_if_needed_returns_false_without_rewrite_for_eq_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_WRITE_IF_NEEDED_EQ);
        std::fs::write(&path, constants_str::SAME).expect("924bdc58 write_string_if_needed_returns_false_without_rewrite_for_eq_content invariant must hold");
        let wrote = super::write_string_if_needed(path_ref(&path), cnt(constants_str::SAME))
            .expect("9f27b9cb write_string_if_needed_returns_false_without_rewrite_for_eq_content invariant must hold");
        assert!(!bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::SAME);
    }
    #[test]
    fn write_string_if_needed_returns_true_and_writes_for_diff_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_WRITE_IF_NEEDED_DIFF);
        std::fs::write(&path, constants_str::OLD).expect("9b4ab8ad write_string_if_needed_returns_true_and_writes_for_diff_content invariant must hold");
        let wrote = super::write_string_if_needed(path_ref(&path), cnt(constants_str::NEW))
            .expect("4e4ce16d write_string_if_needed_returns_true_and_writes_for_diff_content invariant must hold");
        assert!(bool::from(wrote));
        assert_content_and_cleanup(path.as_path(), constants_str::NEW);
    }
    #[test]
    fn path_with_rs_extension_accepts_path_input() {
        let path = crate::rs_file_path::rs_file_path(crate::test_hlp::test_path(
            crate::test_hlp::TestPathStem::new(constants_str::MACROS_HELPERS_RS_EXT_PATH),
        ));
        assert_eq!(
            path.as_ref().extension().and_then(|v| v.to_str()),
            Some("rs")
        );
    }
    #[test]
    fn try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_WRITE_IF_CHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, constants_str::SAME).expect("0242e1a9 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let metadata_before = std::fs::metadata(&path).expect("974bc327 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let _path =
            super::try_write_string_into_file(&base, cnt(constants_str::SAME)).expect("07d9fd90 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        let metadata_after = std::fs::metadata(&path).expect("83087942 try_write_string_into_file_skips_rewrite_when_cnt_is_unchanged invariant must hold");
        assert_eq!(metadata_before.len(), metadata_after.len());
        assert_content_and_cleanup(path.as_ref(), constants_str::SAME);
    }
    #[test]
    fn try_write_string_into_file_writes_when_cnt_differs() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_WRITE_IF_CHANGED_DIFF,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, constants_str::OLD).expect(
            "d870b82e try_write_string_into_file_writes_when_cnt_differs invariant must hold",
        );
        let _path = super::try_write_string_into_file(&base, cnt(constants_str::NEW)).expect(
            "c6fd2bc8 try_write_string_into_file_writes_when_cnt_differs invariant must hold",
        );
        assert_content_and_cleanup(path.as_ref(), constants_str::NEW);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_changed_for_new_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_WRITE_OUTCOME_CHANGED);
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, cnt(constants_str::ABC_ALT_3))
                .expect("947faed1 try_write_string_into_path_with_outcome_returns_changed_for_new_content invariant must hold");
        crate::test_hlp::assert_file_content(
            crate::test_hlp::StdAssertFilePath::new(&path),
            crate::test_hlp::ExpectedFileContent::new(constants_str::ABC_ALT_3),
        );
        assert_outcome_and_cleanup(path.as_path(), &outcome, true);
    }
    #[test]
    fn try_write_string_into_path_with_outcome_returns_unchanged_for_same_content() {
        let path = txt_path(constants_str::MACROS_HELPERS_WRITE_OUTCOME_UNCHANGED);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("d293f783 try_write_string_into_path_with_outcome_returns_unchanged_for_same_content invariant must hold");
        let outcome =
            super::try_write_string_into_path_with_outcome(&path, cnt(constants_str::ABC_ALT_3))
                .expect("b8f8eaf1 try_write_string_into_path_with_outcome_returns_unchanged_for_same_content invariant must hold");
        assert_outcome_and_cleanup(path.as_path(), &outcome, false);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_changed_and_rs_path() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_WRITE_FILE_OUTCOME_CHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, cnt(constants_str::ABC_ALT_3))
                .expect("57cf209a try_write_string_into_file_with_outcome_returns_changed_and_rs_path invariant must hold");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(bool::from(outcome.is_changed()));
        assert_content_and_cleanup(path.as_ref(), constants_str::ABC_ALT_3);
    }
    #[test]
    fn try_write_string_into_file_with_outcome_returns_unchanged_for_same_content() {
        let base = crate::test_hlp::test_path(crate::test_hlp::TestPathStem::new(
            constants_str::MACROS_HELPERS_WRITE_FILE_OUTCOME_UNCHANGED,
        ));
        let path = crate::rs_file_path::rs_file_path(&base);
        std::fs::write(&path, constants_str::ABC_ALT_3).expect("2199f0a7 try_write_string_into_file_with_outcome_returns_unchanged_for_same_content invariant must hold");
        let outcome =
            super::try_write_string_into_file_with_outcome(&base, cnt(constants_str::ABC_ALT_3))
                .expect("f60721a2 try_write_string_into_file_with_outcome_returns_unchanged_for_same_content invariant must hold");
        assert_eq!(outcome.path().as_ref(), path.as_ref());
        assert!(!bool::from(outcome.is_changed()));
        cleanup(path.as_ref());
    }
    #[test]
    fn write_path_outcome_into_path_returns_owned_path() {
        let changed_path = txt_path(constants_str::MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_CHANGED);
        let changed = super::WritePathOutcome::Changed(written_path(changed_path.clone()));
        assert_eq!(changed.into_path(), written_path(changed_path));
        let unchanged_path =
            txt_path(constants_str::MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_UNCHANGED);
        let unchanged = super::WritePathOutcome::Unchanged(written_path(unchanged_path.clone()));
        assert_eq!(unchanged.into_path(), written_path(unchanged_path));
    }
}
