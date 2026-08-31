#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(generate_accessor::Getters)]
pub(crate) struct UserIdForm {
    user_id: server_admin_contract::admin_user_id::AdminUserId,
    confirmation: server_admin_contract::admin_bool::AdminBool,
}
