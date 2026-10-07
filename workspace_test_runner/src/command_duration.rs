#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct CommandDuration(std::time::Duration);
impl CommandDuration {
    pub(super) fn as_millis(self) -> crate::command_duration_millis::CommandDurationMillis {
        crate::command_duration_millis::CommandDurationMillis::from(self.0.as_millis())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_command_duration_milliseconds_truncate_submillisecond_values_and_preserve_range() {
        [
            (std::time::Duration::ZERO, 0u128),
            (std::time::Duration::from_nanos(999_999u64), 0u128),
            (std::time::Duration::from_nanos(1_000_000u64), 1u128),
            (std::time::Duration::from_nanos(1_999_999u64), 1u128),
            (std::time::Duration::from_nanos(2_000_000u64), 2u128),
            (std::time::Duration::new(1u64, 999_999_999u32), 1999u128),
            (std::time::Duration::MAX, 18_446_744_073_709_551_615_999u128),
        ]
        .into_iter()
        .fold((), |(), (duration, expected_milliseconds)| {
            let command_duration = crate::command_duration::CommandDuration::from(duration);
            assert_eq!(
                command_duration.as_millis().to_string(),
                expected_milliseconds.to_string()
            );
        });
    }
}
