#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ServiceRuntime {
    optional_task: Option<crate::BackgroundTask>,
    router: crate::AxumRouter,
}

impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(self) -> (crate::AxumRouter, Option<crate::BackgroundTask>) {
        (self.router, self.optional_task)
    }

    #[must_use]
    pub const fn new(
        router: crate::AxumRouter,
        optional_task: Option<crate::BackgroundTask>,
    ) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}
