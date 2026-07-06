use syn::{PathArguments, PathSegment, punctuated::Punctuated, token::PathSep};
fn mk_path_segment(v: &str) -> PathSegment {
    PathSegment {
        ident: proc_macro2::Ident::new(v, proc_macro2::Span::call_site()),
        arguments: PathArguments::None,
    }
}
#[must_use]
pub fn gen_simple_syn_punct(v: &[&str]) -> Punctuated<PathSegment, PathSep> {
    let mut acc = Punctuated::<PathSegment, PathSep>::new();
    if let Some((last, rest)) = v.split_last() {
        for el in rest {
            acc.push_value(mk_path_segment(el));
            acc.push_punct(PathSep {
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
pub fn string_syn_punct() -> Punctuated<PathSegment, PathSep> {
    gen_simple_syn_punct(&["std", "string", "String"])
}
#[cfg(test)]
mod tests {
    use super::gen_simple_syn_punct;
    use quote::quote;
    #[test]
    fn gen_simple_syn_punct_builds_three_segment_path() {
        let punct = gen_simple_syn_punct(&["std", "string", "String"]);
        assert_eq!(quote! {#punct}.to_string(), "std :: string :: String");
    }
    #[test]
    fn gen_simple_syn_punct_builds_single_segment_path() {
        let punct = gen_simple_syn_punct(&["Only"]);
        assert_eq!(quote! {#punct}.to_string(), "Only");
    }
    #[test]
    fn gen_simple_syn_punct_returns_empty_path_on_empty_input() {
        let punct = gen_simple_syn_punct(&[]);
        assert!(punct.is_empty());
    }
}
