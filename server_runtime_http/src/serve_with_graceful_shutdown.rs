// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::integer_division_remainder_used)]
pub async fn serve_with_graceful_shutdown<Shutdown>(
    tokio_tcp_listener: crate::tokio_tcp_listener::TokioTcpListener,
    axum_router: crate::axum_router::AxumRouter,
    shutdown: Shutdown,
    request_timeout_duration: crate::request_timeout_duration::RequestTimeoutDuration,
) -> Result<(), crate::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError>
where
    Shutdown: Future<Output = ()> + Send + 'static,
{
    let (shutdown_started_tx, shutdown_started_rx) = tokio::sync::oneshot::channel();
    let server = IntoFuture::into_future(
        axum::serve(
            tokio_tcp_listener.into_inner(),
            axum::Router::from(axum_router)
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .with_graceful_shutdown(async move {
            shutdown.await;
            let _send_result = shutdown_started_tx.send(());
        }),
    );
    tokio::pin!(server);
    tokio::select! {
        result = &mut server => result.map_err(|error| crate::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError::Serve(crate::serve_io_error::ServeIoError::from(error))),
        _shutdown_result = shutdown_started_rx => {
            tokio::time::timeout(request_timeout_duration.get(), &mut server)
                .await
                .map_err(|_elapsed| crate::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError::ShutdownTimeout)?
                .map_err(|error| crate::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError::Serve(crate::serve_io_error::ServeIoError::from(error)))
        }
    }
}
