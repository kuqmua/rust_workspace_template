#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn apply_executes_the_prebuilt_plan() {
        let report = crate::execute_plan::execute_plan(
            crate::execution_mode::ExecutionMode::Apply,
            4u8,
            async |plan| Ok::<u8, std::convert::Infallible>(plan.saturating_add(1u8)),
        )
        .await;
        assert_eq!(
            report,
            Ok(crate::execution_report::ExecutionReport::Applied { output: 5u8 })
        );
    }

    #[tokio::test]
    async fn dry_run_returns_plan_without_calling_mutation() {
        let called = std::sync::atomic::AtomicBool::new(false);
        let report = crate::execute_plan::execute_plan(
            crate::execution_mode::ExecutionMode::DryRun,
            4u8,
            async |_plan| {
                called.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok::<u8, std::convert::Infallible>(5u8)
            },
        )
        .await;
        assert_eq!(
            report,
            Ok(crate::execution_report::ExecutionReport::DryRun { plan: 4u8 })
        );
        assert!(!called.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn apply_propagates_mutation_error() {
        let error = constants_str::VALUE_E5B04B63;
        let report = crate::execute_plan::execute_plan(
            crate::execution_mode::ExecutionMode::Apply,
            4u8,
            async |_plan| Err::<u8, &str>(error),
        )
        .await;
        assert_eq!(report, Err(error));
    }
}
