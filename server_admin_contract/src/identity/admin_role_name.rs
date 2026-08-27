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
#[bounded_string(max = super::ADMIN_ROLE_NAME_MAX_CHARS, min = super::ADMIN_ROLE_NAME_MIN_CHARS, chars, serde, utoipa, validator = super::ADMIN_LOGIN_IS_VALID, description = "administrator role name")]
pub struct AdminRoleName(String);
