#[must_use]
pub fn generate_simple_syn_punct<I, S>(v: I) -> super::SynPathSegments
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
    super::SynPathSegments::from(accumulator)
}
