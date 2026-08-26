#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the tool-command owner reads this private process value wrapper"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DerefInner)]
pub(super) struct OsStringValue(std::ffi::OsString);
impl From<&str> for OsStringValue {
    fn from(value: &str) -> Self {
        Self(std::ffi::OsString::from(value))
    }
}
