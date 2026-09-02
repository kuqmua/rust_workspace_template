#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::BoundedStringWrapper,
    proc_macro_newtype::AsRefStr,
)]
#[bounded_string(
    max = constants_usize::VALUE_8_192,
    min = constants_usize::ONE,
    chars,
    serde,
    utoipa,
    validator = |value: &String| value.strip_prefix("https://").is_some_and(|remainder| { let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default(); !authority.is_empty() && !authority.contains('@') && !authority.starts_with('.') && !authority.ends_with('.') && authority.contains('.') }),
    description = "administrator main logo"
)]
pub struct AdminMainLogo(
    bounded_types::bounded_string::BoundedString<
        { constants_usize::ONE },
        { constants_usize::VALUE_8_192 },
        true,
    >,
);
