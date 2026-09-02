#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::FromInner,
)]
pub struct AdminSettingLabel(&'static str);
