#[frontend_contract::domain_types::route_error(AdminHtmlDeleteUserError)]
pub(crate) async fn delete_user(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::UserIdForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    }
    crate::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            crate::user_mutations_delete::user_mutations_delete(
                auth,
                crate::AxumAdminPath(crate::user_path_impl::user_path_impl(form.user_id)),
            )
        },
    )
    .await
}
