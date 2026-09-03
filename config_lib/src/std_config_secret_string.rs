#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_deref_inner::DerefInner,
)]
#[bounded_string(max = 1_048_576, description = "configuration secret text")]
pub struct StdConfigSecretString(
    bounded_types::bounded_string::BoundedString<0usize, 1_048_576, false>,
);
impl secrecy::zeroize::Zeroize for StdConfigSecretString {
    fn zeroize(&mut self) {
        let mut value = std::mem::take(&mut self.0).into_string();
        secrecy::zeroize::Zeroize::zeroize(&mut value);
    }
}
