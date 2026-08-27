#[frontend_contract::domain_types::route_error(AdminHtmlUpdateRoleError)]
pub(super) async fn update_role(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::UpdateRoleForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::super::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Csrf,
        );
    };
    super::super::super::action_result_impl::action_result_impl(
        super::super::super::super::roles::role_mutations_update::role_mutations_update(
            auth,
            super::super::super::super::AxumAdminPath(
                super::super::super::role_path_impl::role_path_impl(form.role_id),
            ),
            super::super::super::super::AxumAdminJson(
                server_admin_contract::domain_types::AdminUpdateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}
