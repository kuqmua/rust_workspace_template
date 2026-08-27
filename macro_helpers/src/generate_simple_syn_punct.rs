#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "generate_simple_syn_punct/generate_simple_syn_punct.rs"]
mod generate_simple_syn_punct;
#[path = "generate_simple_syn_punct/string_syn_punct.rs"]
mod string_syn_punct;
#[path = "generate_simple_syn_punct/syn_path_segment.rs"]
mod syn_path_segment;
#[path = "generate_simple_syn_punct/syn_path_segments.rs"]
mod syn_path_segments;

pub use generate_simple_syn_punct::generate_simple_syn_punct;
pub use string_syn_punct::string_syn_punct;
pub use syn_path_segment::SynPathSegment;
pub use syn_path_segments::SynPathSegments;
#[cfg(test)]
mod tests {
    #[test]
    fn generate_simple_syn_punct_builds_three_segment_path() {
        let punct = super::generate_simple_syn_punct([
            constants_str::STD,
            constants_str::STRING_ALT,
            constants_str::STRING,
        ]);
        assert_eq!(
            quote::quote! {#punct}.to_string(),
            "std :: string :: String"
        );
    }
    #[test]
    fn generate_simple_syn_punct_builds_single_segment_path() {
        let punct = super::generate_simple_syn_punct([constants_str::ONLY]);
        assert_eq!(quote::quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn generate_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct = super::generate_simple_syn_punct(std::iter::empty::<&str>());
        assert!(punct.0.is_empty());
    }
}
