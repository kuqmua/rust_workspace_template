#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_PASSWORD_MAX_CHARS, min = crate::identity::ADMIN_PASSWORD_MIN_CHARS, chars, serde, utoipa, write_only, description = "administrator password")]
pub struct AdminPassword(
    bounded_types::bounded_string::BoundedString<
        { crate::identity::ADMIN_PASSWORD_MIN_CHARS },
        { crate::identity::ADMIN_PASSWORD_MAX_CHARS },
        true,
    >,
);
