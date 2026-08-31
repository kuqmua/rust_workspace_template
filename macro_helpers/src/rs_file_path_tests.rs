// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(crate) fn rs_file_path<P>(file_name: P) -> crate::rs_file_path_buf::RsFilePathBuf
where
    P: AsRef<std::path::Path>,
{
    crate::rs_file_path_buf::RsFilePathBuf::from(
        file_name
            .as_ref()
            .with_extension(constants_str::catalog::RS),
    )
}
#[cfg(test)]
mod tests {
    #[test]
    fn rs_file_path_adds_rs_extension_for_path_without_extension() {
        let actual = crate::rs_file_path_tests::rs_file_path(constants_str::catalog::SRC_GENERATED);
        assert_eq!(actual.as_ref(), std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_replaces_existing_extension() {
        let actual =
            crate::rs_file_path_tests::rs_file_path(constants_str::catalog::SRC_GENERATED_TXT);
        assert_eq!(actual.as_ref(), std::path::Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_keeps_parent_directories() {
        let actual = crate::rs_file_path_tests::rs_file_path(constants_str::catalog::TMP_A_B_C);
        assert_eq!(actual.as_ref(), std::path::Path::new("tmp/a/b/c.rs"));
    }
}
