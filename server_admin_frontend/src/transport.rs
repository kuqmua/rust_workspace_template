#![allow(clippy::future_not_send)] // browser transport runs on the single-threaded WASM executor
#[derive(Clone, Copy, Debug)]
pub(super) struct GlooTransport;
#[derive(Clone, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct GlooNetHttpMethod(gloo_net::http::Method);
#[derive(Clone, Debug, newtype::BoundedString, newtype::AsRefTarget)]
#[bounded_string(max = 4096usize, min = 1usize)]
struct BrowserCsrfToken(String);
impl frontend_contract::Transport for GlooTransport {
    fn send(
        &self,
        request: frontend_contract::TransportRequest,
    ) -> std::pin::Pin<
        Box<
            dyn Future<
                    Output = Result<
                        frontend_contract::TransportResponse,
                        frontend_contract::TransportError,
                    >,
                > + '_,
        >,
    > {
        Box::pin(async move {
            let body = String::from_utf8(request.body().as_ref().to_vec()).map_err(|error| {
                frontend_contract::TransportError::try_from(error.to_string()).unwrap_or_default()
            })?;
            let route = request.route();
            let mut builder = gloo_net::http::RequestBuilder::new(request.path().as_ref())
                .method(gloo_net::http::Method::from(http_method(route.method())))
                .credentials(web_sys::RequestCredentials::Include)
                .header(str_constants::CONTENT_TYPE, str_constants::APPLICATION_JSON)
                .header(
                    str_constants::ROUTE_VALIDATORS_COMMIT_HEADER_NAME,
                    git_info::PROJECT_GIT_INFO.commit.as_ref(),
                );
            if let Some(idempotency_key) = request.idempotency_key() {
                builder = builder.header(str_constants::IDEMPOTENCY_KEY, idempotency_key.as_ref());
            }
            if let Some(if_match) = request.if_match() {
                builder = builder.header(str_constants::IF_MATCH, if_match.as_ref());
            }
            if route.mutation() == frontend_contract::MutationKind::Mutating
                && let Some(token) = csrf_token()
            {
                builder = builder.header(str_constants::X_CSRF_TOKEN, token.as_ref());
            }
            let outbound = if route.method() == frontend_contract::HttpMethod::Get {
                builder.build()
            } else {
                builder.body(body)
            }
            .map_err(|error| {
                frontend_contract::TransportError::try_from(error.to_string()).unwrap_or_default()
            })?;
            let response = outbound.send().await.map_err(|error| {
                frontend_contract::TransportError::try_from(error.to_string()).unwrap_or_default()
            })?;
            let status = frontend_contract::TransportStatus::from(response.status());
            let retry_after = response
                .headers()
                .get(str_constants::RETRY_AFTER)
                .and_then(|value| frontend_contract::TransportRetryAfter::try_from(value).ok());
            response
                .binary()
                .await
                .map(frontend_contract::TransportBody::from)
                .map(|body| {
                    frontend_contract::TransportResponse::new(body, status)
                        .with_retry_after(retry_after)
                })
                .map_err(|error| {
                    frontend_contract::TransportError::try_from(error.to_string())
                        .unwrap_or_default()
                })
        })
    }
}
fn http_method(method: frontend_contract::HttpMethod) -> GlooNetHttpMethod {
    GlooNetHttpMethod::from(match method {
        frontend_contract::HttpMethod::Connect => gloo_net::http::Method::CONNECT,
        frontend_contract::HttpMethod::Delete => gloo_net::http::Method::DELETE,
        frontend_contract::HttpMethod::Get => gloo_net::http::Method::GET,
        frontend_contract::HttpMethod::Head => gloo_net::http::Method::HEAD,
        frontend_contract::HttpMethod::Options => gloo_net::http::Method::OPTIONS,
        frontend_contract::HttpMethod::Patch => gloo_net::http::Method::PATCH,
        frontend_contract::HttpMethod::Post => gloo_net::http::Method::POST,
        frontend_contract::HttpMethod::Put => gloo_net::http::Method::PUT,
        frontend_contract::HttpMethod::Trace => gloo_net::http::Method::TRACE,
    })
}
fn csrf_token() -> Option<BrowserCsrfToken> {
    let document =
        wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlDocument>(web_sys::window()?.document()?)
            .ok()?;
    let cookies = document.cookie().ok()?;
    cookies.split(';').map(str::trim).find_map(|cookie| {
        cookie
            .strip_prefix(str_constants::ADMIN_CSRF_TOKEN_ALT)
            .and_then(|value| BrowserCsrfToken::try_from(value.to_owned()).ok())
    })
}
