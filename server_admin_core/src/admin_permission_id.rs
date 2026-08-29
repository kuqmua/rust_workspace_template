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
pub struct AdminPermissionId(server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(value)
            .map(Self)
            .map_err(|_error| crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error)
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64 {
        self.0
    }
}
