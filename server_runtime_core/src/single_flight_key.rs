#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq,
)]
pub struct SingleFlightKey(String);
impl TryFrom<String> for SingleFlightKey {
    type Error = crate::single_flight_key_error::SingleFlightKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::single_flight_key_maximum_bytes::SINGLE_FLIGHT_KEY_MAXIMUM_BYTES {
            return Err(crate::single_flight_key_error::SingleFlightKeyError::TooLong);
        }
        if value.is_empty() {
            Err(crate::single_flight_key_error::SingleFlightKeyError::Empty)
        } else if value.contains('\0') {
            Err(crate::single_flight_key_error::SingleFlightKeyError::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}
