#[frontend_contract_macros::route_error(AdminHtmlUpdateUserError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn update_user(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::update_user_form::UpdateUserForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    let request = server_admin_contract::admin_update_user_req::AdminUpdateUserReq::new(
        Some(form.display_name),
        Some(form.login),
    );
    crate::action_result_impl::action_result_impl(
        crate::user_mutations_update::user_mutations_update(
            auth,
            crate::axum_admin_path::AxumAdminPath(crate::user_path_impl::user_path_impl(
                form.user_id,
            )),
            crate::axum_admin_json::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
    )
}
