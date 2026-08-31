#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::ToTokens,
)]
pub struct SynMacroAttrRef<'lt>(&'lt syn::Attribute);

impl<'lt> SynMacroAttrRef<'lt> {
    pub(crate) const fn attr(self) -> &'lt syn::Attribute {
        self.0
    }
}
