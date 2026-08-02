#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionMode {
    Apply,
    DryRun,
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub enum ExecutionReport<Plan, Output> {
    Applied { output: Output },
    DryRun { plan: Plan },
}

pub async fn execute_plan<Plan, Output, Error, Apply, ApplyFuture>(
    mode: ExecutionMode,
    plan: Plan,
    apply: Apply,
) -> Result<ExecutionReport<Plan, Output>, Error>
where
    Apply: FnOnce(Plan) -> ApplyFuture,
    ApplyFuture: Future<Output = Result<Output, Error>>,
{
    match mode {
        ExecutionMode::Apply => {
            let output = apply(plan).await?;
            Ok(ExecutionReport::Applied { output })
        }
        ExecutionMode::DryRun => Ok(ExecutionReport::DryRun { plan }),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn apply_executes_the_prebuilt_plan() {
        let report = super::execute_plan(super::ExecutionMode::Apply, 4u8, async |plan| {
            Ok::<u8, std::convert::Infallible>(plan.saturating_add(1u8))
        })
        .await;
        assert_eq!(report, Ok(super::ExecutionReport::Applied { output: 5u8 }));
    }

    #[tokio::test]
    async fn dry_run_returns_plan_without_calling_mutation() {
        let called = std::sync::atomic::AtomicBool::new(false);
        let report = super::execute_plan(super::ExecutionMode::DryRun, 4u8, async |_plan| {
            called.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok::<u8, std::convert::Infallible>(5u8)
        })
        .await;
        assert_eq!(report, Ok(super::ExecutionReport::DryRun { plan: 4u8 }));
        assert!(!called.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[tokio::test]
    async fn apply_propagates_mutation_error() {
        let error = "apply failed";
        let report = super::execute_plan(super::ExecutionMode::Apply, 4u8, async |_plan| {
            Err::<u8, &str>(error)
        })
        .await;
        assert_eq!(report, Err(error));
    }
}
