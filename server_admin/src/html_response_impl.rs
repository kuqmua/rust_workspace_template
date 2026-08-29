pub(crate) fn html_response_impl(
    html: server_admin_frontend::admin_ssr_html::AdminSsrHtml,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Html(String::from(html)))
}
