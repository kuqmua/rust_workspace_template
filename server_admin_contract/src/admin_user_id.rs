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
pub struct AdminUserId(super::super::PositiveNonZeroI64);
impl TryFrom<i64> for AdminUserId {
    type Error = super::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        super::super::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminUserId> for i64 {
    fn from(value: AdminUserId) -> Self {
        value.0.get()
    }
}
impl AdminUserId {
    #[must_use]
    pub const fn value(self) -> super::super::PositiveNonZeroI64 {
        self.0
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn user_identifier_round_trips_i64() {
        let identifier = super::AdminUserId::try_from(constants_i64::ONE)
            .expect("2c819d47 positive identifier must be valid");
        assert_eq!(i64::from(identifier), constants_i64::ONE);
    }
}
