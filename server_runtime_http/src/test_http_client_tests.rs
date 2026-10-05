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

    #[test]
    fn test_reqwest_timeout_wrappers_preserve_positive_duration_boundaries() {
        assert!([
            std::time::Duration::from_nanos(1u64),
            std::time::Duration::new(1u64, 123_456_789u32),
            std::time::Duration::new(u64::MAX, 0u32),
            std::time::Duration::MAX,
        ].into_iter().all(|duration| {
            crate::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration::try_from(duration)
                .is_ok_and(|timeout| *timeout == duration)
                && crate::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration::try_from(duration)
                    .is_ok_and(|timeout| *timeout == duration)
        }));
    }
}
