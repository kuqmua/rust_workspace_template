#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum ConstantPart {
    Fragment(super::SynIdent),
    Literal(super::SynLitStr),
}
