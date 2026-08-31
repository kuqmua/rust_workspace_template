#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::Display,
)]
pub struct PgTableSqlFragmentRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTableSqlFragmentRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
