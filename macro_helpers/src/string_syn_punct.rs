#[must_use]
pub fn string_syn_punct() -> crate::syn_path_segments::SynPathSegments {
    crate::generate_simple_syn_punct::generate_simple_syn_punct([
        constants_str::STD,
        constants_str::STRING_ALT,
        constants_str::STRING,
    ])
}
