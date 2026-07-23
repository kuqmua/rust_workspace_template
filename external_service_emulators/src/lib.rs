#[derive(Clone, Debug)]
pub struct MockNotificationProvider {
    sender: TokioMockNotificationSender,
}

#[derive(Debug)]
pub struct MockNotificationInbox {
    receiver: TokioMockNotificationReceiver,
}

#[derive(Clone, Debug, newtype::FromInner)]
struct TokioMockNotificationSender(
    tokio::sync::mpsc::UnboundedSender<server_runtime::NotificationMessage>,
);

#[derive(Debug, newtype::FromInner)]
struct TokioMockNotificationReceiver(
    tokio::sync::mpsc::UnboundedReceiver<server_runtime::NotificationMessage>,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", str_constants::MOCK_NOTIFICATION_PROVIDER_CLOSED)]
pub struct MockNotificationProviderClosed;

impl MockNotificationInbox {
    pub async fn receive(&mut self) -> Option<server_runtime::NotificationMessage> {
        self.receiver.0.recv().await
    }
}

impl server_runtime::NotificationSender for MockNotificationProvider {
    type Error = MockNotificationProviderClosed;

    fn send(
        &self,
        message: server_runtime::NotificationMessage,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        std::future::ready(
            self.sender
                .0
                .send(message)
                .map_err(|_error| MockNotificationProviderClosed),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct RemoteSyncRequestCount(usize);

#[derive(Clone, Debug)]
pub struct RemoteSyncSource {
    payload: synchronization_service_runtime::SynchronizationPayload,
    request_count: RemoteSyncRequestCount,
}

impl RemoteSyncSource {
    #[must_use]
    pub fn new(payload: synchronization_service_runtime::SynchronizationPayload) -> Self {
        Self {
            payload,
            request_count: RemoteSyncRequestCount::from(0usize),
        }
    }

    #[must_use]
    pub const fn request_count(&self) -> RemoteSyncRequestCount {
        self.request_count
    }
}

impl synchronization_service_runtime::SynchronizationSource for RemoteSyncSource {
    type Error = std::convert::Infallible;

    fn read(
        &mut self,
    ) -> impl Future<
        Output = Result<synchronization_service_runtime::SynchronizationPayload, Self::Error>,
    > + Send {
        self.request_count.0 = self.request_count.0.saturating_add(1usize);
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
            synchronization_service_runtime::SynchronizationPayload::try_from(vec![1u8, 2u8])
                .expect("de19443d"),
        );
        let payload = synchronization_service_runtime::SynchronizationSource::read(&mut source)
            .await
            .expect("a64993d6");
        assert_eq!(payload.as_ref(), &[1u8, 2u8]);
        assert_eq!(usize::from(source.request_count()), 1usize);
    }

    #[tokio::test]
    async fn notification_provider_records_messages_through_runtime_contract() {
        let (provider, mut inbox) = super::mock_notification_provider();
        let message = server_runtime::NotificationMessage::try_from(
            str_constants::TEST_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect("6ef25d4a");
        let result = server_runtime::NotificationSender::send(&provider, message).await;
        assert_eq!(result, Ok(()));
        assert!(inbox.receive().await.is_some());
    }
}
