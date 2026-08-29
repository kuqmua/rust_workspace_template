#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedString,
    newtype::DebugRedacted,
    newtype::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_PASSWORD_MAX_CHARS, min = crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS, chars, serde, utoipa, write_only, validator = crate::identity::ADMIN_NEW_PASSWORD_IS_VALID, description = "new administrator password")]
pub struct AdminNewPassword(String);
