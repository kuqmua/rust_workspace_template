#[cfg(test)]
mod tests {
    #[test]
    fn test_service_runtime_returns_owned_parts() {
        let runtime = crate::service_runtime::ServiceRuntime::new(
            crate::axum_router::AxumRouter::from(axum::Router::new()),
            None,
        );
        let (_router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
    }
}
