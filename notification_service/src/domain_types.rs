// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each
#![allow(clippy::field_scoped_visibility_modifiers)] // sibling application and adapter modules consume these private binary domain models

#[path = "notification_state.rs"]
mod notification_state;
pub(crate) use notification_state::*;
#[path = "axum_notification_state.rs"]
mod axum_notification_state;
pub(crate) use axum_notification_state::*;
#[path = "axum_notification_json.rs"]
mod axum_notification_json;
pub(crate) use axum_notification_json::*;
#[path = "axum_notification_response.rs"]
mod axum_notification_response;
pub(crate) use axum_notification_response::*;
#[path = "axum_notification_router.rs"]
mod axum_notification_router;
pub(crate) use axum_notification_router::*;
#[path = "http_notification_status_code.rs"]
mod http_notification_status_code;
pub(crate) use http_notification_status_code::*;
#[path = "create_notification_error.rs"]
mod create_notification_error;
pub(crate) use create_notification_error::*;
#[path = "metrics_error.rs"]
mod metrics_error;
pub(crate) use metrics_error::*;
#[path = "metrics_exporter_prometheus_renderer.rs"]
mod metrics_exporter_prometheus_renderer;
pub(crate) use metrics_exporter_prometheus_renderer::*;
#[path = "notification_body_maximum_bytes.rs"]
mod notification_body_maximum_bytes;
pub(crate) use notification_body_maximum_bytes::*;
#[path = "notification_exit_code.rs"]
mod notification_exit_code;
pub(crate) use notification_exit_code::*;
#[path = "notification_service_error.rs"]
mod notification_service_error;
pub(crate) use notification_service_error::*;
#[path = "notification_config_error.rs"]
mod notification_config_error;
pub(crate) use notification_config_error::*;
#[path = "sqlx_notification_database_error.rs"]
mod sqlx_notification_database_error;
pub(crate) use sqlx_notification_database_error::*;
#[path = "sqlx_notification_migration_error.rs"]
mod sqlx_notification_migration_error;
pub(crate) use sqlx_notification_migration_error::*;
#[path = "notification_io_error.rs"]
mod notification_io_error;
pub(crate) use notification_io_error::*;
#[path = "notification_serve_error.rs"]
mod notification_serve_error;
pub(crate) use notification_serve_error::*;
#[path = "metrics_exporter_prometheus_notification_build_error.rs"]
mod metrics_exporter_prometheus_notification_build_error;
pub(crate) use metrics_exporter_prometheus_notification_build_error::*;
#[path = "notification_observability_init_error.rs"]
mod notification_observability_init_error;
pub(crate) use notification_observability_init_error::*;
#[path = "notification_observability_shutdown_error.rs"]
mod notification_observability_shutdown_error;
pub(crate) use notification_observability_shutdown_error::*;
#[path = "notification_error_code.rs"]
mod notification_error_code;
pub(crate) use notification_error_code::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
