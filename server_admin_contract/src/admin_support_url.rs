#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| text_policy::validate_https_url_text::validate_https_url_text(text_policy::https_url_text_ref::HttpsUrlTextRef::from(value.as_str())).is_ok(),
    description = "administrator support URL"
)]
pub struct AdminSupportUrl(
    bounded_types::bounded_string::BoundedString<
        { constants_usize::ONE },
        { constants_usize::VALUE_8_192 },
        true,
    >,
);
