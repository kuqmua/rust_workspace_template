#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString, newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub(super) struct SummaryText(String);
impl SummaryText {
    pub(super) fn push_str(&mut self, value: crate::text_ref::TextRef<'_>) -> Result<(), ()> {
        if self
            .0
            .len()
            .checked_add(value.as_ref().len())
            .is_none_or(|len| len > constants_usize::VALUE_1_048_576)
        {
            return Err(());
        }
        self.0.push_str(value.as_ref());
        Ok(())
    }
}
