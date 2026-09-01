#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::Display)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::SQLCOLUMNREF).finish()
    }
}
