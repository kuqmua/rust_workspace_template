#[frontend_contract::domain_types::route_error(AdminHtmlCreateRoleError)]
pub(super) async fn create_role(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::CreateRoleForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::super::super::AdminError::Csrf);
    };
    super::super::action_result(
        super::super::super::roles::mutations::create(
            auth,
            super::super::super::AxumAdminJson(
                server_admin_contract::domain_types::AdminCreateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlUpdateRoleError)]
pub(super) async fn update_role(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::UpdateRoleForm,
    >,
) -> axum::response::Response {
    let Ok(auth) = super::super::form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::super::super::AdminError::Csrf);
    };
    super::super::action_result(
        super::super::super::roles::mutations::update(
            auth,
            super::super::super::AxumAdminPath(super::super::role_path(form.role_id)),
            super::super::super::AxumAdminJson(
                server_admin_contract::domain_types::AdminUpdateRoleReq::new(form.name),
            ),
        )
        .await,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
    )
}

#[frontend_contract::domain_types::route_error(AdminHtmlDeleteRoleError)]
pub(super) async fn delete_role(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::RoleIdForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::AdminError::Validation,
        );
    }
    super::super::authenticated_action(
        auth,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        |auth| {
            super::super::super::roles::mutations::delete(
                auth,
                super::super::super::AxumAdminPath(super::super::role_path(form.role_id)),
            )
        },
    )
    .await
}

#[frontend_contract::domain_types::route_error(AdminHtmlRolePermissionsError)]
pub(super) async fn role_permissions(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::RolePermissionsForm,
    >,
) -> axum::response::Response {
    super::assignment_action(
        auth,
        &form.expected_permission_ids,
        form.selected,
        super::super::permission_ids,
        server_admin_contract::domain_types::AdminFrontendPath::Roles,
        server_admin_contract::domain_types::AdminSetRolePermissionsReq::new,
        super::super::super::AxumAdminPath(super::super::role_path(form.role_id)),
        super::super::super::roles::mutations::set_permissions,
    )
    .await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::handler_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::RoleCreate, create_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate, update_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleDelete, delete_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RolePermissions, role_permissions),
)]
struct AdminHtmlRoleActionRouteRegistry;

pub(super) fn router() -> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlRoleActionRouteRegistry::router())
}
