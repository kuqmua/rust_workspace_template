#[cfg(test)]
mod tests {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    struct ClientTransport;
    impl frontend_contract::transport::Transport for ClientTransport {
        fn send(
            &self,
            _request: frontend_contract::transport_request::TransportRequest,
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
            crate::notification_message::NotificationMessage::try_from("ready".to_owned()),
            Ok(_value)
        ));
        assert!(matches!(
            crate::notification_message::NotificationMessage::try_from("x".repeat(4_097usize)),
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
}
