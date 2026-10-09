#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(try_from = "i64")]
#[derive(proc_macro_getters::Getters)]
pub struct AdminSessionId(
    #[getters(copy)] server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
);
impl TryFrom<i64> for AdminSessionId {
    type Error = server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
