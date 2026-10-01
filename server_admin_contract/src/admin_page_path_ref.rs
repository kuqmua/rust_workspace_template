#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);

impl<'path_lt> AdminPagePathRef<'path_lt> {
    pub(crate) fn record_identifier(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
    ) -> Option<&'path_lt str> {
        self.get()
            .strip_prefix(admin_data_table.frontend_path().as_ref())
            .and_then(|value| value.strip_prefix('/'))
    }

    pub(crate) fn record_read_id(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
        admin_frontend_path: crate::admin_frontend_path::AdminFrontendPath,
    ) -> Option<crate::positive_non_zero_i64::PositiveNonZeroI64> {
        self.record_id(admin_data_table).or_else(|| {
            let (prefix, parameter_suffix) = admin_frontend_path.get().split_once('{')?;
            let (_parameter, suffix) = parameter_suffix.split_once('}')?;
            let value = self
                .get()
                .strip_prefix(prefix)?
                .strip_suffix(suffix)?
                .parse::<i64>()
                .ok()?;
            crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).ok()
        })
    }

    pub(crate) fn record_id(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
    ) -> Option<crate::positive_non_zero_i64::PositiveNonZeroI64> {
        let value = self
            .record_identifier(admin_data_table)
            .and_then(|value| value.parse::<i64>().ok())?;
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).ok()
    }
}
