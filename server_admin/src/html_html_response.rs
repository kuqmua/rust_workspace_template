pub(super) fn html_response_impl(
    html: server_admin_frontend::domain_types::ssr::AdminSsrHtml,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Html(String::from(html)))
}
