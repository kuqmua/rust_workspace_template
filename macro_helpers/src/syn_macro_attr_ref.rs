#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub struct SynMacroAttrRef<'lt>(&'lt syn::Attribute);

impl<'lt> SynMacroAttrRef<'lt> {
    pub(crate) const fn attr(self) -> &'lt syn::Attribute {
        self.0
    }
}
