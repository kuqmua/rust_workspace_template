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
pub struct AdminRoleId(super::super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminRoleId {
    type Error = super::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminRoleId> for i64 {
    fn from(value: AdminRoleId) -> Self {
        value.0.get()
    }
}
impl AdminRoleId {
    #[must_use]
    pub const fn value(self) -> super::super::PositiveNonZeroI64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn role_identifier_round_trips_i64() {
        let identifier = super::AdminRoleId::try_from(constants_i64::ONE)
            .expect("4dbec052 role identifier must be positive");
        assert_eq!(i64::from(identifier), constants_i64::ONE);
    }
}
