#[proc_macro_frontend_contract::route_error(AdminProfilePageError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn profile(auth: crate::admin_auth_req::AdminAuthReq) -> axum::response::Response {
    crate::csr_page::csr_page(
        auth,
        server_admin_contract::admin_page::AdminPage::Profile,
        None,
    )
    .await
}
