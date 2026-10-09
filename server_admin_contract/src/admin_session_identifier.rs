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
pub struct AdminSessionIdentifier(crate::positive_non_zero_i64::PositiveNonZeroI64);
impl TryFrom<i64> for AdminSessionIdentifier {
    type Error = super::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map(Self)
    }
}
impl From<AdminSessionIdentifier> for i64 {
    fn from(value: AdminSessionIdentifier) -> Self {
        value.0.get()
    }
}
impl AdminSessionIdentifier {
    #[must_use]
    pub const fn value(self) -> crate::positive_non_zero_i64::PositiveNonZeroI64 {
        self.0
    }

    #[must_use]
    pub fn from_frontend_path(
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        let (prefix, suffix) = crate::admin_frontend_path::AdminFrontendPath::SessionRead
            .get()
            .split_once(constants_str::ADMIN_SESSION_ID_PLACEHOLDER)?;
        let value = admin_page_path_ref
            .get()
            .strip_prefix(prefix)?
            .strip_suffix(suffix)?
            .parse::<i64>()
            .ok()?;
        Self::try_from(value).ok()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_session_identifier_round_trips_through_read_route() {
        let identifier_result = super::AdminSessionIdentifier::try_from(constants_i64::ONE);
        assert_eq!(identifier_result.iter().count(), constants_usize::ONE);
        identifier_result.ok().into_iter().for_each(|identifier| {
            let route_path = crate::admin_route_path::AdminRoutePath::from(identifier);
            assert_eq!(
                super::AdminSessionIdentifier::from_frontend_path(
                    crate::admin_page_path_ref::AdminPagePathRef::from(route_path.as_ref())
                ),
                Some(identifier)
            );
        });
        assert!(
            super::AdminSessionIdentifier::from_frontend_path(
                crate::admin_page_path_ref::AdminPagePathRef::from(
                    crate::admin_frontend_path::AdminFrontendPath::Sessions.get()
                )
            )
            .is_none()
        );
        assert!(
            super::AdminSessionIdentifier::from_frontend_path(
                crate::admin_page_path_ref::AdminPagePathRef::from(
                    crate::admin_frontend_path::AdminFrontendPath::AccessSessionsRead.get()
                )
            )
            .is_none()
        );
    }
}
