#![allow(clippy::single_call_fn)] // route inventory registers focused role operations once

pub(super) async fn create(
    auth: super::AdminAuthReq,
    request: super::AxumAdminJson<server_admin_contract::AdminCreateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesCreate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let role_id = super::super::repository::roles::insert_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &name,
    )
    .await
    .map_err(|error| super::shared::map_unique_violation(error.0))?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(role_id),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(server_admin_contract::AdminCreateRoleRes::new(
                server_admin_contract::AdminRoleId::from(role_id.value()),
            )),
        )),
    ))
}
pub(super) async fn update(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminUpdateRoleReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesUpdate).await?;
    let name = super::super::AdminRoleName::try_from(request.0.into_name().into_inner())
        .map_err(|_error| super::AdminError::Validation)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::update_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        &name,
    )
    .await
    .map_err(|error| super::shared::map_unique_violation(error.0))?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn delete(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor = super::authorize_custom(&auth, super::super::AdminPermission::RolesDelete).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    super::super::repository::roles::delete_role(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
    )
    .await
    .map_err(super::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(super::AdminError::Conflict)?;
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn set_permissions(
    auth: super::AdminAuthReq,
    path: super::AxumAdminPath<super::super::AdminRoleId>,
    request: super::AxumAdminJson<server_admin_contract::AdminSetRolePermissionsReq>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    let actor =
        super::authorize_custom(&auth, super::super::AdminPermission::RolePermissionsUpdate)
            .await?;
    let (expected_permission_ids, contract_permission_ids) = request.0.into_parts();
    if AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&expected_permission_ids)
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&expected_permission_ids)
            .len()
        || AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&contract_permission_ids)
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != AsRef::<[server_admin_contract::AdminPermissionId]>::as_ref(&contract_permission_ids)
                .len()
    {
        return Err(super::AdminError::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(super::AdminError::from)?;
    let outcome = super::super::repository::permissions::replace_role_permissions(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        path.0,
        expected_permission_ids.as_ref(),
        contract_permission_ids.as_ref(),
    )
    .await
    .map_err(super::AdminError::from)?;
    match outcome {
        super::super::repository::ReplaceRolePermissionsOutcome::Updated => {}
        super::super::repository::ReplaceRolePermissionsOutcome::UnknownPermission => {
            return Err(super::AdminError::Validation);
        }
        super::super::repository::ReplaceRolePermissionsOutcome::MissingRole
        | super::super::repository::ReplaceRolePermissionsOutcome::StaleAssignment
        | super::super::repository::ReplaceRolePermissionsOutcome::SystemRole => {
            return Err(super::AdminError::Conflict);
        }
    }
    super::record_audit_success_in_connection(
        super::SqlxAdminPgConnectionRef::from(&mut *tx),
        super::AdminAuditSuccessRef {
            action: super::super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::super::AdminAuditResource::Role,
            resource_id: super::AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit().await.map_err(super::AdminError::from)?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
pub(super) async fn roles_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminRolesPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::RolesRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::AdminTableSortField::ROLE,
    )?;
    let pool = super::super::repository::SqlxAdminRepositoryPoolRef::from(
        auth.state.as_ref().pool.as_ref(),
    );
    let (roles, total) = super::super::repository::roles::list_roles(pool, &query.0)
        .await
        .map_err(super::shared::map_repository_error)?;
    let permissions = super::super::repository::permissions::list_permission_catalog(pool)
        .await
        .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::AdminRolesPage::new(
        roles,
        permissions,
        super::shared::page_total(total)?,
    ))
}
pub(super) async fn list(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    roles_page(auth, query).await.map(|page| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            page,
        )))
    })
}
pub(super) async fn permissions_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<server_admin_contract::AdminPermissionsPage, super::AdminError> {
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::PermissionsRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    super::shared::validate_table_sort(
        &query.0,
        &server_admin_contract::AdminTableSortField::PERMISSION,
    )?;
    let (permissions, total) = super::super::repository::permissions::list_permissions(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        &query.0,
    )
    .await
    .map_err(super::shared::map_repository_error)?;
    Ok(server_admin_contract::AdminPermissionsPage::new(
        permissions,
        super::shared::page_total(total)?,
    ))
}
pub(super) async fn list_permissions(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    permissions_page(auth, query).await.map(|page| {
        super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            page,
        )))
    })
}
