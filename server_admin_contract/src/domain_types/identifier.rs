#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminUserId(super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminUserId> for i64 {
    fn from(value: AdminUserId) -> Self {
        value.0.get()
    }
}
impl AdminUserId {
    #[must_use]
    pub const fn value(self) -> super::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminRoleId(super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminRoleId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminRoleId> for i64 {
    fn from(value: AdminRoleId) -> Self {
        value.0.get()
    }
}
impl AdminRoleId {
    #[must_use]
    pub const fn value(self) -> super::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminPermissionId(super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminPermissionId> for i64 {
    fn from(value: AdminPermissionId) -> Self {
        value.0.get()
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn value(self) -> super::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminAuditLogId(super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminAuditLogId> for i64 {
    fn from(value: AdminAuditLogId) -> Self {
        value.0.get()
    }
}
impl AdminAuditLogId {
    #[must_use]
    pub const fn value(self) -> super::PositiveNonZeroI64 {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminIdTryFromI64Error;

#[cfg(test)]
mod tests {
    #[test]
    fn user_identifier_round_trips_i64() {
        let identifier = super::AdminUserId::try_from(constants_i64::ONE)
            .expect("2c819d47 positive identifier must be valid");
        assert_eq!(i64::from(identifier), constants_i64::ONE);
    }
}
