#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminAuditLogId(crate::positive_non_zero_i64::PositiveNonZeroI64);
impl TryFrom<i64> for AdminAuditLogId {
    type Error = super::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminAuditLogId> for i64 {
    fn from(value: AdminAuditLogId) -> Self {
        value.0.get()
    }
}
impl AdminAuditLogId {
    #[must_use]
    pub const fn value(self) -> crate::positive_non_zero_i64::PositiveNonZeroI64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_audit_log_identifier_round_trips_i64() {
        let identifier = super::AdminAuditLogId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_1A6EF5D3);
        assert_eq!(i64::from(identifier), constants_i64::ONE);
    }
}
