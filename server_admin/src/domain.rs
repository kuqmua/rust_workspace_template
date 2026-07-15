#![allow(clippy::field_scoped_visibility_modifiers)] // sibling domain modules require raw representations while facade reexports must keep fields externally private
#[derive(newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SecrecyAdminString(pub(super) secrecy::SecretBox<String>);
impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::text::REDACTED_ALT_3)
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
    newtype::Newtype,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
#[newtype(as_ref_owned, into_inner)]
pub struct StdAdminString(pub(super) String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct StdAdminStrRef<'value_lt>(pub(super) &'value_lt str);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct StdAdminBool(pub(super) bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(deref_inner, from_inner)]
pub struct StdAdminNonZeroUsize(pub(super) std::num::NonZeroUsize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct UuidAdminValue(pub(super) uuid::Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
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
    newtype::Newtype,
)]
#[newtype(from_inner)]
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
    newtype::Newtype,
)]
#[newtype(from_inner)]
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
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminPermissionId(pub(super) i64);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminAuditLogId(pub(super) i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminPermissionName(pub(super) super::AdminPermission);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 128, chars, description = "administrator login", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminLogin(pub(super) String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 256, chars, description = "administrator display name", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminDisplayName(pub(super) String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 128, chars, description = "administrator role name", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminRoleName(pub(super) String);
