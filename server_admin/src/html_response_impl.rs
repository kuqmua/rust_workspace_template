pub(crate) fn html_response_impl(
    admin_ssr_html: frontend_admin::admin_ssr_html::AdminSsrHtml,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Html(String::from(admin_ssr_html)))
}
