#[cfg(test)]
mod tests {
    #[test]
    fn test_cleanup_batch_rejects_zero() {
        assert_eq!(
            crate::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
            Err(crate::admin_cleanup_configuration_error::AdminCleanupConfigurationError::BatchSizeOutOfRange),
        );
    }
}
