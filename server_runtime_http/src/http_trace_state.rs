#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
)]
pub struct HttpTraceState(String);

impl TryFrom<String> for HttpTraceState {
    type Error = crate::http_trace_state_error::HttpTraceStateError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty()
            || string.len() > 512usize
            || !string.bytes().all(|byte| (0x20u8..=0x7eu8).contains(&byte))
        {
            return Err(crate::http_trace_state_error::HttpTraceStateError::Invalid);
        }
        Ok(Self(string))
    }
}
