#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(try_from = "i64")]
#[schema(value_type = i64)]
pub struct AdminPermissionId(crate::positive_non_zero_i64::PositiveNonZeroI64);
impl TryFrom<i64> for AdminPermissionId {
    type Error = super::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(i64: i64) -> Result<Self, Self::Error> {
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(i64).map(Self)
    }
}
impl From<AdminPermissionId> for i64 {
    fn from(admin_permission_id: AdminPermissionId) -> Self {
        admin_permission_id.0.get()
    }
}
impl AdminPermissionId {
    #[must_use]
    pub const fn value(self) -> crate::positive_non_zero_i64::PositiveNonZeroI64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_permission_identifier_round_trips_i64() {
        let identifier = super::AdminPermissionId::try_from(constants_i64::ONE)
            .expect(constants_str::DIAGNOSTIC_F28B31C9);
        assert_eq!(i64::from(identifier), constants_i64::ONE);
    }
}
