#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportResponse {
    body: super::TransportBody,
    retry_after: Option<super::TransportRetryAfter>,
    status: super::TransportStatus,
}

impl TransportResponse {
    #[must_use]
    pub const fn new(body: super::TransportBody, status: super::TransportStatus) -> Self {
        Self {
            body,
            retry_after: None,
            status,
        }
    }
    #[must_use]
    pub fn with_retry_after(mut self, retry_after: Option<super::TransportRetryAfter>) -> Self {
        self.retry_after = retry_after;
        self
    }
    #[must_use]
    pub const fn body(&self) -> &super::TransportBody {
        &self.body
    }
    #[must_use]
    pub const fn status(&self) -> super::TransportStatus {
        self.status
    }
    #[must_use]
    pub const fn retry_after(&self) -> Option<&super::TransportRetryAfter> {
        self.retry_after.as_ref()
    }
    pub fn success_body(
        &self,
        expected: super::TransportStatus,
    ) -> Result<&super::TransportBody, super::ClientError> {
        if self.status == expected {
            Ok(&self.body)
        } else {
            Err(super::decode_api_problem(&self.body).map_or(
                super::ClientError::Status {
                    actual: self.status,
                    expected,
                },
                super::ClientError::Problem,
            ))
        }
    }
}
