#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
)]
#[bounded_string(max = constants_usize::VALUE_8_192, chars)]
pub(crate) struct AdminCsrfToken(
    bounded_types::bounded_string::BoundedString<0usize, { constants_usize::VALUE_8_192 }, true>,
);

impl std::fmt::Debug for AdminCsrfToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::REDACTED_ALT_3)
    }
}
