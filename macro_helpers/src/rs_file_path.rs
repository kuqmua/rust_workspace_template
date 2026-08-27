#[path = "rs_file_path/rs_file_path.rs"]
mod rs_file_path;
#[path = "rs_file_path/rs_file_path_buf.rs"]
mod rs_file_path_buf;

pub(crate) use rs_file_path::rs_file_path;
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
