pub async fn execute_plan<Plan, Output, Error, Apply, ApplyFuture>(
    execution_mode: crate::execution_mode::ExecutionMode,
    plan: Plan,
    apply: Apply,
) -> Result<crate::execution_report::ExecutionReport<Plan, Output>, Error>
where
    Apply: FnOnce(Plan) -> ApplyFuture,
    ApplyFuture: Future<Output = Result<Output, Error>>,
{
    match execution_mode {
        crate::execution_mode::ExecutionMode::Apply => {
            let output = apply(plan).await?;
            Ok(crate::execution_report::ExecutionReport::Applied { output })
        }
        crate::execution_mode::ExecutionMode::DryRun => {
            Ok(crate::execution_report::ExecutionReport::DryRun { plan })
        }
    }
}
