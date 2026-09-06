#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
)]
pub enum AdminBadgeVariant {
    #[default]
    Neutral,
    Success,
}

impl AdminBadgeVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Neutral => constants_str::VALUE_5386B853,
            Self::Success => constants_str::VALUE_01AFB233,
        }
    }
}
