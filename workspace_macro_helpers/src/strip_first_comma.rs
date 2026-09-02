pub fn strip_first_comma<I>(i: &mut I) -> crate::first_comma_stripped::FirstCommaStripped
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    crate::first_comma_stripped::FirstCommaStripped::from(
        matches!(i.next(), Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == ','),
    )
}
