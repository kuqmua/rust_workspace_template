#[tokio::test]
async fn test_remote_source_implements_synchronization_source_contract() {
    let mut source = crate::remote_sync_source::RemoteSyncSource::new(
        synchronization_service_runtime::synchronization_payload::SynchronizationPayload::try_from(
            vec![1u8, 2u8],
        )
        .expect(constants_str::DIAGNOSTIC_DE19443D),
    );
    let payload =
        synchronization_service_runtime::synchronization_source::SynchronizationSource::read(
            &mut source,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A64993D6);
    assert_eq!(payload.as_ref(), &[1u8, 2u8]);
    assert_eq!(usize::from(source.request_count()), constants_usize::ONE);
}

#[tokio::test]
async fn test_notification_provider_records_messages_through_runtime_contract() {
    let (provider, mut inbox) =
        crate::create_mock_notification_provider::create_mock_notification_provider();
    let message =
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage::try_from(
            constants_str::TEST_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_6EF25D4A);
    let result =
        server_runtime_http::notification_sender::NotificationSender::send(&provider, message)
            .await;
    assert_eq!(result, Ok(()));
    assert!(inbox.receive().await.is_some());
}
