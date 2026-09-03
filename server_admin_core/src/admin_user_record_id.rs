#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
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
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(try_from = "i64")]
pub struct AdminUserRecordId(server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserRecordId {
    type Error = crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error;
    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        crate::admin_entity_id_from_i64::admin_entity_id_from_i64(i64).map(Self)
    }
}
impl AdminUserRecordId {
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0.get()
    }
    #[must_use]
    pub const fn value(self) -> server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64 {
        self.0
    }
}
