#![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private
#[derive(newtype::AsRefOwned, newtype::FromInner)]
pub struct SecrecyAdminString(pub(super) secrecy::SecretBox<String>);
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
pub struct StdAdminString(pub(super) String);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, newtype::AsRefInner, newtype::FromInner,
)]
pub struct StdAdminStrRef<'value_lt>(pub(super) &'value_lt str);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::FromInner,
)]
pub struct StdAdminBool(pub(super) bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner)]
pub struct StdAdminNonZeroUsize(pub(super) std::num::NonZeroUsize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::FromInner,
)]
pub struct UuidAdminValue(pub(super) uuid::Uuid);
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
pub struct StdAdminSocketAddr(pub(super) std::net::SocketAddr);
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
    newtype::FromInner,
)]
pub struct AdminUserId(pub(super) i64);
impl std::fmt::Display for AdminUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
pub struct AdminRoleId(pub(super) i64);
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
pub struct AdminPermissionId(pub(super) i64);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, newtype::FromInner,
)]
pub struct AdminAuditLogId(pub(super) i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AdminPermissionName(pub(super) super::AdminPermission);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[serde(try_from = "String")]
#[bounded_string(
    max = server_admin_contract::ADMIN_LOGIN_MAX_CHARS,
    min = server_admin_contract::ADMIN_LOGIN_MIN_CHARS,
    chars,
    description = "administrator login",
    utoipa,
    validator = server_admin_contract::admin_login_is_valid
)]
pub struct AdminLogin(pub(super) String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[serde(try_from = "String")]
#[bounded_string(
    max = server_admin_contract::ADMIN_DISPLAY_NAME_MAX_CHARS,
    min = server_admin_contract::ADMIN_DISPLAY_NAME_MIN_CHARS,
    chars,
    description = "administrator display name",
    utoipa,
    validator = server_admin_contract::admin_display_name_is_valid
)]
pub struct AdminDisplayName(pub(super) String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[serde(try_from = "String")]
#[bounded_string(
    max = server_admin_contract::ADMIN_ROLE_NAME_MAX_CHARS,
    min = server_admin_contract::ADMIN_ROLE_NAME_MIN_CHARS,
    chars,
    description = "administrator role name",
    utoipa,
    validator = server_admin_contract::admin_role_name_is_valid
)]
pub struct AdminRoleName(pub(super) String);
