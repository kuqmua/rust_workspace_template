#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RemoveDirOnDrop {
    path: std::path::PathBuf,
}
#[cfg(feature = "test-utils")]
impl Drop for RemoveDirOnDrop {
    fn drop(&mut self) {
        crate::remove_dir_all_if_exists(self.get_path(), constants_str::E28698F2);
        if let Some(parent) = self.get_path().parent()
            && let Err(error) = std::fs::remove_dir(parent)
            && error.kind() != std::io::ErrorKind::NotFound
            && error.kind() != std::io::ErrorKind::DirectoryNotEmpty
        {
            std::panic::panic_any(constants_str::PANIC_A83F7C18.replacen(
                constants_str::PANIC_PLACEHOLDER_81240055,
                error.to_string().as_str(),
                1usize,
            ));
        }
    }
}
