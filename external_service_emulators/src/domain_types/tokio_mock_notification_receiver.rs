use tokio::sync::mpsc::UnboundedReceiver;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationReceiver(
    pub(super) UnboundedReceiver<server_runtime_http::domain_types::NotificationMessage>,
);
