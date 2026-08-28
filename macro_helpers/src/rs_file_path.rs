#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::single_call_fn)]
pub(crate) fn rs_file_path<P>(file_name: P) -> RsFilePathBuf
where
    P: AsRef<std::path::Path>,
{
    RsFilePathBuf::from(file_name.as_ref().with_extension(constants_str::RS))
}
#[path = "rs_file_path_buf.rs"]
mod rs_file_path_buf;
pub(crate) use rs_file_path_buf::RsFilePathBuf;
#[cfg(test)]
mod tests {
    #[test]
    fn rs_file_path_adds_rs_extension_for_path_without_extension() {
        let actual = super::rs_file_path(constants_str::SRC_GENERATED);
        assert_eq!(actual.0, std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_replaces_existing_extension() {
        let actual = super::rs_file_path(constants_str::SRC_GENERATED_TXT);
        assert_eq!(actual.0, std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_keeps_parent_directories() {
        let actual = super::rs_file_path(constants_str::TMP_A_B_C);
        assert_eq!(actual.0, std::path::Path::new("tmp/a/b/c.rs"));
    }
}
