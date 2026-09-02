#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::DerefInner,
)]
pub(super) struct OsStringValue(std::ffi::OsString);
impl From<&str> for OsStringValue {
    fn from(value: &str) -> Self {
        Self(std::ffi::OsString::from(value))
    }
}
