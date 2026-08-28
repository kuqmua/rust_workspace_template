#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}

impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> crate::TransportStatus {
        match self {
            Self::Code200 => crate::TransportStatus::from(crate::KnownHttpStatus::Ok),
            Self::Code201 => crate::TransportStatus::from(crate::KnownHttpStatus::Created),
            Self::Code204 => crate::TransportStatus::from(crate::KnownHttpStatus::NoContent),
        }
    }
}
