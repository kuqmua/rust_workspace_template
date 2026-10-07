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
    let mut cloned_source = source.clone();
    assert_eq!(
        usize::from(cloned_source.request_count()),
        constants_usize::ONE
    );
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
    assert_eq!(
        usize::from(cloned_source.request_count()),
        constants_usize::ONE
    );
    let cloned_read =
        synchronization_service_runtime::synchronization_source::SynchronizationSource::read(
            &mut cloned_source,
        )
        .await;
    assert!(cloned_read.is_ok_and(|synchronization_payload| {
        synchronization_payload.as_ref() == payload.as_ref()
    }));
    assert_eq!(usize::from(cloned_source.request_count()), 2usize);
    assert_eq!(usize::from(source.request_count()), 2usize);
    assert_eq!(cloned_source.payload().as_ref(), payload.as_ref());
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

#[test]
fn test_remote_sync_request_count_saturates_at_native_maximum() {
    let maximum = std::num::NonZeroUsize::MAX.get();
    [maximum.saturating_sub(constants_usize::ONE), maximum]
        .into_iter()
        .fold((), |(), value| {
            let mut request_count =
                crate::remote_sync_request_count::RemoteSyncRequestCount::from(value);
            request_count.increment();
            assert_eq!(usize::from(request_count), maximum);
            request_count.increment();
            assert_eq!(usize::from(request_count), maximum);
        });
}

#[tokio::test]
async fn test_cloned_notification_provider_preserves_fifo_and_sender_lifetime() {
    let (provider, mut inbox) =
        crate::create_mock_notification_provider::create_mock_notification_provider();
    let cloned_provider = provider.clone();
    let first_message =
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage::try_from(
            constants_str::TEST_NOTIFICATION_MESSAGE.to_owned(),
        );
    assert!(first_message.is_ok());
    let Ok(first_notification) = first_message else {
        return;
    };
    assert_eq!(
        server_runtime_http::notification_sender::NotificationSender::send(
            &provider,
            first_notification,
        )
        .await,
        Ok(())
    );
    drop(provider);
    let second_message =
        server_runtime_http::runtime_notification_message::RuntimeNotificationMessage::try_from(
            constants_str::X.to_owned(),
        );
    assert!(second_message.is_ok());
    let Ok(second_notification) = second_message else {
        return;
    };
    assert_eq!(
        server_runtime_http::notification_sender::NotificationSender::send(
            &cloned_provider,
            second_notification,
        )
        .await,
        Ok(())
    );
    drop(cloned_provider);
    assert!(
        inbox
            .receive()
            .await
            .is_some_and(|runtime_notification_message| {
                runtime_notification_message.as_ref() == constants_str::TEST_NOTIFICATION_MESSAGE
            })
    );
    assert!(
        inbox
            .receive()
            .await
            .is_some_and(|runtime_notification_message| {
                runtime_notification_message.as_ref() == constants_str::X
            })
    );
    assert!(inbox.receive().await.is_none());
}
