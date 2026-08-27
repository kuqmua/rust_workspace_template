#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub(crate) struct AdminSignInUser {
    id: crate::domain_types::AdminUserId,
    password_hash: crate::domain_types::AdminPasswordHash,
    is_banned: crate::domain_types::StdAdminBool,
}

impl TryFrom<(i64, String, bool)> for AdminSignInUser {
    type Error = crate::domain_types::SqlxAdminError;

    fn try_from((id, password_hash, is_banned): (i64, String, bool)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: crate::domain_types::AdminUserId::try_from(id)?,
            password_hash: crate::domain_types::AdminPasswordHash::new(
                pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
            ),
            is_banned: crate::domain_types::StdAdminBool::from(is_banned),
        })
    }
}

impl From<AdminSignInUser>
    for (
        crate::domain_types::AdminUserId,
        crate::domain_types::AdminPasswordHash,
        crate::domain_types::StdAdminBool,
    )
{
    fn from(value: AdminSignInUser) -> Self {
        (value.id, value.password_hash, value.is_banned)
    }
}
