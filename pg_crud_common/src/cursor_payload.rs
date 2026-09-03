#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct CursorPayload(String);

impl CursorPayload {
    const MAXIMUM_LENGTH: usize = 65_536usize;
}

impl TryFrom<String> for CursorPayload {
    type Error = crate::cursor_payload_error::CursorPayloadError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > Self::MAXIMUM_LENGTH {
            Err(crate::cursor_payload_error::CursorPayloadError::Empty)
        } else {
            Ok(Self(string))
        }
    }
}
