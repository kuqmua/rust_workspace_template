#[frontend_contract::route_error(AdminHtmlUpdateUserError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn update_user(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UpdateUserForm>,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminUpdateUserReq::new(
        Some(form.display_name),
        Some(form.login),
    );
    crate::action_result_impl::action_result_impl(
        crate::user_mutations_update::user_mutations_update(
            auth,
            crate::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
            crate::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}
