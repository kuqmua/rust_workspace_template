#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum MacroAttrError {
    #[error("attr_not_list")]
    AttrNotList,
    #[error("no_attr")]
    NoAttr,
}
