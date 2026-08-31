#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::Display,
)]
pub struct PgTableNameRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTableNameRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
