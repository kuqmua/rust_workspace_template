pub(crate) async fn delete_confirmed_entity(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    confirmed_delete_target: crate::confirmed_delete_target::ConfirmedDeleteTarget,
) -> axum::response::Response {
    match confirmed_delete_target {
        crate::confirmed_delete_target::ConfirmedDeleteTarget::Role(form) => {
            crate::confirmed_authenticated_action_impl::confirmed_authenticated_action_impl(
                admin_auth_request,
                *form.get_confirmation(),
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
                |auth| {
                    crate::role_mutations_delete::role_mutations_delete(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::role_path_impl::role_path_impl(*form.get_role_id()),
                        ),
                    )
                },
            )
            .await
        }
        crate::confirmed_delete_target::ConfirmedDeleteTarget::User(form) => {
            crate::confirmed_authenticated_action_impl::confirmed_authenticated_action_impl(
                admin_auth_request,
                *form.get_confirmation(),
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
                |auth| {
                    crate::user_mutations_delete::user_mutations_delete(
                        auth,
                        crate::axum_admin_path::AxumAdminPath::from(
                            crate::user_path_impl::user_path_impl(*form.get_user_id()),
                        ),
                    )
                },
            )
            .await
        }
    }
}
