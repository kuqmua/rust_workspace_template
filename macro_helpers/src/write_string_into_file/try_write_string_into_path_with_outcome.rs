// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(crate) fn try_write_string_into_path_with_outcome(
    path: impl AsRef<std::path::Path>,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::WritePathOutcome> {
    let path_ref = path.as_ref();
    let should_write =
        super::write_string_if_needed(super::WrittenFilePathRef::from(path_ref), string_cnt)?;
    let path_buf = super::WrittenFilePathBuf::from(path_ref.to_path_buf());
    Ok(if bool::from(should_write) {
        super::WritePathOutcome::Changed(path_buf)
    } else {
        super::WritePathOutcome::Unchanged(path_buf)
    })
}
