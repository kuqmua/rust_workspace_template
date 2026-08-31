#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TransactionFailure<OperationError, RollbackError>
where
    OperationError: std::error::Error + 'static,
    RollbackError: std::error::Error + 'static,
{
    #[error("transaction operation failed: {source}")]
    Operation {
        #[source]
        source: OperationError,
    },
    #[error("transaction operation failed: {operation}; rollback also failed: {rollback}")]
    OperationAndRollback {
        operation: OperationError,
        rollback: RollbackError,
    },
}

impl<OperationError, RollbackError> TransactionFailure<OperationError, RollbackError>
where
    OperationError: std::error::Error + 'static,
    RollbackError: std::error::Error + 'static,
{
    #[must_use]
    pub fn from_operation_and_rollback(
        operation: OperationError,
        rollback: Result<(), RollbackError>,
    ) -> Self {
        match rollback {
            Ok(()) => Self::Operation { source: operation },
            Err(rollback_error) => Self::OperationAndRollback {
                operation,
                rollback: rollback_error,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display_preserves_operation_and_rollback_errors() {
        let failure = crate::transaction_failure::TransactionFailure::from_operation_and_rollback(
            std::io::Error::other(constants_str::TEST_TRANSACTION_OPERATION_ERROR),
            Err(std::io::Error::other(
                constants_str::TEST_TRANSACTION_ROLLBACK_ERROR,
            )),
        );
        let message = failure.to_string();
        assert!(message.contains(constants_str::TEST_TRANSACTION_OPERATION_ERROR));
        assert!(message.contains(constants_str::TEST_TRANSACTION_ROLLBACK_ERROR));
    }
}
