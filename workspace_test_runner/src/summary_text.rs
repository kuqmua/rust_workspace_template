#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(super) struct SummaryText(
    bounded_types::bounded_string::BoundedString<
        0usize,
        { constants_usize::VALUE_1_048_576 },
        false,
    >,
);
impl SummaryText {
    pub(super) fn push_str(
        &mut self,
        text_ref: crate::text_ref::TextRef<'_>,
    ) -> Result<(), crate::summary_text_append_error::SummaryTextAppendError> {
        if self
            .0
            .as_str()
            .len()
            .checked_add(text_ref.as_ref().len())
            .is_none_or(|len| len > constants_usize::VALUE_1_048_576)
        {
            return Err(crate::summary_text_append_error::SummaryTextAppendError::CapacityExceeded);
        }
        let mut candidate = self.0.as_str().to_owned();
        candidate.push_str(text_ref.as_ref());
        self.0 = bounded_types::bounded_string::BoundedString::from_truncated(candidate);
        Ok(())
    }
}
