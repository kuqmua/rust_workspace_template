#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = 7usize,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.len() == 7usize && value.bytes().next() == Some(b'#') && value.bytes().skip(constants_usize::ONE).all(|byte| byte.is_ascii_hexdigit()),
    description = "administrator primary color"
)]
pub struct AdminPrimaryColor(
    bounded_types::bounded_string::BoundedString<7usize, { constants_usize::VALUE_8_192 }, true>,
);
