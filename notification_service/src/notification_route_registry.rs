proc_macro_frontend_contract::endpoint_registry! {
    pub(super);
    state = crate::notification_state::NotificationState;
    (
        notification_service_contract::notification_operational_route::NotificationOperationalRoute::Metrics,
        crate::metrics::metrics
    ),
    (
        notification_service_contract::notification_operational_route::NotificationOperationalRoute::OpenApi,
        super::notification_open_api::notification_open_api
    ),
}
