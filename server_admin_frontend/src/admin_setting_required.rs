#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(in crate::domain_types::shared::settings) struct AdminSettingRequired(bool);
