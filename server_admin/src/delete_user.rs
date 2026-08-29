#[frontend_contract_macros::route_error(AdminHtmlDeleteUserError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn delete_user(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::user_id_form::UserIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    }
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
        |auth| {
            crate::user_mutations_delete::user_mutations_delete(
                auth,
                crate::axum_admin_path::AxumAdminPath(crate::user_path_impl::user_path_impl(
                    form.user_id,
                )),
            )
        },
    )
    .await
}
