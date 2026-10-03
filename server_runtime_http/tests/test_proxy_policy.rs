#![allow(
    unused_crate_dependencies,
    reason = "integration target exercises one HTTP path from a workspace crate with many dependencies"
)]

#[cfg(test)]
mod tests {
    async fn create_test_http_listener() -> Result<
        server_runtime_http::tokio_tcp_listener::TokioTcpListener,
        server_runtime_http::serve_io_error::ServeIoError,
    > {
        tokio::net::TcpListener::bind((constants_str::VALUE_127_0_0_1, 0u16))
            .await
            .map(server_runtime_http::tokio_tcp_listener::TokioTcpListener::from)
            .map_err(server_runtime_http::serve_io_error::ServeIoError::from)
    }

    #[tokio::test(start_paused = true)]
    #[ignore = "provisions a local HTTP listener and client; run through workspace_test_runner database"]
    async fn test_graceful_shutdown_deadline_reports_pending_request() {
        let listener_result = create_test_http_listener().await;
        assert!(listener_result.is_ok());
        let Ok(tokio_tcp_listener) = listener_result else {
            return;
        };
        let listener = tokio_tcp_listener.into_inner();
        let address_result = listener.local_addr();
        assert!(address_result.is_ok());
        let Ok(address) = address_result else {
            return;
        };
        let url_result = reqwest::Url::parse(&format!("http://{address}"));
        assert!(url_result.is_ok());
        let Ok(url) = url_result else {
            return;
        };
        let client_result = reqwest::Client::builder().no_proxy().build();
        assert!(client_result.is_ok());
        let Ok(client) = client_result else {
            return;
        };
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(2u64),
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        let Ok(request_timeout_duration) = timeout_result else {
            return;
        };
        let (entered_sender, mut entered_receiver) = tokio::sync::mpsc::channel(1usize);
        let (release_sender, release_receiver) = tokio::sync::watch::channel(false);
        let router = axum::Router::new().fallback(move || {
            let entered = entered_sender.clone();
            let mut release = release_receiver.clone();
            async move {
                if entered.send(()).await.is_err() || release.changed().await.is_err() {
                    http::StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    http::StatusCode::OK
                }
            }
        });
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            server_runtime_http::serve_with_graceful_shutdown::serve_with_graceful_shutdown(
                server_runtime_http::tokio_tcp_listener::TokioTcpListener::from(listener),
                server_runtime_http::axum_router::AxumRouter::from(router),
                async move {
                    let received = shutdown_receiver.await;
                    assert_eq!(received, Ok(()));
                },
                request_timeout_duration,
            )
            .await
        });
        let request = tokio::spawn(async move { client.get(url).send().await });
        let entered_result = entered_receiver.recv().await;
        let shutdown_result = shutdown_sender.send(());
        let server_result = server.await;
        let release_result = release_sender.send(true);
        let request_result = request.await;
        assert_eq!(entered_result, Some(()));
        assert!(shutdown_result.is_ok());
        assert!(release_result.is_ok_and(|()| *release_sender.borrow()));
        assert!(matches!(server_result, Ok(Err(server_runtime_http::serve_with_graceful_shutdown_error::ServeWithGracefulShutdownError::ShutdownTimeout))));
        assert!(request_result.is_ok_and(|result| {
            result.is_ok_and(|response| response.status() == http::StatusCode::OK)
        }));
    }

    #[tokio::test]
    #[ignore = "provisions a local HTTP listener and client; requires loopback proxy bypass; run through workspace_test_runner database"]
    async fn test_http_client_preserves_success_and_server_error_responses() {
        let listener_result = create_test_http_listener()
            .await
            .map(server_runtime_http::tokio_tcp_listener::TokioTcpListener::into_inner);
        assert!(listener_result.is_ok());
        let Ok(listener) = listener_result else {
            return;
        };
        let address_result = listener.local_addr();
        assert!(address_result.is_ok());
        let Ok(address) = address_result else {
            return;
        };
        let url_result = reqwest::Url::parse(&format!(
            "{}:{}",
            constants_str::HTTP_LOCALHOST,
            address.port()
        ));
        assert!(url_result.is_ok());
        let Ok(url) = url_result else {
            return;
        };
        let timeout_configuration = server_runtime_http::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration::try_from(std::time::Duration::from_secs(1u64)).and_then(|reqwest_connect_timeout_duration| {
            server_runtime_http::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration::try_from(std::time::Duration::from_secs(2u64)).map(|reqwest_request_timeout_duration| {
                server_runtime_http::reqwest_client_policy::ReqwestClientPolicy::new(
                    reqwest_connect_timeout_duration,
                    reqwest_request_timeout_duration,
                    server_runtime_http::outbound_host_policy::OutboundHostPolicy::AllowPrivate,
                )
            })
        });
        assert!(
            timeout_configuration
                .as_ref()
                .is_ok_and(|policy| policy.host_policy()
                    == server_runtime_http::outbound_host_policy::OutboundHostPolicy::AllowPrivate)
        );
        let Ok(policy) = timeout_configuration else {
            return;
        };
        let client_result = server_runtime_http::reqwest_client::ReqwestClient::try_new(policy);
        assert!(client_result.is_ok());
        let Ok(client) = client_result else {
            return;
        };
        let reqwest_client = client;
        let router = axum::Router::new().route(
            constants_str::SLASH,
            axum::routing::get(async || {
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    constants_str::ABCD_ALT,
                )
            })
            .post(async || (http::StatusCode::OK, constants_str::ABCD_ALT)),
        );
        let timeout_result =
            server_runtime_http::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(2u64),
            );
        assert_ne!(timeout_result, Err(server_runtime_http::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero));
        let Ok(request_timeout_duration) = timeout_result else {
            return;
        };
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            server_runtime_http::serve_with_graceful_shutdown::serve_with_graceful_shutdown(
                server_runtime_http::tokio_tcp_listener::TokioTcpListener::from(listener),
                server_runtime_http::axum_router::AxumRouter::from(router),
                async move {
                    let shutdown_received = shutdown_receiver.await;
                    assert_eq!(shutdown_received, Ok(()));
                },
                request_timeout_duration,
            )
            .await
        });
        let request = reqwest::Request::new(reqwest::Method::GET, url.clone());
        let response_result = reqwest_client
            .execute(server_runtime_http::reqwest_request::ReqwestRequest::from(
                request,
            ))
            .await;
        let response_matches = if let Ok(response) = response_result {
            let inner = response.into_inner();
            let status_matches = inner.status() == http::StatusCode::INTERNAL_SERVER_ERROR;
            status_matches
                && inner
                    .text()
                    .await
                    .is_ok_and(|text| text == constants_str::ABCD_ALT)
        } else {
            false
        };
        let success_response = reqwest_client
            .execute(server_runtime_http::reqwest_request::ReqwestRequest::from(
                reqwest::Request::new(reqwest::Method::POST, url),
            ))
            .await;
        let success_matches = success_response
            .is_ok_and(|response| response.into_inner().status() == http::StatusCode::OK);
        let shutdown_result = shutdown_sender.send(());
        let server_result = server.await;
        assert!(shutdown_result.is_ok());
        assert!(server_result.is_ok_and(|result| result.is_ok()));
        assert!(response_matches);
        assert!(success_matches);
    }

    #[tokio::test]
    #[ignore = "requires HTTP_PROXY to point to the local test listener"]
    async fn test_reject_private_checks_target_dns_with_ambient_proxy() {
        let listener = tokio::net::TcpListener::bind(constants_str::TEST_LOCAL_PROXY_SOCKET)
            .await
            .expect(constants_str::DIAGNOSTIC_A17C436E);
        let proxy_task = tokio::spawn(async move {
            let (mut connection, _) = listener
                .accept()
                .await
                .expect(constants_str::DIAGNOSTIC_B87A120D);
            tokio::io::AsyncWriteExt::write_all(
                &mut connection,
                b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n",
            )
            .await
            .expect(constants_str::DIAGNOSTIC_A9E5126F);
        });
        let policy = server_runtime_http::reqwest_client_policy::ReqwestClientPolicy::new(
            server_runtime_http::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration::try_from(
                std::time::Duration::from_secs(1),
            )
            .expect(constants_str::DIAGNOSTIC_C9037A11),
            server_runtime_http::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration::try_from(
                std::time::Duration::from_secs(2),
            )
            .expect(constants_str::DIAGNOSTIC_C3D62B8F),
            server_runtime_http::outbound_host_policy::OutboundHostPolicy::RejectPrivate,
        );
        let client = server_runtime_http::reqwest_client::ReqwestClient::try_new(policy)
            .expect(constants_str::DIAGNOSTIC_AA8917E4);
        let request = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse(constants_str::HTTP_LOCALHOST)
                .expect(constants_str::DIAGNOSTIC_D32A9F83),
        );
        let result = client
            .execute(server_runtime_http::reqwest_request::ReqwestRequest::from(
                request,
            ))
            .await;
        proxy_task.abort();
        let _proxy_result = proxy_task.await;
        assert!(result.is_err_and(|reqwest_error| {
            std::iter::successors(std::error::Error::source(&reqwest_error), |source| {
                source.source()
            })
            .any(|source| {
                source.downcast_ref::<server_runtime_http::outbound_url_error::OutboundUrlError>()
                    == Some(
                        &server_runtime_http::outbound_url_error::OutboundUrlError::ForbiddenHost,
                    )
            })
        }));
    }

    #[test]
    #[ignore = "constructs an HTTP client; run through workspace_test_runner database"]
    #[cfg_attr(
        miri,
        ignore = "native TLS initialization calls OpenSSL functions that Miri does not support"
    )]
    fn test_validates_and_applies_w3c_trace_context() {
        let trace_parent = server_runtime_http::http_trace_parent::HttpTraceParent::try_from(
            constants_str::TRACEPARENT_TEST_VALUE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_6B490BF8);
        let trace_state = server_runtime_http::http_trace_state::HttpTraceState::try_from(
            constants_str::TRACESTATE_TEST_VALUE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_B82FB9EF);
        let request_id = server_runtime_http::request_id::RequestId::try_from(
            constants_str::REQUEST_ID_TEST_VALUE.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_50C01EA8);
        let client = server_runtime_http::reqwest_client::ReqwestClient::try_new(
            server_runtime_http::reqwest_client_policy::ReqwestClientPolicy::new(
                server_runtime_http::reqwest_connect_timeout_duration::ReqwestConnectTimeoutDuration::try_from(
                    std::time::Duration::from_secs(1u64),
                )
                .expect(constants_str::DIAGNOSTIC_CE032A9F),
                server_runtime_http::reqwest_request_timeout_duration::ReqwestRequestTimeoutDuration::try_from(
                    std::time::Duration::from_secs(2u64),
                )
                .expect(constants_str::DIAGNOSTIC_A1DABED3),
                server_runtime_http::outbound_host_policy::OutboundHostPolicy::AllowPrivate,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_8DED9D63);
        let request_builder: reqwest::RequestBuilder =
            server_runtime_http::outbound_trace_context::OutboundTraceContext::new(
                trace_parent,
                Some(trace_state),
                Some(request_id),
            )
            .apply(
                reqwest::Client::from(client)
                    .get(constants_str::HTTPS_EXAMPLE_COM)
                    .into(),
            )
            .into();
        let request = request_builder
            .build()
            .expect(constants_str::DIAGNOSTIC_1574578F);
        assert_eq!(
            request.headers()[constants_str::TRACESTATE],
            constants_str::TRACESTATE_TEST_VALUE
        );
        assert_eq!(
            request.headers()[constants_str::X_REQUEST_ID],
            constants_str::REQUEST_ID_TEST_VALUE
        );
    }
}
