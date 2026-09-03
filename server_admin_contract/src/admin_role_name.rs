#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_ROLE_NAME_MAX_CHARS, min = crate::identity::ADMIN_ROLE_NAME_MIN_CHARS, chars, serde, utoipa, validator = crate::identity::ADMIN_LOGIN_IS_VALID, description = "administrator role name")]
pub struct AdminRoleName(
    bounded_types::bounded_string::BoundedString<
        { crate::identity::ADMIN_ROLE_NAME_MIN_CHARS },
        { crate::identity::ADMIN_ROLE_NAME_MAX_CHARS },
        true,
    >,
);
