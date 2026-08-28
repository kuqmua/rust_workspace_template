#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminSettingInputKind {
    Text,
    TextArea,
    Url,
}
