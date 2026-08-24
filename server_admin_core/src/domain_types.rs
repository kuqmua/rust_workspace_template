#![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefOwned, newtype::FromInner)]
pub struct SecrecyAdminString(secrecy::SecretBox<StdAdminString>);

impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
impl TryFrom<String> for SecrecyAdminString {
    type Error = StdAdminStringTryFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        StdAdminString::try_from(value)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl secrecy::ExposeSecret<StdAdminString> for SecrecyAdminString {
    fn expose_secret(&self) -> &StdAdminString {
        secrecy::ExposeSecret::expose_secret(&self.0)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
pub struct StdAdminString(String);
impl secrecy::zeroize::Zeroize for StdAdminString {
    fn zeroize(&mut self) {
        secrecy::zeroize::Zeroize::zeroize(&mut self.0);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
enum AdminResourceText {
    PositiveI64(server_admin_contract::PositiveNonZeroI64),
    SystemSettings,
    Uuid(UuidAdminValue),
}
impl From<AdminResourceText> for StdAdminString {
    fn from(resource: AdminResourceText) -> Self {
        Self(match resource {
            AdminResourceText::PositiveI64(value) => value.get().to_string(),
            AdminResourceText::SystemSettings => constants_str::VALUE_1.to_owned(),
            AdminResourceText::Uuid(value) => value.get().to_string(),
        })
    }
}
impl StdAdminString {
    #[must_use]
    pub fn from_positive_i64(value: server_admin_contract::PositiveNonZeroI64) -> Self {
        Self::from(AdminResourceText::PositiveI64(value))
    }

    #[must_use]
    pub fn from_uuid(value: UuidAdminValue) -> Self {
        Self::from(AdminResourceText::Uuid(value))
    }

    #[must_use]
    pub fn system_settings_resource() -> Self {
        Self::from(AdminResourceText::SystemSettings)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct StdAdminStrRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "bool")]
pub struct StdAdminBool(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminNonZeroUsize(std::num::NonZeroUsize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "uuid::Uuid")]
pub struct UuidAdminValue(uuid::Uuid);
impl utoipa::PartialSchema for UuidAdminValue {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .format(Some(utoipa::openapi::SchemaFormat::Custom(
                constants_str::PG_CRUD_PG_UUID.to_owned(),
            )))
            .into()
    }
}
impl utoipa::ToSchema for UuidAdminValue {}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminSocketAddr(std::net::SocketAddr);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
pub struct AdminUserId(server_admin_contract::PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(|_error| AdminIdTryFromI64Error)
    }
}
impl AdminUserId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
pub struct AdminRoleId(server_admin_contract::PositiveNonZeroI64);
impl TryFrom<i64> for AdminRoleId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(|_error| AdminIdTryFromI64Error)
    }
}
impl AdminRoleId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
pub struct AdminPermissionId(server_admin_contract::PositiveNonZeroI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(|_error| AdminIdTryFromI64Error)
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
pub struct AdminAuditLogId(server_admin_contract::PositiveNonZeroI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(|_error| AdminIdTryFromI64Error)
    }
}
impl AdminAuditLogId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminIdTryFromI64Error;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminPermissionName(server_admin_contract::AdminPermission);

#[cfg(test)]
mod tests {
    #[test]
    fn administrator_secret_text_enforces_internal_bound() {
        let at_limit = "a".repeat(constants_usize::VALUE_8_192);
        let secret = super::SecrecyAdminString::try_from(at_limit.clone()).expect(
            "6673b876 administrator_secret_text_enforces_internal_bound invariant must hold",
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(&secret)
                .as_ref()
                .as_str(),
            at_limit.as_str()
        );
        assert_eq!(
            super::SecrecyAdminString::try_from("a".repeat(8_193usize)).err(),
            Some(super::StdAdminStringTryFromStringError::TooLong {
                len: 8_193usize,
                max: constants_usize::VALUE_8_192,
            })
        );
    }
    #[test]
    fn administrator_secret_text_is_redacted_and_zeroizable() {
        let raw = constants_str::NEVER_PRINT_THIS_VALUE;
        let secret = super::SecrecyAdminString::try_from(raw.to_owned()).expect(
            "67b629e2 administrator_secret_text_is_redacted_and_zeroizable invariant must hold",
        );
        assert!(!format!("{secret:?}").contains(raw));
        let mut bounded = super::StdAdminString::try_from(raw.to_owned()).expect(
            "201f3c4b administrator_secret_text_is_redacted_and_zeroizable invariant must hold",
        );
        secrecy::zeroize::Zeroize::zeroize(&mut bounded);
        assert!(bounded.as_ref().is_empty());
    }
    #[test]
    fn administrator_resource_values_are_stable() {
        let positive = server_admin_contract::PositiveNonZeroI64::try_from(42i64)
            .expect("2570af3b administrator_resource_values_are_stable invariant must hold");
        assert_eq!(
            super::StdAdminString::from_positive_i64(positive).as_ref(),
            "42"
        );
        assert_eq!(
            super::StdAdminString::system_settings_resource().as_ref(),
            "1"
        );
        let uuid_value = uuid::Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0001u128);
        let expected = uuid_value.to_string();
        let uuid = super::UuidAdminValue::from(uuid_value);
        assert_eq!(
            super::StdAdminString::from_uuid(uuid).as_ref().as_str(),
            expected.as_str()
        );
    }
}
