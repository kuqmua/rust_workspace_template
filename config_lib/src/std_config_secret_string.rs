#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::DebugRedacted,
    proc_macro_newtype::DerefInner,
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
