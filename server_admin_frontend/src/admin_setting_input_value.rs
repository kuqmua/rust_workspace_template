#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct AdminSettingInputValue(Box<str>);
