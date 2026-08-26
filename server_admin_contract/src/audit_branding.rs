#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator audit timestamp"
)]
pub struct AdminAuditTimestamp(String);
pub const ADMIN_AUDIT_DETAILS_MAX_BYTES: usize = 4096usize;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    newtype::FromInner,
)]
pub struct AdminAuditDetailsBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(
    "administrator audit details contain {} bytes, maximum is {} bytes",
    .0.0,
    ADMIN_AUDIT_DETAILS_MAX_BYTES
)]
pub struct AdminAuditDetailsTooLarge(AdminAuditDetailsBytes);
impl AdminAuditDetailsTooLarge {
    #[must_use]
    pub const fn actual_bytes(self) -> AdminAuditDetailsBytes {
        self.0
    }
    #[must_use]
    pub fn maximum_bytes(self) -> AdminAuditDetailsBytes {
        AdminAuditDetailsBytes::from(ADMIN_AUDIT_DETAILS_MAX_BYTES)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);
impl TryFrom<serde_json::Value> for SerdeJsonAdminAuditDetails {
    type Error = AdminAuditDetailsTooLarge;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let actual_bytes = value.to_string().len();
        if actual_bytes > ADMIN_AUDIT_DETAILS_MAX_BYTES {
            return Err(AdminAuditDetailsTooLarge(AdminAuditDetailsBytes::from(
                actual_bytes,
            )));
        }
        Ok(Self(value))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars, serde, utoipa, validator = |value: &String| { let path = super::AdminPagePathRef::from(value.as_str()); super::AdminPage::from_path(path).is_some() || super::AdminDataTable::from_frontend_path(path).is_some() }, description = "administrator default route")]
pub struct AdminDefaultRoute(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, min = constants_usize::ONE, chars, serde, utoipa, validator = |value: &String| !value
    .trim()
    .is_empty(), description = "administrator site name")]
pub struct AdminSiteName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator main logo"
)]
pub struct AdminMainLogo(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization contacts"
)]
pub struct AdminOrganizationContacts(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    chars,
    serde,
    utoipa,
    description = "administrator organization name"
)]
pub struct AdminOrganizationName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = 7usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.len() == 7usize && value.bytes().next() == Some(b'#') && value.bytes().skip(constants_usize::ONE).all(|byte| byte.is_ascii_hexdigit()),
    description = "administrator primary color"
)]
pub struct AdminPrimaryColor(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator support URL"
)]
pub struct AdminSupportUrl(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| !value.trim().is_empty(),
    description = "administrator tab title"
)]
pub struct AdminTabTitle(String);

#[cfg(test)]
mod tests {
    #[test]
    fn audit_detail_limit_is_stable() {
        assert_eq!(
            super::AdminAuditDetailsTooLarge::from(super::AdminAuditDetailsBytes::from(
                constants_usize::ONE,
            ))
            .maximum_bytes(),
            super::AdminAuditDetailsBytes::from(super::ADMIN_AUDIT_DETAILS_MAX_BYTES),
        );
    }
}
