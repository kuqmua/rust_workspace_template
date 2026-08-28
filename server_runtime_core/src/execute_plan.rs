pub async fn execute_plan<Plan, Output, Error, Apply, ApplyFuture>(
    mode: super::ExecutionMode,
    plan: Plan,
    apply: Apply,
) -> Result<super::ExecutionReport<Plan, Output>, Error>
where
    Apply: FnOnce(Plan) -> ApplyFuture,
    ApplyFuture: Future<Output = Result<Output, Error>>,
{
    match mode {
        super::ExecutionMode::Apply => {
            let output = apply(plan).await?;
            Ok(super::ExecutionReport::Applied { output })
        }
        super::ExecutionMode::DryRun => Ok(super::ExecutionReport::DryRun { plan }),
    }
}
