#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars)]
pub(in crate::domain_types::start) struct AdminCsrfToken(String);

impl std::fmt::Debug for AdminCsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
