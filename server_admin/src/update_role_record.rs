#[proc_macro_frontend_contract_route_error::route_error(AdminHtmlUpdateRoleRecordError)]
#[allow(
    clippy::single_call_fn,
    reason = "typed frontend route registration requires a named endpoint function"
)]
pub(crate) async fn update_role_record(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_contract::admin_role_id::AdminRoleId,
    >,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::update_role_form::UpdateRoleForm>,
) -> crate::axum_admin_response::AxumAdminResponse {
    crate::update_record_form::update_record_form(
        admin_auth_request,
        crate::admin_record_update_form::AdminRecordUpdateForm::Role {
            admin_role_id: axum_admin_path.into_inner(),
            axum_admin_form,
        },
    )
    .await
}
