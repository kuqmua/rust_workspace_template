#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ServiceRuntime {
    optional_task: Option<super::super::BackgroundTask>,
    router: super::super::AxumRouter,
}

impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        super::super::AxumRouter,
        Option<super::super::BackgroundTask>,
    ) {
        (self.router, self.optional_task)
    }

    #[must_use]
    pub const fn new(
        router: super::super::AxumRouter,
        optional_task: Option<super::super::BackgroundTask>,
    ) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}
