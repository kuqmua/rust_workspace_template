// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)]

use super::metrics::metrics;
use super::open_api::open_api;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::domain_types::NotificationState;
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::Metrics,
        metrics
    ),
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::OpenApi,
        open_api
    ),
)]
pub(super) struct NotificationRouteRegistry;
