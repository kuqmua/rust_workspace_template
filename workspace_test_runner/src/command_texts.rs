#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct CommandTexts(
    bounded_types::bounded_vec::BoundedVec<crate::command_text::CommandText, 0, { usize::MAX }>,
);

impl CommandTexts {
    pub(super) fn as_ref(&self) -> &[crate::command_text::CommandText] {
        self.0.as_ref()
    }
}
