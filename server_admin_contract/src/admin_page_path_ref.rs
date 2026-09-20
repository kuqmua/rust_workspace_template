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

impl AdminPagePathRef<'_> {
    pub(crate) fn record_id(
        self,
        admin_data_table: crate::admin_data_table::AdminDataTable,
    ) -> Option<crate::positive_non_zero_i64::PositiveNonZeroI64> {
        let value = self
            .get()
            .strip_prefix(admin_data_table.frontend_path().as_ref())
            .and_then(|value| value.strip_prefix('/'))
            .and_then(|value| value.parse::<i64>().ok())?;
        crate::positive_non_zero_i64::PositiveNonZeroI64::try_from(value).ok()
    }
}
