#[frontend_contract::domain_types::route_error(AdminHtmlDeleteUserError)]
pub(super) async fn delete_user(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::UserIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
    }
    super::super::super::authenticated_action_impl::authenticated_action_impl(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::super::super::super::users::user_mutations_delete::user_mutations_delete(
                auth,
                super::super::super::super::AxumAdminPath(
                    super::super::super::user_path_impl::user_path_impl(form.user_id),
                ),
            )
        },
    )
    .await
}
