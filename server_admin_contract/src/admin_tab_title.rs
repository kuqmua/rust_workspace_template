#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedStringWrapper,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| !value.trim().is_empty(),
    description = "administrator tab title"
)]
pub struct AdminTabTitle(
    bounded_types::bounded_string::BoundedString<
        { constants_usize::ONE },
        { constants_usize::VALUE_8_192 },
        true,
    >,
);
