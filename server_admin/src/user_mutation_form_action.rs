pub(crate) async fn user_mutation_form_action(
    auth: crate::admin_auth_req::AdminAuthReq,
    target: crate::user_mutation_form_target::UserMutationFormTarget,
) -> axum::response::Response {
    match target {
        crate::user_mutation_form_target::UserMutationFormTarget::Ban(form) => {
            crate::authenticated_action_impl::authenticated_action_impl(
                auth,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                |auth| {
                    crate::mutations_set_ban::mutations_set_ban(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::user_path_impl::user_path_impl(*form.get_user_id()),
                        ),
                        crate::axum_admin_json::AxumAdminJson::from(
                            server_admin_contract::admin_set_user_ban_req::AdminSetUserBanReq::new(
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
                auth,
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                |auth| {
                    crate::mutations_set_password::mutations_set_password(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::user_path_impl::user_path_impl(*form.get_user_id()),
                        ),
                        crate::axum_admin_json::AxumAdminJson::from(
                            server_admin_contract::admin_set_user_password_req::AdminSetUserPasswordReq::new(
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
