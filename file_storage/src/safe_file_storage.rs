use super::{FileStorageRootPathBuf, StoragePathRef};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_constructor::New)]
pub struct SafeFileStorage {
    root: FileStorageRootPathBuf,
}
#[allow(
    clippy::multiple_inherent_impl,
    reason = "domain constructor and path access stay separate from filesystem adapter operations"
)]
impl SafeFileStorage {
    pub(crate) fn root(&self) -> StoragePathRef<'_> {
        StoragePathRef::from(self.root.as_ref())
    }
}
