#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct CompileErrorMessage<'message_lt>(&'message_lt str);

impl<'message_lt> From<&'message_lt String> for CompileErrorMessage<'message_lt> {
    fn from(value: &'message_lt String) -> Self {
        Self(value.as_str())
    }
}
