#[cfg(test)]
mod tests {
    #[test]
    fn service_runtime_returns_owned_parts() {
        let runtime = crate::service_runtime::ServiceRuntime::new(
            crate::axum_router::AxumRouter::from(axum::Router::new()),
            None,
        );
        let (_router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
    }
}

// Root-owned module compatibility wrappers.
mod add_status_route {}
mod serve_io_error {}
mod serve_with_graceful_shutdown {}
mod serve_with_graceful_shutdown_error {}
mod service_runtime {}
mod tokio_tcp_listener {}
