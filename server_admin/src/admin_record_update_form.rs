#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminRecordUpdateForm {
    User {
        admin_user_id: server_admin_contract::admin_user_id::AdminUserId,
        axum_admin_form:
            crate::axum_admin_form::AxumAdminForm<crate::update_user_form::UpdateUserForm>,
    },
    Role {
        admin_role_id: server_admin_contract::admin_role_id::AdminRoleId,
        axum_admin_form:
            crate::axum_admin_form::AxumAdminForm<crate::update_role_form::UpdateRoleForm>,
    },
}
