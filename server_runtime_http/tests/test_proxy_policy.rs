#![allow(
    unused_crate_dependencies,
    reason = "integration target exercises one HTTP path from a workspace crate with many dependencies"
)]

#[cfg(test)]
mod tests {
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
        let _error = result.expect_err(constants_str::DIAGNOSTIC_F3A9108B);
    }
}
