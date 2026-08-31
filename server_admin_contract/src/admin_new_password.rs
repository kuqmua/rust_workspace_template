#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedStringWrapper,
    newtype::DebugRedacted,
    newtype::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_PASSWORD_MAX_CHARS, min = crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS, chars, serde, utoipa, write_only, validator = crate::identity::ADMIN_NEW_PASSWORD_IS_VALID, description = "new administrator password")]
pub struct AdminNewPassword(
    bounded_types::bounded_string::BoundedString<
        { crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS },
        { crate::identity::ADMIN_PASSWORD_MAX_CHARS },
        true,
    >,
);
