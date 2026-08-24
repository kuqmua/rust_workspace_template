#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminNoBody;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator session identifier"
)]
pub struct AdminSessionIdentifier(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 64,
    chars,
    serde,
    utoipa,
    description = "administrator session timestamp"
)]
pub struct AdminSessionTimestamp(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminSessionView {
    created_at: AdminSessionTimestamp,
    expires_at: AdminSessionTimestamp,
    id: AdminSessionIdentifier,
    #[serde(default)]
    is_current: super::AdminBool,
}
#[cfg(test)]
mod tests;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminSessionsPage {
    items: super::AdminSessionViews,
    #[schema(value_type = u64)]
    total: super::AdminPageTotal,
}
impl AdminSessionsPage {
    #[must_use]
    pub const fn new(items: super::AdminSessionViews, total: super::AdminPageTotal) -> Self {
        Self { items, total }
    }
    #[must_use]
    pub const fn items(&self) -> &[AdminSessionView] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> super::AdminPageTotal {
        self.total
    }
}
impl AdminSessionView {
    #[must_use]
    pub const fn new(
        created_at: AdminSessionTimestamp,
        expires_at: AdminSessionTimestamp,
        id: AdminSessionIdentifier,
        is_current: super::AdminBool,
    ) -> Self {
        Self {
            created_at,
            expires_at,
            id,
            is_current,
        }
    }
    #[must_use]
    pub const fn created_at(&self) -> &AdminSessionTimestamp {
        &self.created_at
    }
    #[must_use]
    pub const fn expires_at(&self) -> &AdminSessionTimestamp {
        &self.expires_at
    }
    #[must_use]
    pub const fn id(&self) -> &AdminSessionIdentifier {
        &self.id
    }
    #[must_use]
    pub const fn is_current(&self) -> super::AdminBool {
        self.is_current
    }
}
