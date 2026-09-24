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
pub struct AdminPermissionResourceActionId(crate::positive_non_zero_i64::PositiveNonZeroI64);

impl TryFrom<i64> for AdminPermissionResourceActionId {
    type Error = crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).map(Self)
    }
}

impl From<AdminPermissionResourceActionId> for i64 {
    fn from(value: AdminPermissionResourceActionId) -> Self {
        value.0.get()
    }
}

impl AdminPermissionResourceActionId {
    #[must_use]
    pub fn from_frontend_path(
        admin_page_path_ref: crate::admin_page_path_ref::AdminPagePathRef<'_>,
    ) -> Option<Self> {
        admin_page_path_ref
            .record_id(crate::admin_data_table::AdminDataTable::PermissionResourceActions)
            .map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_permission_resource_action_identifier_parses_from_detail_frontend_path() {
        assert_eq!(
            super::AdminPermissionResourceActionId::try_from(0i64),
            Err(crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error::Invalid)
        );
        let identifier_result = super::AdminPermissionResourceActionId::try_from(7i64);
        identifier_result.ok().into_iter().for_each(|identifier| {
            let route_path = crate::admin_route_path::AdminRoutePath::from(identifier);
            assert_eq!(
                super::AdminPermissionResourceActionId::from_frontend_path(
                    crate::admin_page_path_ref::AdminPagePathRef::from(route_path.as_ref())
                ),
                Some(identifier)
            );
        });
    }
}
