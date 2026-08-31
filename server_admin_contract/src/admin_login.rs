#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::BoundedStringWrapper,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_LOGIN_MAX_CHARS, min = crate::identity::ADMIN_LOGIN_MIN_CHARS, chars, serde, utoipa, validator = crate::identity::ADMIN_LOGIN_IS_VALID, description = "administrator login")]
pub struct AdminLogin(
    bounded_types::bounded_string::BoundedString<
        { crate::identity::ADMIN_LOGIN_MIN_CHARS },
        { crate::identity::ADMIN_LOGIN_MAX_CHARS },
        true,
    >,
);
