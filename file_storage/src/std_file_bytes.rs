#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub struct StdFileBytes(
    bounded_types::bounded_vec::BoundedVec<u8, 0, { crate::domain_types::MAXIMUM_FILE_BYTES }>,
);
impl TryFrom<Vec<u8>> for StdFileBytes {
    type Error = crate::file_storage_path_error::FileStoragePathError;
    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        match bounded_types::bounded_vec::BoundedVec::try_from(vec) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(crate::file_storage_path_error::FileStoragePathError::FileTooLarge),
        }
    }
}
