#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefStr,
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
    pub(super) fn push_str(&mut self, text_ref: crate::text_ref::TextRef<'_>) -> Result<(), ()> {
        if self
            .0
            .as_str()
            .len()
            .checked_add(text_ref.as_ref().len())
            .is_none_or(|len| len > constants_usize::VALUE_1_048_576)
        {
            return Err(());
        }
        let mut candidate = self.0.as_str().to_owned();
        candidate.push_str(text_ref.as_ref());
        self.0 = bounded_types::bounded_string::BoundedString::try_from(candidate)
            .map_err(|_error| ())?;
        Ok(())
    }
}
