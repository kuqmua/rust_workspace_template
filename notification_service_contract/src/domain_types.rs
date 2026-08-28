pub use crate::create_notification_req::CreateNotificationReq;
pub use crate::create_notification_res::CreateNotificationRes;
pub use crate::create_notification_route::*;
pub use crate::notification_api_body_max_bytes::NOTIFICATION_API_BODY_MAX_BYTES;
pub use crate::notification_message::NotificationMessage;
pub(crate) use crate::notification_message_max_len::NOTIFICATION_MESSAGE_MAX_LEN;
pub use crate::notification_message_try_from_string_error::NotificationMessageTryFromStringError;
pub use crate::notification_operational_route::*;
pub use crate::notification_route::*;
pub use crate::uuid_notification_id::UuidNotificationId;

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    struct ClientTransport;
    impl frontend_contract::domain_types::Transport for ClientTransport {
        fn send(
            &self,
            _request: frontend_contract::domain_types::TransportRequest,
        ) -> impl Future<
            Output = Result<
                frontend_contract::domain_types::TransportResponse,
                frontend_contract::domain_types::TransportError,
            >,
        > + '_ {
            std::future::ready(Err(
                frontend_contract::domain_types::TransportError::default(),
            ))
        }
    }
    #[test]
    fn every_notification_route_has_named_route_and_client_functions() {
        assert_eq!(
            <super::NotificationRouteFamily as frontend_contract::domain_types::RouteFamily>::ROUTE_COUNT,
            constants_usize::ONE
        );
        assert_eq!(
            super::create_notification_route(),
            super::NotificationRoute::Create.contract().path()
        );
        assert_eq!(
            size_of_val(&super::create_notification_client::<ClientTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            <super::NotificationOperationalRouteFamily as frontend_contract::domain_types::RouteFamily>::ROUTE_COUNT,
            constants_usize::ZERO
        );
        assert_eq!(
            super::metrics_route(),
            super::NotificationOperationalRoute::Metrics
                .contract()
                .path()
        );
        assert_eq!(
            super::open_api_route(),
            super::NotificationOperationalRoute::OpenApi
                .contract()
                .path()
        );
        assert_eq!(
            size_of_val(&super::metrics_client::<ClientTransport>),
            constants_usize::ZERO
        );
        assert_eq!(
            size_of_val(&super::open_api_client::<ClientTransport>),
            constants_usize::ZERO
        );
    }
    #[test]
    fn notification_message_enforces_bounds() {
        assert!(matches!(
            super::NotificationMessage::try_from(String::new()),
            Err(super::NotificationMessageTryFromStringError::Empty)
        ));
        assert!(matches!(
            super::NotificationMessage::try_from("ready".to_owned()),
            Ok(_value)
        ));
        assert!(matches!(
            super::NotificationMessage::try_from("x".repeat(4_097usize)),
            Err(super::NotificationMessageTryFromStringError::TooLong)
        ));
    }

    #[test]
    fn notification_message_deserialization_enforces_bounds() {
        let _empty_error = <super::NotificationMessage as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(String::new()),
        )
        .expect_err(constants_str::VALUE_61A01611);
        let _too_long_error = <super::NotificationMessage as serde::Deserialize>::deserialize(
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                constants_str::X.repeat(super::NOTIFICATION_MESSAGE_MAX_LEN + constants_usize::ONE),
            ),
        )
        .expect_err(constants_str::VALUE_F2CF39E2);
    }
}
