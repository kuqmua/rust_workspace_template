#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(super) struct TokioMockNotificationReceiver(
    pub(super) UnboundedReceiver<server_runtime_http::notification_message::NotificationMessage>,
);
