#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}

impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> super::super::TransportStatus {
        match self {
            Self::Code200 => super::super::TransportStatus::from(super::super::KnownHttpStatus::Ok),
            Self::Code201 => {
                super::super::TransportStatus::from(super::super::KnownHttpStatus::Created)
            }
            Self::Code204 => {
                super::super::TransportStatus::from(super::super::KnownHttpStatus::NoContent)
            }
        }
    }
}
