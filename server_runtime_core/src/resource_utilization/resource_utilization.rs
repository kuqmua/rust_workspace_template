#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct ResourceUtilization {
    pub(super) maximum: super::ResourceAmount,
    pub(super) used: super::ResourceAmount,
    pub(super) percent: super::ResourceUtilizationPercent,
    pub(super) status: super::ResourceUtilizationStatus,
}

impl ResourceUtilization {
    #[must_use]
    pub const fn maximum(self) -> super::ResourceAmount {
        self.maximum
    }

    #[must_use]
    pub const fn percent(self) -> super::ResourceUtilizationPercent {
        self.percent
    }

    #[must_use]
    pub const fn status(self) -> super::ResourceUtilizationStatus {
        self.status
    }

    #[must_use]
    pub const fn used(self) -> super::ResourceAmount {
        self.used
    }
}
