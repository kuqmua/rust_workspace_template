#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_new::New)]
pub struct ServiceRuntime {
    #[constructor(order = 1)]
    optional_task: Option<crate::background_task::BackgroundTask>,
    #[constructor(order = 0)]
    router: crate::axum_router::AxumRouter,
}

impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::axum_router::AxumRouter,
        Option<crate::background_task::BackgroundTask>,
    ) {
        (self.router, self.optional_task)
    }
}
