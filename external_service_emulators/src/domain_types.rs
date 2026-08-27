#[path = "domain_types/mock_notification_inbox.rs"]
mod mock_notification_inbox;
#[path = "domain_types/mock_notification_provider.rs"]
mod mock_notification_provider;
#[path = "domain_types/mock_notification_provider_closed.rs"]
mod mock_notification_provider_closed;
#[path = "domain_types/remote_sync_request_count.rs"]
mod remote_sync_request_count;
#[path = "domain_types/remote_sync_source.rs"]
mod remote_sync_source;
#[path = "domain_types/tokio_mock_notification_receiver.rs"]
mod tokio_mock_notification_receiver;
#[path = "domain_types/tokio_mock_notification_sender.rs"]
mod tokio_mock_notification_sender;
pub use mock_notification_inbox::MockNotificationInbox;
pub use mock_notification_provider::{MockNotificationProvider, mock_notification_provider};
pub use mock_notification_provider_closed::MockNotificationProviderClosed;
pub use remote_sync_request_count::RemoteSyncRequestCount;
pub use remote_sync_source::RemoteSyncSource;
use tokio_mock_notification_receiver::TokioMockNotificationReceiver;
use tokio_mock_notification_sender::TokioMockNotificationSender;

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
