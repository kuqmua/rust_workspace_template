#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use tokio::sync::mpsc::UnboundedSender;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationSender(
    pub(super) UnboundedSender<server_runtime_http::notification_message::NotificationMessage>,
);
