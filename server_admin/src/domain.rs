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
    newtype::Display,
    newtype::FromInner,
)]
pub struct AdminUserId(pub(super) i64);
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
