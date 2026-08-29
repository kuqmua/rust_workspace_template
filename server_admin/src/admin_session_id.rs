#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "server_admin_core::uuid_admin_value::UuidAdminValue")]
pub struct AdminSessionId(pub(crate) server_admin_core::uuid_admin_value::UuidAdminValue);
