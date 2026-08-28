pub use crate::add_status_route::add_status_route;
pub use crate::serve_io_error::ServeIoError;
pub use crate::serve_with_graceful_shutdown::serve_with_graceful_shutdown;
pub use crate::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError;
pub use crate::service_runtime::ServiceRuntime;
pub use crate::tokio_tcp_listener::TokioTcpListener;

#[cfg(test)]
mod tests {
    #[test]
    fn service_runtime_returns_owned_parts() {
        let runtime =
            super::ServiceRuntime::new(crate::AxumRouter::from(axum::Router::new()), None);
        let (_router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
    }
}

// Root-owned module compatibility wrappers.
mod add_status_route {
    pub use crate::add_status_route::*;
}
mod serve_io_error {
    pub use crate::serve_io_error::*;
}
mod serve_with_graceful_shutdown {
    pub use crate::serve_with_graceful_shutdown::*;
}
mod serve_with_graceful_shutdown_error {
    pub use crate::serve_with_graceful_shutdown_error::*;
}
mod service_runtime {
    pub use crate::service_runtime::*;
}
mod tokio_tcp_listener {
    pub use crate::tokio_tcp_listener::*;
}
