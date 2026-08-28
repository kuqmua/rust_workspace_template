#[path = "add_status_route.rs"]
mod add_status_route;
#[path = "serve_io_error.rs"]
mod serve_io_error;
#[path = "serve_with_graceful_shutdown.rs"]
mod serve_with_graceful_shutdown;
#[path = "serve_with_graceful_shutdown_error.rs"]
mod serve_with_graceful_shutdown_error;
#[path = "service_runtime.rs"]
mod service_runtime;
#[path = "tokio_tcp_listener.rs"]
mod tokio_tcp_listener;

pub use add_status_route::add_status_route;
pub use serve_io_error::ServeIoError;
pub use serve_with_graceful_shutdown::serve_with_graceful_shutdown;
pub use serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError;
pub use service_runtime::ServiceRuntime;
pub use tokio_tcp_listener::TokioTcpListener;

#[cfg(test)]
mod tests {
    #[test]
    fn service_runtime_returns_owned_parts() {
        let runtime =
            super::ServiceRuntime::new(super::super::AxumRouter::from(axum::Router::new()), None);
        let (_router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
    }
}
