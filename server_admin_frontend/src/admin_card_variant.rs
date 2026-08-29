#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "component props and wire enum variants retain their semantic presentation order"
)]
#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum AdminCardVariant {
    #[default]
    Default,
    #[cfg(not(target_arch = "wasm32"))]
    Auth,
    #[cfg(not(target_arch = "wasm32"))]
    Code,
    Profile,
    Security,
    Settings,
}

impl AdminCardVariant {
    pub(super) const fn class(self) -> &'static str {
        match self {
            Self::Default => constants_str::test_fixtures::VALUE_417CCDBE,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Auth => constants_str::test_fixtures::VALUE_A8036BFC,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Code => constants_str::test_fixtures::VALUE_1FDF161B,
            Self::Profile => constants_str::test_fixtures::VALUE_51A2D8C6,
            Self::Security => constants_str::test_fixtures::VALUE_140F31FA,
            Self::Settings => constants_str::test_fixtures::VALUE_48A99713,
        }
    }
}
