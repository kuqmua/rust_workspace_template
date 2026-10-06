#[tokio::test]
async fn test_remote_source_implements_synchronization_source_contract() {
    let mut source = crate::remote_sync_source::RemoteSyncSource::new(
        synchronization_service_runtime::synchronization_payload::SynchronizationPayload::try_from(
            vec![1u8, 2u8],
        )
        .expect(constants_str::DIAGNOSTIC_DE19443D),
    );
    assert_eq!(usize::from(source.request_count()), constants_usize::ZERO);
    let payload =
        synchronization_service_runtime::synchronization_source::SynchronizationSource::read(
            &mut source,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A64993D6);
    assert_eq!(payload.as_ref(), &[1u8, 2u8]);
    assert_eq!(usize::from(source.request_count()), constants_usize::ONE);
    let repeated_read =
        synchronization_service_runtime::synchronization_source::SynchronizationSource::read(
            &mut source,
        )
        .await;
    assert!(repeated_read.is_ok_and(|synchronization_payload| {
        synchronization_payload.as_ref() == payload.as_ref()
    }));
    assert_eq!(source.payload().as_ref(), payload.as_ref());
    assert_eq!(usize::from(source.request_count()), 2usize);
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
    drop(provider);
    assert!(
        inbox
            .receive()
            .await
            .is_some_and(|runtime_notification_message| {
                runtime_notification_message.as_ref() == constants_str::TEST_NOTIFICATION_MESSAGE
            })
    );
    assert!(inbox.receive().await.is_none());
}

#[tokio::test]
async fn test_notification_provider_reports_closed_receiver_through_runtime_contract() {
    let (provider, inbox) =
        crate::create_mock_notification_provider::create_mock_notification_provider();
    drop(inbox);
    let message =
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage::try_from(
            constants_str::TEST_NOTIFICATION_MESSAGE.to_owned(),
        );
    assert!(message.is_ok());
    if let Ok(runtime_notification_message) = message {
        let result = server_runtime_http::notification_sender::NotificationSender::send(
            &provider,
            runtime_notification_message,
        )
        .await;
        assert!(matches!(
            result,
            Err(crate::mock_notification_provider_closed::MockNotificationProviderClosed::Closed)
        ));
    }
}
