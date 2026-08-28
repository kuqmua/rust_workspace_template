use crate::domain_types::AdminIdTryFromI64Error;

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
pub struct AdminAuditLogId(server_admin_contract::domain_types::PositiveNonZeroI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::domain_types::PositiveNonZeroI64::try_from(value)
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
    pub const fn value(self) -> server_admin_contract::domain_types::PositiveNonZeroI64 {
        self.0
    }
}
