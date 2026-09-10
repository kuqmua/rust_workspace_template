pub(crate) async fn user_mutations_update(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_path: crate::axum_admin_path::AxumAdminPath<
        server_admin_core::admin_user_record_id::AdminUserRecordId,
    >,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_user_request::AdminUpdateUserRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let filter = server_admin_contract::admin_user_filter::AdminUserFilter::new(
        Some(server_admin_contract::admin_user_id::AdminUserId::from(
            axum_admin_path.get_inner().value(),
        )),
        None,
        None,
        None,
    );
    let updates = [
        server_admin_contract::admin_user_update::AdminUserUpdate::new(
            axum_admin_json.into_inner(),
            filter,
        ),
    ];
    crate::user_mutations_update_filtered::user_mutations_update_filtered(
        admin_auth_request,
        crate::admin_user_update_slice::AdminUserUpdateSlice::from(updates.as_slice()),
    )
    .await
}
