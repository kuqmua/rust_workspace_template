#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub struct MultipartBytes(bounded_types::bounded_vec::BoundedVec<u8, 0, 16_777_216>);

impl TryFrom<Vec<u8>> for MultipartBytes {
    type Error = crate::multipart_value_error::MultipartValueError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        let actual = crate::multipart_value_length::MultipartValueLength::from(vec.len());
        match bounded_types::bounded_vec::BoundedVec::try_from(vec) {
            Ok(bounded) => Ok(Self(bounded)),
            Err(_error) => Err(Self::Error::TooLong { actual }),
        }
    }
}
