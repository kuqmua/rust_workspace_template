use super::domain_types::AdminIdTryFromI64Error;

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
pub struct AdminUserId(server_admin_contract::domain_types::PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserId {
    type Error = AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::domain_types::PositiveNonZeroI64::try_from(value)
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
    pub const fn value(self) -> server_admin_contract::domain_types::PositiveNonZeroI64 {
        self.0
    }
}
