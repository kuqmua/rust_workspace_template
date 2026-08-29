#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}

impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> crate::transport_status::TransportStatus {
        match self {
            Self::Code200 => crate::transport_status::TransportStatus::from(
                crate::known_http_status::KnownHttpStatus::Ok,
            ),
            Self::Code201 => crate::transport_status::TransportStatus::from(
                crate::known_http_status::KnownHttpStatus::Created,
            ),
            Self::Code204 => crate::transport_status::TransportStatus::from(
                crate::known_http_status::KnownHttpStatus::NoContent,
            ),
        }
    }
}
