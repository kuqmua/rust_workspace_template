#[proc_macro_frontend_contract::route_error(AdminHtmlDeleteRoleError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn delete_role(
    auth: crate::admin_auth_req::AdminAuthReq,
    form: crate::axum_admin_form::AxumAdminForm<crate::role_id_form::RoleIdForm>,
) -> axum::response::Response {
    crate::delete_confirmed_entity::delete_confirmed_entity(
        auth,
        crate::confirmed_delete_target::ConfirmedDeleteTarget::Role(form.into_inner()),
    )
    .await
}
