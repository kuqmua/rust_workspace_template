#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedString,
    newtype::DebugRedacted,
    newtype::DerefInner,
)]
#[bounded_string(max = 1_048_576, description = "configuration secret text")]
pub struct StdConfigSecretString(String);
impl secrecy::zeroize::Zeroize for StdConfigSecretString {
    fn zeroize(&mut self) {
        secrecy::zeroize::Zeroize::zeroize(&mut self.0);
    }
}
