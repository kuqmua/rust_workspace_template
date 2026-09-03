#[proc_macro_frontend_contract::route_error(AdminHtmlDeleteRoleError)]
#[allow(
    clippy::single_call_fn,
    reason = "delete role remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn delete_role(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<crate::role_id_form::RoleIdForm>,
) -> axum::response::Response {
    crate::delete_confirmed_entity::delete_confirmed_entity(
        admin_auth_request,
        crate::confirmed_delete_target::ConfirmedDeleteTarget::Role(axum_admin_form.into_inner()),
    )
    .await
}
