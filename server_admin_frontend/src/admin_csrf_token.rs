#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars)]
pub(crate) struct AdminCsrfToken(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, true>,
);

impl std::fmt::Debug for AdminCsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
