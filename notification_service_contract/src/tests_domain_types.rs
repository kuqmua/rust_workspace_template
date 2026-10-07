#![allow(
    unused_variables,
    reason = "test trait fixtures preserve repository type-based parameter names"
)]

#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    struct ClientTransport;
    impl frontend_contract::transport::Transport for ClientTransport {
        fn send(
            &self,
            transport_request: frontend_contract::transport_request::TransportRequest,
        ) -> impl Future<
            Output = Result<
                frontend_contract::transport_response::TransportResponse,
                frontend_contract::transport_error::TransportError,
            >,
        > + '_ {
            std::future::ready(Err(
                frontend_contract::transport_error::TransportError::default(),
            ))
        }
    }
    #[test]
    fn test_every_notification_route_has_named_route_and_client_functions() {
        assert_eq!(
            <crate::notification_route::NotificationRouteFamily as frontend_contract::route_family::RouteFamily>::ROUTE_COUNT,
            constants_usize::ONE
        );
        assert_eq!(
            crate::create_notification_route::create_notification_route(),
            crate::notification_route::NotificationRoute::Create
                .contract()
                .path()
        );
        assert_eq!(
            size_of_val(
                &crate::create_notification_route::create_notification_client::<ClientTransport>
            ),
            constants_usize::ZERO
        );
        assert_eq!(
            <crate::notification_operational_route::NotificationOperationalRouteFamily as frontend_contract::route_family::RouteFamily>::ROUTE_COUNT,
            constants_usize::ZERO
        );
        assert_eq!(
            crate::notification_operational_route::metrics_route(),
            crate::notification_operational_route::NotificationOperationalRoute::Metrics
                .contract()
                .path()
        );
        assert_eq!(
            crate::notification_operational_route::open_api_route(),
            crate::notification_operational_route::NotificationOperationalRoute::OpenApi
                .contract()
                .path()
        );
        assert!(
            [
                crate::notification_operational_route::NotificationOperationalRoute::Metrics,
                crate::notification_operational_route::NotificationOperationalRoute::OpenApi,
            ]
            .into_iter()
            .all(|route| {
                route
                    .contract()
                    .path()
                    .as_ref()
                    .ends_with(constants_str::READ_ROUTE_SUFFIX)
            })
        );
        assert_eq!(
            size_of_val(&crate::notification_operational_route::metrics_client::<ClientTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            size_of_val(&crate::notification_operational_route::open_api_client::<ClientTransport>),
            constants_usize::ZERO
        );
    }
    #[test]
    fn test_notification_message_enforces_bounds() {
        assert!(matches!(
            crate::notification_message::NotificationMessage::try_from(String::new()),
            Err(crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError::Empty)
        ));
        assert!(matches!(
            crate::notification_message::NotificationMessage::try_from(
                constants_str::VALUE_B24D6D33.to_owned()
            ),
            Ok(_value)
        ));
        assert!(matches!(
            crate::notification_message::NotificationMessage::try_from(constants_str::X.repeat(4_097usize)),
            Err(crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError::TooLong)
        ));
    }

    #[test]
    fn test_notification_message_preserves_ascii_and_unicode_byte_boundaries() {
        assert!([
            constants_str::X.repeat(4_096usize),
            '\u{00e9}'.to_string().repeat(2_048usize),
        ].into_iter().all(|input| {
            let length = input.len();
            let pointer = input.as_ptr();
            let decoded = <crate::notification_message::NotificationMessage as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(input.clone()),
            );
            decoded.is_ok_and(|notification_message| notification_message.as_ref() == input)
                && crate::notification_message::NotificationMessage::try_from(input).is_ok_and(|notification_message| {
                    notification_message.as_ref().len() == length
                        && notification_message.as_ref().as_ptr() == pointer
                })
        }));
        let oversized = '\u{00e9}'.to_string().repeat(2_049usize);
        let decoded =
            <crate::notification_message::NotificationMessage as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                    oversized.clone(),
                ),
            );
        assert!(decoded.is_err_and(|error| error.to_string().contains(
            crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError::TooLong.to_string().as_str()
        )));
        assert!(matches!(crate::notification_message::NotificationMessage::try_from(oversized),
            Err(crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError::TooLong)
        ));
    }

    #[test]
    fn test_notification_message_deserialization_enforces_bounds() {
        let _empty_error =
            <crate::notification_message::NotificationMessage as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
            )
            .expect_err(constants_str::VALUE_61A01611);
        let _too_long_error =
            <crate::notification_message::NotificationMessage as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                    constants_str::X.repeat(
                        crate::notification_message_max_len::NOTIFICATION_MESSAGE_MAX_LEN
                            + constants_usize::ONE,
                    ),
                ),
            )
            .expect_err(constants_str::VALUE_F2CF39E2);
    }

    #[test]
    fn test_notification_request_transfers_message_without_reallocating_text() {
        assert!(
            [constants_str::VALUE_B24D6D33, constants_str::U_1F496]
                .into_iter()
                .all(|text| {
                    let input = String::from(text);
                    let pointer = input.as_ptr();
                    crate::notification_message::NotificationMessage::try_from(input).is_ok_and(
                        |notification_message| {
                            let request =
                                crate::create_notification_request::CreateNotificationRequest::new(
                                    notification_message,
                                );
                            let transferred = request.into_message();
                            transferred.as_ref() == text && transferred.as_ref().as_ptr() == pointer
                        },
                    )
                })
        );
    }

    #[test]
    fn test_notification_request_deserialization_revalidates_message_and_rejects_unknown_fields() {
        assert!([
            (constants_str::VALUE_B24D6D33.to_owned(), true),
            (constants_str::U_1F496.to_owned(), true),
            (constants_str::EMPTY.to_owned(), false),
            (constants_str::X.repeat(4_097usize), false),
        ].into_iter().all(|(text, accepted)| {
            let decoded = <crate::create_notification_request::CreateNotificationRequest as serde::Deserialize>::deserialize(
                serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new([(stringify!(message), text.as_str())].into_iter()),
            );
            decoded.as_ref().is_ok() == accepted
                && decoded.is_ok_and(|request| request.into_message().as_ref() == text) == accepted
        }));
        assert!(<crate::create_notification_request::CreateNotificationRequest as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                std::iter::once((stringify!(message), constants_str::VALUE_B24D6D33)).take(0usize),
            ),
        ).is_err_and(|error| error.to_string().contains(stringify!(message))));
        assert!(<crate::create_notification_request::CreateNotificationRequest as serde::Deserialize>::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new([
                (stringify!(message), constants_str::VALUE_B24D6D33),
                (stringify!(unexpected), constants_str::VALUE_B24D6D33),
            ].into_iter()),
        ).is_err_and(|error| error.to_string().contains(stringify!(unexpected))));
    }
}
