#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_display::Display,
)]
pub struct SqlColumnRef<'column_lt>(&'column_lt dyn std::fmt::Display);
impl<'column_lt, T> From<&'column_lt T> for SqlColumnRef<'column_lt>
where
    T: std::fmt::Display,
{
    fn from(value: &'column_lt T) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for SqlColumnRef<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple(constants_str::SQLCOLUMNREF).finish()
    }
}
