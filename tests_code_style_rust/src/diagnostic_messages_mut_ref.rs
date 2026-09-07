#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_mut_target::DerefMutTarget,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct DiagnosticMessagesMutRef<'msgs_lt>(&'msgs_lt mut Vec<String>);
impl<'msgs_lt> From<&'msgs_lt mut crate::source_text_list::SourceTextList>
    for DiagnosticMessagesMutRef<'msgs_lt>
{
    fn from(value: &'msgs_lt mut crate::source_text_list::SourceTextList) -> Self {
        Self::from(&mut **value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_diagnostic_adapter_updates_its_source_list() {
        let mut source_text_list = crate::source_text_list::SourceTextList::default();
        super::DiagnosticMessagesMutRef::from(&mut source_text_list)
            .push(String::from(constants_str::ERROR));
        assert_eq!(source_text_list.as_slice(), [constants_str::ERROR]);
    }
}
