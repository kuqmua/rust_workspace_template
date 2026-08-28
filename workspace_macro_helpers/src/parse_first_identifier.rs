use crate::domain_types::FirstIdentifier;

pub fn parse_first_identifier<I>(input: &mut I) -> Option<FirstIdentifier>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match input.next()? {
        proc_macro2::TokenTree::Ident(identifier) => Some(
            FirstIdentifier::try_from(identifier.to_string()).unwrap_or_else(FirstIdentifier::from),
        ),
        proc_macro2::TokenTree::Group(group)
            if group.delimiter() == proc_macro2::Delimiter::None =>
        {
            parse_first_identifier(&mut group.stream().into_iter())
        }
        proc_macro2::TokenTree::Group(_)
        | proc_macro2::TokenTree::Punct(_)
        | proc_macro2::TokenTree::Literal(_) => None,
    }
}
