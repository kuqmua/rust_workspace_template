#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
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
    pub(super) fn push_str(&mut self, value: crate::text_ref::TextRef<'_>) -> Result<(), ()> {
        if self
            .0
            .as_str()
            .len()
            .checked_add(value.as_ref().len())
            .is_none_or(|len| len > constants_usize::VALUE_1_048_576)
        {
            return Err(());
        }
        let mut candidate = self.0.as_str().to_owned();
        candidate.push_str(value.as_ref());
        self.0 = bounded_types::bounded_string::BoundedString::try_from(candidate)
            .map_err(|_error| ())?;
        Ok(())
    }
}
