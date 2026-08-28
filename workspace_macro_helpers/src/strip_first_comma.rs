use super::FirstCommaStripped;

pub fn strip_first_comma<I>(input: &mut I) -> FirstCommaStripped
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    FirstCommaStripped::from(
        matches!(input.next(), Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == ','),
    )
}
