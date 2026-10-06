#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
        operation_error: OperationError,
        result: Result<(), RollbackError>,
    ) -> Self {
        match result {
            Ok(()) => Self::Operation {
                source: operation_error,
            },
            Err(rollback_error) => Self::OperationAndRollback {
                operation: operation_error,
                rollback: rollback_error,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transaction_failure_variants_preserve_domain_errors_and_source_contract() {
        assert!(
            [Ok(()), Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotFinite)]
                .into_iter()
                .all(|result| {
                    let failure = crate::transaction_failure::TransactionFailure::from_operation_and_rollback(
                        crate::list_total_error::ListTotalError::Negative,
                        result,
                    );
                    let operation_message = crate::list_total_error::ListTotalError::Negative.to_string();
                    let message_matches = failure.to_string().contains(operation_message.as_str());
                    message_matches && match result {
                        Ok(()) => {
                            matches!(&failure,
                                crate::transaction_failure::TransactionFailure::Operation { source }
                                    if *source == crate::list_total_error::ListTotalError::Negative
                            ) && std::error::Error::source(&failure)
                                .and_then(|source| source.downcast_ref::<crate::list_total_error::ListTotalError>())
                                == Some(&crate::list_total_error::ListTotalError::Negative)
                        }
                        Err(expected_rollback) => {
                            matches!(&failure,
                                crate::transaction_failure::TransactionFailure::OperationAndRollback { operation, rollback }
                                    if *operation == crate::list_total_error::ListTotalError::Negative
                                        && *rollback == expected_rollback
                            ) && std::error::Error::source(&failure).is_none()
                                && failure.to_string().contains(expected_rollback.to_string().as_str())
                        }
                    }
                })
        );
    }

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
