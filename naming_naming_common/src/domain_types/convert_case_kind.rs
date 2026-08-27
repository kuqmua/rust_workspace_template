#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct ConvertCaseKind(pub(super) convert_case::Case<'static>);
