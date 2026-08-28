#[frontend_contract::domain_types::route_error(AdminHtmlDeleteRoleError)]
pub(super) async fn delete_role(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::RoleIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
    }
    super::super::super::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        |auth| {
            super::super::super::super::roles::role_mutations_delete::role_mutations_delete(
                auth,
                super::super::super::super::AxumAdminPath(
                    super::super::super::role_path_impl::role_path_impl(form.role_id),
                ),
            )
        },
    )
    .await
}
