#[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefTarget)]
pub(crate) struct StdRsFilePath(pub std::path::PathBuf);
#[allow(clippy::single_call_fn)] // centralized .rs extension mapping keeps path behavior consistent across file-write helpers
pub(crate) fn rs_file_path<P>(file_name: P) -> StdRsFilePath
where
    P: AsRef<std::path::Path>,
{
    StdRsFilePath(file_name.as_ref().with_extension(str_constants::RS))
}
#[cfg(test)]
mod tests {
    #[test]
    fn rs_file_path_adds_rs_extension_for_path_without_extension() {
        let actual = super::rs_file_path(str_constants::SRC_GENERATED);
        assert_eq!(actual.0, std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_replaces_existing_extension() {
        let actual = super::rs_file_path(str_constants::SRC_GENERATED_TXT);
        assert_eq!(actual.0, std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_keeps_parent_directories() {
        let actual = super::rs_file_path(str_constants::TMP_A_B_C);
        assert_eq!(actual.0, std::path::Path::new("tmp/a/b/c.rs"));
    }
}
