// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(crate) fn rs_file_path<P>(p: P) -> crate::rs_file_path_buf::RsFilePathBuf
where
    P: AsRef<std::path::Path>,
{
    crate::rs_file_path_buf::RsFilePathBuf::from(p.as_ref().with_extension(constants_str::RS))
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_rs_file_path_adds_rs_extension_for_path_without_extension() {
        let actual = crate::rs_file_path_tests::rs_file_path(constants_str::SRC_GENERATED);
        assert_eq!(
            actual.as_ref(),
            std::path::Path::new(constants_str::VALUE_126F5D8E)
        );
    }
    #[test]
    fn test_rs_file_path_replaces_existing_extension() {
        let actual = crate::rs_file_path_tests::rs_file_path(constants_str::SRC_GENERATED_TXT);
        assert_eq!(
            actual.as_ref(),
            std::path::Path::new(constants_str::VALUE_126F5D8E)
        );
    }
    #[test]
    fn test_rs_file_path_keeps_parent_directories() {
        let actual = crate::rs_file_path_tests::rs_file_path(constants_str::TMP_A_B_C);
        assert_eq!(
            actual.as_ref(),
            std::path::Path::new(constants_str::VALUE_95045552)
        );
    }
}
