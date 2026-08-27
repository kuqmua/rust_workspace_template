#[allow(clippy::single_call_fn)]
pub(crate) fn rs_file_path<P>(file_name: P) -> super::RsFilePathBuf
where
    P: AsRef<std::path::Path>,
{
    super::RsFilePathBuf::from(file_name.as_ref().with_extension(constants_str::RS))
}
