#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_foundation::FromInner,
)]
pub struct UniqueOptionBTreeSet<OptionValue>(std::collections::BTreeSet<OptionValue>);
impl<OptionValue> Default for UniqueOptionBTreeSet<OptionValue> {
    fn default() -> Self {
        Self::from(std::collections::BTreeSet::new())
    }
}
impl<OptionValue> UniqueOptionBTreeSet<OptionValue>
where
    OptionValue: Copy + Ord,
{
    #[must_use]
    pub fn contains(
        &self,
        option_value: OptionValue,
    ) -> crate::std_unique_option_set_contains::StdUniqueOptionSetContains {
        crate::std_unique_option_set_contains::StdUniqueOptionSetContains::from(
            self.0.contains(&option_value),
        )
    }
    #[must_use]
    pub fn is_empty(&self) -> crate::std_unique_option_set_is_empty::StdUniqueOptionSetIsEmpty {
        crate::std_unique_option_set_is_empty::StdUniqueOptionSetIsEmpty::from(self.0.is_empty())
    }
    pub fn try_insert_with<DuplicateError>(
        &mut self,
        option_value: OptionValue,
        duplicate_error: DuplicateError,
    ) -> syn::Result<()>
    where
        DuplicateError: FnOnce() -> syn::Error,
    {
        if !self.0.insert(option_value) {
            return Err(duplicate_error());
        }
        Ok(())
    }
}
