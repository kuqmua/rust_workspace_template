#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ChildProcessSetError {
    #[error("child process set is full")]
    Full,
    #[error("child process identifier overflowed")]
    IdOverflow,
    #[error("child process shutdown failed")]
    Process(#[source] crate::child_process_error::ChildProcessError),
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for ChildProcessSetError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = value;
        Self::Full
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_child_process_capacity_conversion_classifies_each_bound_failure() {
        let zero = bounded_types::bounded_len::BoundedLen::from(0usize);
        let one = bounded_types::bounded_len::BoundedLen::from(1usize);
        assert!(
            [
                bounded_types::bounded_value_error::BoundedValueError::AboveMax {
                    actual: one,
                    max: zero
                },
                bounded_types::bounded_value_error::BoundedValueError::BelowMin {
                    actual: zero,
                    min: one
                },
                bounded_types::bounded_value_error::BoundedValueError::InvalidBounds {
                    min: one,
                    max: zero
                },
            ]
            .into_iter()
            .all(|bounded_value_error| matches!(
                crate::child_process_set_error::ChildProcessSetError::from(bounded_value_error),
                crate::child_process_set_error::ChildProcessSetError::Full
            ))
        );
    }
}
