#[frontend_contract::domain_types::route_error(AdminHtmlCreateUserError)]
pub(super) async fn create_user(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::CreateUserForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::super::super::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminCreateUserReq::new(
        form.display_name,
        form.login,
        form.password,
    );
    super::super::action_result(
        super::super::super::users::mutations::create(
            auth,
            super::super::super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateUserError)]
pub(super) async fn update_user(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UpdateUserForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::super::super::AdminError::Csrf);
    };
    let request = server_admin_contract::domain_types::AdminUpdateUserReq::new(
        Some(form.display_name),
        Some(form.login),
    );
    super::super::action_result(
        super::super::super::users::mutations::update(
            auth,
            super::super::super::AxumAdminPath(super::super::user_path(form.user_id)),
            super::super::super::AxumAdminJson(request),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserPasswordError)]
pub(super) async fn user_password(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UserPasswordForm,
    >,
) -> axum::response::Response {
    super::super::authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::super::super::users::mutations::set_password(
                auth,
                super::super::super::AxumAdminPath(super::super::user_path(form.user_id)),
                super::super::super::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserPasswordReq::new(
                        form.password,
                    ),
                ),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserBanError)]
pub(super) async fn user_ban(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UserBanForm,
    >,
) -> axum::response::Response {
    super::super::authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::super::super::users::mutations::set_ban(
                auth,
                super::super::super::AxumAdminPath(super::super::user_path(form.user_id)),
                super::super::super::AxumAdminJson(
                    server_admin_contract::domain_types::AdminSetUserBanReq::new(form.is_banned),
                ),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlDeleteUserError)]
pub(super) async fn delete_user(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UserIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::AdminError::Validation,
        );
    }
    super::super::authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        |auth| {
            super::super::super::users::mutations::delete(
                auth,
                super::super::super::AxumAdminPath(super::super::user_path(form.user_id)),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlUserRolesError)]
pub(super) async fn user_roles(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UserRolesForm,
    >,
) -> axum::response::Response {
    super::assignment_action(
        auth,
        &form.expected_role_ids,
        form.selected,
        super::super::role_ids,
        server_admin_contract::domain_types::AdminFrontendPath::Users,
        server_admin_contract::domain_types::AdminSetUserRolesReq::new,
        super::super::super::AxumAdminPath(super::super::user_path(form.user_id)),
        super::super::super::users::mutations::set_roles,
    )
    .await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::UserCreate, create_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserUpdate, update_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserPassword, user_password),
    (server_admin_contract::domain_types::AdminHtmlAction::UserBan, user_ban),
    (server_admin_contract::domain_types::AdminHtmlAction::UserDelete, delete_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserRoles, user_roles),
)]
struct AdminHtmlUserActionRouteRegistry;

pub(super) fn router() -> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlUserActionRouteRegistry::router())
}
