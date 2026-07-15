#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(from_inner, into_inner_from)]
pub struct SynPathSegment(syn::PathSegment);
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(from_inner, into_inner_from, to_tokens)]
pub struct SynPathSegments(syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>);
#[must_use]
pub fn generate_simple_syn_punct<I, S>(v: I) -> SynPathSegments
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
    SynPathSegments::from(accumulator)
}
#[must_use]
pub fn string_syn_punct() -> SynPathSegments {
    generate_simple_syn_punct([
        str_constants::STD,
        str_constants::STRING_ALT,
        str_constants::STRING,
    ])
}
#[cfg(test)]
mod tests {
    #[test]
    fn generate_simple_syn_punct_builds_three_segment_path() {
        let punct = super::generate_simple_syn_punct([
            str_constants::STD,
            str_constants::STRING_ALT,
            str_constants::STRING,
        ]);
        assert_eq!(
            quote::quote! {#punct}.to_string(),
            "std :: string :: String"
        );
    }
    #[test]
    fn generate_simple_syn_punct_builds_single_segment_path() {
        let punct = super::generate_simple_syn_punct([str_constants::ONLY]);
        assert_eq!(quote::quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn generate_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct = super::generate_simple_syn_punct(std::iter::empty::<&str>());
        assert!(punct.0.is_empty());
    }
}
