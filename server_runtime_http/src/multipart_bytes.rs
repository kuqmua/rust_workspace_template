#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefTarget,
)]
pub struct MultipartBytes(bounded_types::bounded_vec::BoundedVec<u8, 0, 16_777_216>);

impl TryFrom<Vec<u8>> for MultipartBytes {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let actual = crate::multipart_value_length::MultipartValueLength::from(value.len());
        match bounded_types::bounded_vec::BoundedVec::try_from(value) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(Self::Error::TooLong { actual }),
        }
    }
}
