#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct HttpTraceState(String);

impl TryFrom<String> for HttpTraceState {
    type Error = crate::http_trace_state_error::HttpTraceStateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 512usize
            || !value.bytes().all(|byte| (0x20u8..=0x7eu8).contains(&byte))
        {
            return Err(crate::http_trace_state_error::HttpTraceStateError::Invalid);
        }
        Ok(Self(value))
    }
}
