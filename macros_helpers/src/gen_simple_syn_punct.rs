#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(from_inner, into_inner_from)]
pub struct SynPathSegment(syn::PathSegment);
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(from_inner, into_inner_from, to_tokens)]
pub struct SynPathSegments(syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>);
#[allow(clippy::single_call_fn)] // named constructor keeps syn::PathSegment assembly separate from punctuation loop
fn mk_path_segment<S>(v: S) -> SynPathSegment
where
    S: AsRef<str>,
{
    SynPathSegment::from(syn::PathSegment {
        ident: proc_macro2::Ident::new(v.as_ref(), proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    })
}
#[must_use]
pub fn gen_simple_syn_punct<I, S>(v: I) -> SynPathSegments
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut acc = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
    let mut iter = v.into_iter().peekable();
    while let Some(el) = iter.next() {
        acc.push_value(mk_path_segment(el).into());
        if iter.peek().is_some() {
            acc.push_punct(syn::token::PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
    }
    SynPathSegments::from(acc)
}
#[must_use]
pub fn string_syn_punct() -> SynPathSegments {
    gen_simple_syn_punct(["std", "string", "String"])
}
#[cfg(test)]
mod tests {
    #[test]
    fn gen_simple_syn_punct_builds_three_segment_path() {
        let punct = super::gen_simple_syn_punct(["std", "string", "String"]);
        assert_eq!(
            quote::quote! {#punct}.to_string(),
            "std :: string :: String"
        );
    }
    #[test]
    fn gen_simple_syn_punct_builds_single_segment_path() {
        let punct = super::gen_simple_syn_punct(["Only"]);
        assert_eq!(quote::quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn gen_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct = super::gen_simple_syn_punct(std::iter::empty::<&str>());
        assert!(punct.0.is_empty());
    }
}
