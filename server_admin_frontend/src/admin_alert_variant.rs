#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum AdminAlertVariant {
    #[default]
    Error,
    #[cfg(not(target_arch = "wasm32"))]
    Success,
}

impl AdminAlertVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Error => constants_str::test_fixtures::VALUE_6EFBABDA,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Success => constants_str::test_fixtures::VALUE_A443C355,
        }
    }

    pub(super) const fn role(self) -> &'static str {
        match self {
            Self::Error => constants_str::test_fixtures::HTML_ALERT_ROLE,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Success => constants_str::test_fixtures::HTML_STATUS_ROLE,
        }
    }
}
