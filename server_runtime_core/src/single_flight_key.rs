use super::{SINGLE_FLIGHT_KEY_MAXIMUM_BYTES, SingleFlightKeyError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SingleFlightKey(String);
impl TryFrom<String> for SingleFlightKey {
    type Error = SingleFlightKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SINGLE_FLIGHT_KEY_MAXIMUM_BYTES {
            return Err(SingleFlightKeyError::TooLong);
        }
        if value.is_empty() {
            Err(SingleFlightKeyError::Empty)
        } else if value.contains('\0') {
            Err(SingleFlightKeyError::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}
