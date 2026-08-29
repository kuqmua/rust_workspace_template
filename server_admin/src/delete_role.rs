#[frontend_contract_macros::route_error(AdminHtmlDeleteRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn delete_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    crate::axum_admin_form::AxumAdminForm(form): crate::axum_admin_form::AxumAdminForm<
        crate::role_id_form::RoleIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    }
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
        |auth| {
            crate::role_mutations_delete::role_mutations_delete(
                auth,
                crate::axum_admin_path::AxumAdminPath(crate::role_path_impl::role_path_impl(
                    form.role_id,
                )),
            )
        },
    )
    .await
}
