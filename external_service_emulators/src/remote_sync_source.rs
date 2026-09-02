#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct RemoteSyncSource {
    payload: synchronization_service_runtime::synchronization_payload::SynchronizationPayload,
    #[getters(copy)]
    request_count: crate::remote_sync_request_count::RemoteSyncRequestCount,
}

impl RemoteSyncSource {
    #[must_use]
    pub fn new(
        payload: synchronization_service_runtime::synchronization_payload::SynchronizationPayload,
    ) -> Self {
        Self {
            payload,
            request_count: crate::remote_sync_request_count::RemoteSyncRequestCount::from(
                constants_usize::ZERO,
            ),
        }
    }
}

impl synchronization_service_runtime::synchronization_source::SynchronizationSource
    for RemoteSyncSource
{
    type Error = std::convert::Infallible;

    fn read(
        &mut self,
    ) -> impl Future<
        Output = Result<
            synchronization_service_runtime::synchronization_payload::SynchronizationPayload,
            Self::Error,
        >,
    > + Send {
        self.request_count.increment();
        std::future::ready(Ok(self.payload.clone()))
    }
}
