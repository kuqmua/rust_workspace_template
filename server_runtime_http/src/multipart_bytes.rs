#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct MultipartBytes(bounded_types::BoundedVec<u8, 0, 16_777_216>);

impl TryFrom<Vec<u8>> for MultipartBytes {
    type Error = super::MultipartValueError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let actual = super::MultipartValueLength::from(value.len());
        match bounded_types::BoundedVec::try_from(value) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(Self::Error::TooLong { actual }),
        }
    }
}
