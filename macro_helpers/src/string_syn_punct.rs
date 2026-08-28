#[must_use]
pub fn string_syn_punct() -> super::SynPathSegments {
    super::generate_simple_syn_punct([
        constants_str::STD,
        constants_str::STRING_ALT,
        constants_str::STRING,
    ])
}
