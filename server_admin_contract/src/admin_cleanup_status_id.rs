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
pub struct AdminCleanupStatusId(crate::positive_non_zero_i64::PositiveNonZeroI64);

impl TryFrom<i64> for AdminCleanupStatusId {
    type Error = crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map(Self)
    }
}

impl From<AdminCleanupStatusId> for i64 {
    fn from(value: AdminCleanupStatusId) -> Self {
        value.0.get()
    }
}

impl AdminCleanupStatusId {
    #[must_use]
    pub fn from_frontend_path(
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        admin_page_path_ref
            .record_id(crate::admin_data_table::AdminDataTable::CleanupStatus)
            .map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cleanup_status_identifier_parses_from_detail_frontend_path() {
        let expected_identifier_result = super::AdminCleanupStatusId::try_from(constants_i64::ONE);
        assert_eq!(
            expected_identifier_result.iter().count(),
            constants_usize::ONE
        );
        expected_identifier_result
            .ok()
            .into_iter()
            .for_each(|expected_identifier| {
                let route_path = crate::admin_route_path::AdminRoutePath::from(expected_identifier);
                assert_eq!(
                    super::AdminCleanupStatusId::from_frontend_path(
                        crate::admin_page_path_ref::AdminPagePathRef::from(route_path.as_ref()),
                    ),
                    Some(expected_identifier)
                );
            });
    }
}
