// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::integer_division_remainder_used)]
pub async fn serve_with_graceful_shutdown<Shutdown>(
    listener: super::TokioTcpListener,
    router: crate::AxumRouter,
    shutdown: Shutdown,
    shutdown_timeout: crate::RequestTimeoutDuration,
) -> Result<(), super::ServeWithGracefulShutdownError>
where
    Shutdown: Future<Output = ()> + Send + 'static,
{
    let (shutdown_started_tx, shutdown_started_rx) = tokio::sync::oneshot::channel();
    let server = IntoFuture::into_future(
        axum::serve(
            listener.0,
            axum::Router::from(router)
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .with_graceful_shutdown(async move {
            shutdown.await;
            let _send_result = shutdown_started_tx.send(());
        }),
    );
    tokio::pin!(server);
    tokio::select! {
        result = &mut server => result.map_err(|error| super::ServeWithGracefulShutdownError::Serve(super::ServeIoError(error))),
        _shutdown_result = shutdown_started_rx => {
            tokio::time::timeout(shutdown_timeout.get(), &mut server)
                .await
                .map_err(|_elapsed| super::ServeWithGracefulShutdownError::ShutdownTimeout)?
                .map_err(|error| super::ServeWithGracefulShutdownError::Serve(super::ServeIoError(error)))
        }
    }
}
