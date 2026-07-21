#![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private
#[derive(newtype::AsRefOwned)]
pub struct SecrecyAdminString(secrecy::SecretBox<String>);
impl From<secrecy::SecretBox<String>> for SecrecyAdminString {
    fn from(value: secrecy::SecretBox<String>) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
pub struct StdAdminString(String);
pub(super) enum AdminAuditResourceValue {
    Role(AdminRoleId),
    Session(crate::AdminSessionId),
    SystemSettings,
    User(AdminUserId),
}
impl From<AdminAuditResourceValue> for StdAdminString {
    fn from(value: AdminAuditResourceValue) -> Self {
        Self(match value {
            AdminAuditResourceValue::Role(role) => role.get().to_string(),
            AdminAuditResourceValue::Session(session) => session.0.get().to_string(),
            AdminAuditResourceValue::SystemSettings => str_constants::VALUE_1.to_owned(),
            AdminAuditResourceValue::User(user) => user.get().to_string(),
        })
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, newtype::AsRefInner, newtype::FromInner,
)]
pub struct StdAdminStrRef<'value_lt>(&'value_lt str);
impl<'value_lt> StdAdminStrRef<'value_lt> {
    #[must_use]
    pub const fn get(self) -> &'value_lt str {
        self.0
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::FromInner,
)]
#[serde(from = "bool")]
pub struct StdAdminBool(bool);
impl StdAdminBool {
    #[must_use]
    pub const fn get(self) -> bool {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner)]
pub struct StdAdminNonZeroUsize(std::num::NonZeroUsize);
impl StdAdminNonZeroUsize {
    #[must_use]
    pub const fn get(self) -> std::num::NonZeroUsize {
        self.0
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::FromInner,
)]
#[serde(from = "uuid::Uuid")]
pub struct UuidAdminValue(uuid::Uuid);
impl UuidAdminValue {
    #[must_use]
    pub const fn get(self) -> uuid::Uuid {
        self.0
    }
}
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for UuidAdminValue {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            stringify!(UuidAdminValue),
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .format(Some(utoipa::openapi::SchemaFormat::Custom(
                    str_constants::PG_CRUD_PG_UUID.to_owned(),
                )))
                .into(),
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::AsRefOwned, newtype::FromInner)]
pub struct StdAdminSocketAddr(std::net::SocketAddr);
impl StdAdminSocketAddr {
    #[must_use]
    pub const fn get(self) -> std::net::SocketAddr {
        self.0
    }
}
#[derive(
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
#[serde(from = "i64")]
pub struct AdminUserId(i64);
impl AdminUserId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(
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
#[serde(from = "i64")]
pub struct AdminRoleId(i64);
impl AdminRoleId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(
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
#[serde(from = "i64")]
pub struct AdminPermissionId(i64);
impl AdminPermissionId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, newtype::FromInner,
)]
pub struct AdminAuditLogId(i64);
impl AdminAuditLogId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AdminPermissionName(super::AdminPermission);
