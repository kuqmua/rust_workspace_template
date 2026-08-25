#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct MockNotificationProvider {
    sender: TokioMockNotificationSender,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct MockNotificationInbox {
    receiver: TokioMockNotificationReceiver,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct TokioMockNotificationSender(
    tokio::sync::mpsc::UnboundedSender<server_runtime_http::domain_types::NotificationMessage>,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct TokioMockNotificationReceiver(
    tokio::sync::mpsc::UnboundedReceiver<server_runtime_http::domain_types::NotificationMessage>,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::MOCK_NOTIFICATION_PROVIDER_CLOSED)]
pub struct MockNotificationProviderClosed;

impl MockNotificationInbox {
    pub async fn receive(
        &mut self,
    ) -> Option<server_runtime_http::domain_types::NotificationMessage> {
        self.receiver.0.recv().await
    }
}

impl server_runtime_http::domain_types::NotificationSender for MockNotificationProvider {
    type Error = MockNotificationProviderClosed;

    fn send(
        &self,
        message: server_runtime_http::domain_types::NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(
            self.sender
                .0
                .send(message)
                .map_err(|_error| MockNotificationProviderClosed),
        )
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct RemoteSyncRequestCount(usize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct RemoteSyncSource {
    payload: synchronization_service_runtime::domain_types::SynchronizationPayload,
    request_count: RemoteSyncRequestCount,
}

impl RemoteSyncSource {
    #[must_use]
    pub fn new(
        payload: synchronization_service_runtime::domain_types::SynchronizationPayload,
    ) -> Self {
        Self {
            payload,
            request_count: RemoteSyncRequestCount::from(constants_usize::ZERO),
        }
    }

    #[must_use]
    pub const fn request_count(&self) -> RemoteSyncRequestCount {
        self.request_count
    }
}

impl synchronization_service_runtime::domain_types::SynchronizationSource for RemoteSyncSource {
    type Error = std::convert::Infallible;

    fn read(
        &mut self,
    ) -> impl Future<
        Output = Result<
            synchronization_service_runtime::domain_types::SynchronizationPayload,
            Self::Error,
        >,
    > + Send {
        self.request_count.0 = self.request_count.0.saturating_add(constants_usize::ONE);
        std::future::ready(Ok(self.payload.clone()))
    }
}

#[must_use]
pub fn mock_notification_provider() -> (MockNotificationProvider, MockNotificationInbox) {
    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    (
        MockNotificationProvider {
            sender: TokioMockNotificationSender::from(sender),
        },
        MockNotificationInbox {
            receiver: TokioMockNotificationReceiver::from(receiver),
        },
    )
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn remote_source_implements_synchronization_source_contract() {
        let mut source = super::RemoteSyncSource::new(
            synchronization_service_runtime::domain_types::SynchronizationPayload::try_from(vec![1u8, 2u8])
                .expect("de19443d remote_source_implements_synchronization_source_contract invariant must hold"),
        );
        let payload = synchronization_service_runtime::domain_types::SynchronizationSource::read(
            &mut source,
        )
        .await
        .expect(
            "a64993d6 remote_source_implements_synchronization_source_contract invariant must hold",
        );
        assert_eq!(payload.as_ref(), &[1u8, 2u8]);
        assert_eq!(usize::from(source.request_count()), constants_usize::ONE);
    }

    #[tokio::test]
    async fn notification_provider_records_messages_through_runtime_contract() {
        let (provider, mut inbox) = super::mock_notification_provider();
        let message = server_runtime_http::domain_types::NotificationMessage::try_from(
            constants_str::TEST_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect("6ef25d4a notification_provider_records_messages_through_runtime_contract invariant must hold");
        let result =
            server_runtime_http::domain_types::NotificationSender::send(&provider, message).await;
        assert_eq!(result, Ok(()));
        assert!(inbox.receive().await.is_some());
    }
}
