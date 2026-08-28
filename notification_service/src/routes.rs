#[path = "create_notification.rs"]
mod create_notification;
#[path = "metrics.rs"]
mod metrics;
#[path = "notification_api_route_registry.rs"]
mod notification_api_route_registry;
#[path = "notification_route_registry.rs"]
mod notification_route_registry;
#[path = "open_api.rs"]
mod open_api;
#[cfg(test)]
#[path = "open_api_document.rs"]
mod open_api_document;
#[path = "router.rs"]
mod router;

#[cfg(test)]
pub(crate) use open_api_document::open_api_document;
pub(crate) use router::router;
