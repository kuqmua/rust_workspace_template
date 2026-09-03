#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AdministratorPasswordFilePathBuf(std::path::PathBuf);

impl AdministratorPasswordFilePathBuf {
    pub(crate) fn as_path_ref(&self) -> server_runtime_http::runtime_path_ref::RuntimePathRef<'_> {
        server_runtime_http::runtime_path_ref::RuntimePathRef::from(self.0.as_path())
    }
}
