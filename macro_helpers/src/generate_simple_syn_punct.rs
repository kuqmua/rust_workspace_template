#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[must_use]
pub fn generate_simple_syn_punct<I, S>(v: I) -> crate::syn_path_segments::SynPathSegments
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut accumulator =
        syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
    let mut iter = v.into_iter().peekable();
    while let Some(element) = iter.next() {
        accumulator.push_value(syn::PathSegment {
            ident: proc_macro2::Ident::new(element.as_ref(), proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        if iter.peek().is_some() {
            accumulator.push_punct(syn::token::PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
    }
    crate::syn_path_segments::SynPathSegments::from(accumulator)
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_simple_syn_punct_builds_three_segment_path() {
        let punct = crate::generate_simple_syn_punct::generate_simple_syn_punct([
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
    fn test_generate_simple_syn_punct_builds_single_segment_path() {
        let punct =
            crate::generate_simple_syn_punct::generate_simple_syn_punct([constants_str::ONLY]);
        assert_eq!(quote::quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn test_generate_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct =
            crate::generate_simple_syn_punct::generate_simple_syn_punct(std::iter::empty::<&str>());
        assert!(punct.is_empty());
    }
}
