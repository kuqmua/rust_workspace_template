use tokio::sync::mpsc::UnboundedSender;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationSender(
    pub(super) UnboundedSender<server_runtime_http::domain_types::NotificationMessage>,
);
