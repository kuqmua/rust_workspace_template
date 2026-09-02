#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
pub struct SafeFileStorage {
    root: crate::file_storage_root_path_buf::FileStorageRootPathBuf,
}
#[allow(
    clippy::multiple_inherent_impl,
    reason = "domain constructor and path access stay separate from filesystem adapter operations"
)]
impl SafeFileStorage {
    pub(crate) fn root(&self) -> crate::storage_path_ref::StoragePathRef<'_> {
        crate::storage_path_ref::StoragePathRef::from(self.root.as_ref())
    }
}
