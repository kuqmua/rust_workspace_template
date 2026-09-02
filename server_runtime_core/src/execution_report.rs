#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum ExecutionReport<Plan, Output> {
    Applied { output: Output },
    DryRun { plan: Plan },
}
