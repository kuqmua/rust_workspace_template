#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_DISPLAY_NAME_MAX_CHARS, min = crate::identity::ADMIN_DISPLAY_NAME_MIN_CHARS, chars, serde, utoipa, validator = crate::identity::ADMIN_DISPLAY_NAME_IS_VALID, description = "administrator display name")]
pub struct AdminDisplayName(String);
