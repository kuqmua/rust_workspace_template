#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct NotificationExitCode(std::process::ExitCode);
impl std::process::Termination for NotificationExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_notification_exit_adapter_preserves_every_process_status() {
        assert!((u8::MIN..=u8::MAX).all(|value| {
            let notification_exit_code = crate::notification_exit_code::NotificationExitCode::from(
                std::process::ExitCode::from(value),
            );
            std::process::Termination::report(notification_exit_code)
                == std::process::ExitCode::from(value)
        }));
    }
}
