#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct ServiceRuntime {
    optional_task: Option<super::BackgroundTask>,
    router: super::AxumRouter,
}

impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(self) -> (super::AxumRouter, Option<super::BackgroundTask>) {
        (self.router, self.optional_task)
    }

    #[must_use]
    pub const fn new(
        router: super::AxumRouter,
        optional_task: Option<super::BackgroundTask>,
    ) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioTcpListener(tokio::net::TcpListener);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct ServeIoError(std::io::Error);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ServeWithGracefulShutdownError {
    #[error("server failed: {0}")]
    Serve(#[source] ServeIoError),
    #[error("{}", constants_str::SERVER_GRACEFUL_SHUTDOWN_TIMED_OUT)]
    ShutdownTimeout,
}

#[must_use]
pub fn add_status_route(router: super::AxumRouter) -> super::AxumRouter {
    super::AxumRouter::from(axum::Router::from(router).route(
        constants_str::STATUS,
        axum::routing::get(async || http::StatusCode::OK),
    ))
}

#[allow(clippy::integer_division_remainder_used)] // tokio::select expansion uses internal randomized branch arithmetic
pub async fn serve_with_graceful_shutdown<Shutdown>(
    listener: TokioTcpListener,
    router: super::AxumRouter,
    shutdown: Shutdown,
    shutdown_timeout: super::RequestTimeoutDuration,
) -> Result<(), ServeWithGracefulShutdownError>
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
        result = &mut server => result.map_err(|error| ServeWithGracefulShutdownError::Serve(ServeIoError(error))),
        _shutdown_result = shutdown_started_rx => {
            tokio::time::timeout(shutdown_timeout.get(), &mut server)
                .await
                .map_err(|_elapsed| ServeWithGracefulShutdownError::ShutdownTimeout)?
                .map_err(|error| ServeWithGracefulShutdownError::Serve(ServeIoError(error)))
        }
    }
}

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
