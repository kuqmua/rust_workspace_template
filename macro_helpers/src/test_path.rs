static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub(crate) fn test_path(
    test_path_stem: crate::test_path_stem::TestPathStem<'_>,
) -> crate::rs_file_path_buf::RsFilePathBuf {
    let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    crate::rs_file_path_buf::RsFilePathBuf::from(std::env::temp_dir().join(format!(
        "{}_{}_{}",
        test_path_stem.as_ref(),
        std::process::id(),
        seq
    )))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_temporary_paths_preserve_stem_directory_and_distinct_identity() {
        let stem = stringify!(test_temporary_paths_preserve_stem_directory_and_distinct_identity);
        let first = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(stem));
        let second = crate::test_path::test_path(crate::test_path_stem::TestPathStem::new(stem));
        let directory = std::env::temp_dir();
        let prefix = format!("{stem}_{}_", std::process::id());
        assert_ne!(first.as_ref(), second.as_ref());
        assert!([first, second].into_iter().all(|path| {
            path.as_ref().parent() == Some(directory.as_path())
                && path
                    .as_ref()
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .and_then(|name| name.strip_prefix(prefix.as_str()))
                    .is_some_and(|sequence| sequence.parse::<usize>().is_ok())
        }));
    }
}
