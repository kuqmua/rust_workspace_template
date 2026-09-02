#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(proc_macro_getters::Getters)]
pub(crate) struct AdminSignInUser {
    id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    password_hash: crate::admin_password_hash::AdminPasswordHash,
    is_banned: server_admin_core::std_admin_bool::StdAdminBool,
}

impl TryFrom<(i64, String, bool)> for AdminSignInUser {
    type Error = crate::sqlx_admin_error::SqlxAdminError;

    fn try_from(value: (i64, String, bool)) -> Result<Self, Self::Error> {
        let (id, password_hash, is_banned) = value;
        Ok(Self {
            id: server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(id)?,
            password_hash: crate::admin_password_hash::AdminPasswordHash::new(
                pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret::from(
                    password_hash,
                ),
            ),
            is_banned: server_admin_core::std_admin_bool::StdAdminBool::from(is_banned),
        })
    }
}

impl From<AdminSignInUser>
    for (
        server_admin_core::admin_user_record_id::AdminUserRecordId,
        crate::admin_password_hash::AdminPasswordHash,
        server_admin_core::std_admin_bool::StdAdminBool,
    )
{
    fn from(value: AdminSignInUser) -> Self {
        let id = *value.get_id();
        let is_banned = *value.get_is_banned();
        (id, value.password_hash, is_banned)
    }
}
