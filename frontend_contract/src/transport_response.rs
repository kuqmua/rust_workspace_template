#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
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
        transport_body: crate::transport_body::TransportBody,
        transport_status: crate::transport_status::TransportStatus,
    ) -> Self {
        Self {
            body: transport_body,
            retry_after: None,
            status: transport_status,
        }
    }
    #[must_use]
    pub fn with_retry_after(
        mut self,
        option: Option<crate::transport_retry_after::TransportRetryAfter>,
    ) -> Self {
        self.retry_after = option;
        self
    }

    #[must_use]
    pub const fn retry_after(&self) -> Option<&crate::transport_retry_after::TransportRetryAfter> {
        self.retry_after.as_ref()
    }
    pub fn success_body(
        &self,
        transport_status: crate::transport_status::TransportStatus,
    ) -> Result<&crate::transport_body::TransportBody, crate::client_error::ClientError> {
        if self.status == transport_status {
            Ok(&self.body)
        } else {
            Err(
                crate::decode_api_problem::decode_api_problem(&self.body).map_or(
                    crate::client_error::ClientError::Status {
                        actual: self.status,
                        expected: transport_status,
                    },
                    crate::client_error::ClientError::Problem,
                ),
            )
        }
    }
}
