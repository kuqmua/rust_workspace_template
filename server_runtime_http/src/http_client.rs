#[cfg(test)]
mod tests {
    #[test]
    fn test_timeout_wrappers_reject_zero() {
        assert_eq!(
            crate::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration::try_from(
                std::time::Duration::ZERO
            )
            .err(),
            Some(crate::std_reqwest_timeout_error::StdReqwestTimeoutError::Zero)
        );
        assert_eq!(
            crate::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration::try_from(
                std::time::Duration::ZERO
            )
            .err(),
            Some(crate::std_reqwest_timeout_error::StdReqwestTimeoutError::Zero)
        );
    }
}

// Root-owned module compatibility wrappers.
mod reqwest_client {}
mod reqwest_client_build_error {}
mod reqwest_client_policy {}
mod reqwest_connect_timeout_duration {}
mod reqwest_request_timeout_duration {}
mod std_reqwest_timeout_duration_ref {}
mod std_reqwest_timeout_error {}
mod tracing_http_client_span {}
