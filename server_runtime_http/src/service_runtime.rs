#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ServiceRuntime {
    optional_task: Option<crate::background_task::BackgroundTask>,
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

    #[must_use]
    pub const fn new(
        router: crate::axum_router::AxumRouter,
        optional_task: Option<crate::background_task::BackgroundTask>,
    ) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}
