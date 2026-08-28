use crate::domain_types::{FileStoragePathError, MAXIMUM_FILE_BYTES};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct StdFileBytes(bounded_types::domain_types::vector::BoundedVec<u8, 0, MAXIMUM_FILE_BYTES>);
impl TryFrom<Vec<u8>> for StdFileBytes {
    type Error = FileStoragePathError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match bounded_types::domain_types::vector::BoundedVec::try_from(value) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(FileStoragePathError::FileTooLarge),
        }
    }
}
