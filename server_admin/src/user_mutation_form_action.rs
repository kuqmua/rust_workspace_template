pub(crate) async fn user_mutation_form_action(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    user_mutation_form_target: crate::user_mutation_form_target::UserMutationFormTarget,
) -> axum::response::Response {
    match user_mutation_form_target {
        crate::user_mutation_form_target::UserMutationFormTarget::Ban(form) => {
            crate::authenticated_action_impl::authenticated_action_impl(
                admin_auth_request,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                |auth| {
                    crate::mutations_set_ban::mutations_set_ban(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::user_path_impl::user_path_impl(*form.get_user_id()),
                        ),
                        crate::axum_admin_json::AxumAdminJson::from(
                            server_admin_contract::admin_set_user_ban_request::AdminSetUserBanRequest::new(
                                *form.get_is_banned(),
                            ),
                        ),
                    )
                },
            )
            .await
        }
        crate::user_mutation_form_target::UserMutationFormTarget::Password(form) => {
            crate::authenticated_action_impl::authenticated_action_impl(
                admin_auth_request,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                |auth| {
                    crate::mutations_set_password::mutations_set_password(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::user_path_impl::user_path_impl(*form.get_user_id()),
                        ),
                        crate::axum_admin_json::AxumAdminJson::from(
                            server_admin_contract::admin_set_user_password_request::AdminSetUserPasswordRequest::new(
                                form.get_password().clone(),
                            ),
                        ),
                    )
                },
            )
            .await
        }
    }
}
