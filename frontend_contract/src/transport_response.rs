#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportResponse {
    body: crate::transport_body::TransportBody,
    #[getters(skip)]
    retry_after: Option<crate::transport_retry_after::TransportRetryAfter>,
    #[getters(copy)]
    status: crate::transport_status::TransportStatus,
}

impl TransportResponse {
    #[must_use]
    pub const fn new(
        body: crate::transport_body::TransportBody,
        status: crate::transport_status::TransportStatus,
    ) -> Self {
        Self {
            body,
            retry_after: None,
            status,
        }
    }
    #[must_use]
    pub fn with_retry_after(
        mut self,
        retry_after: Option<crate::transport_retry_after::TransportRetryAfter>,
    ) -> Self {
        self.retry_after = retry_after;
        self
    }

    #[must_use]
    pub const fn retry_after(&self) -> Option<&crate::transport_retry_after::TransportRetryAfter> {
        self.retry_after.as_ref()
    }
    pub fn success_body(
        &self,
        expected: crate::transport_status::TransportStatus,
    ) -> Result<&crate::transport_body::TransportBody, crate::client_error::ClientError> {
        if self.status == expected {
            Ok(&self.body)
        } else {
            Err(
                crate::decode_api_problem::decode_api_problem(&self.body).map_or(
                    crate::client_error::ClientError::Status {
                        actual: self.status,
                        expected,
                    },
                    crate::client_error::ClientError::Problem,
                ),
            )
        }
    }
}
