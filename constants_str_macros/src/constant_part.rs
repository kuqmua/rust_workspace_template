#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum ConstantPart {
    Fragment(crate::syn_ident::SynIdent),
    Literal(crate::syn_lit_str::SynLitStr),
}
