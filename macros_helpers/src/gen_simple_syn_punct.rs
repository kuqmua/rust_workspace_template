fn mk_path_segment(v: &str) -> syn::PathSegment {
    syn::PathSegment {
        ident: proc_macro2::Ident::new(v, proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    }
}
#[must_use]
pub fn gen_simple_syn_punct(
    v: &[&str],
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
    let mut acc = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
    if let Some((last, rest)) = v.split_last() {
        for el in rest {
            acc.push_value(mk_path_segment(el));
            acc.push_punct(syn::token::PathSep {
                spans: [
                    proc_macro2::Span::call_site(),
                    proc_macro2::Span::call_site(),
                ],
            });
        }
        acc.push_value(mk_path_segment(last));
    }
    acc
}
#[must_use]
pub fn string_syn_punct() -> syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep> {
    gen_simple_syn_punct(&["std", "string", "String"])
}
#[cfg(test)]
mod tests {
    #[test]
    fn gen_simple_syn_punct_builds_three_segment_path() {
        let punct = super::gen_simple_syn_punct(&["std", "string", "String"]);
        assert_eq!(
            quote::quote! {#punct}.to_string(),
            "std :: string :: String"
        );
    }
    #[test]
    fn gen_simple_syn_punct_builds_single_segment_path() {
        let punct = super::gen_simple_syn_punct(&["Only"]);
        assert_eq!(quote::quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn gen_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct = super::gen_simple_syn_punct(&[]);
        assert!(punct.is_empty());
    }
}
