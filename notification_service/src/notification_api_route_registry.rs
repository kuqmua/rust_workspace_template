// The owner module retains lint-sensitive semantics from the original implementation.

frontend_contract_macros::route_registry! {
    #[openapi()]
    pub(super);
    state = crate::notification_state::NotificationState,
    family = notification_service_contract::notification_route::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::notification_message::NotificationMessage,
        notification_service_contract::uuid_notification_id::UuidNotificationId
    );
    (
        notification_service_contract::create_notification_route::CreateNotificationRoute,
        crate::create_notification::create_notification
    ),
}
