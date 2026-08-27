#[frontend_contract::domain_types::route_error(AdminHtmlUpdateUserError)]
pub(super) async fn update_user(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::UpdateUserForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::super::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Csrf,
        );
    };
    let request = server_admin_contract::domain_types::AdminUpdateUserReq::new(
        Some(form.display_name),
        Some(form.login),
    );
    super::super::super::action_result_impl::action_result_impl(
        super::super::super::super::users::user_mutations_update::user_mutations_update(
            auth,
            super::super::super::super::AxumAdminPath(
                super::super::super::user_path_impl::user_path_impl(form.user_id),
            ),
            super::super::super::super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}
