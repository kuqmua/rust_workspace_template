#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(super) struct CompileErrorMessage<'message_lt>(&'message_lt str);

impl<'message_lt> From<&'message_lt String> for CompileErrorMessage<'message_lt> {
    fn from(value: &'message_lt String) -> Self {
        Self(value.as_str())
    }
}
