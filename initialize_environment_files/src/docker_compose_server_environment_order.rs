#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum DockerComposeServerEnvironmentOrder {
    Migrate,
    Serve,
}
