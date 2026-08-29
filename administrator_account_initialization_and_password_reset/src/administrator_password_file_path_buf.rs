#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AdministratorPasswordFilePathBuf(std::path::PathBuf);

impl AdministratorPasswordFilePathBuf {
    pub(crate) fn as_path_ref(&self) -> server_runtime_http::path_ref::PathRef<'_> {
        server_runtime_http::path_ref::PathRef::from(self.0.as_path())
    }
}
