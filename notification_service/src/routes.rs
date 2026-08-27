#[path = "routes/create_notification.rs"]
mod create_notification;
#[path = "routes/metrics.rs"]
mod metrics;
#[path = "routes/notification_api_route_registry.rs"]
mod notification_api_route_registry;
#[path = "routes/notification_route_registry.rs"]
mod notification_route_registry;
#[path = "routes/open_api.rs"]
mod open_api;
#[cfg(test)]
#[path = "routes/open_api_document.rs"]
mod open_api_document;
#[path = "routes/router.rs"]
mod router;

#[cfg(test)]
pub(crate) use open_api_document::open_api_document;
pub(crate) use router::router;
