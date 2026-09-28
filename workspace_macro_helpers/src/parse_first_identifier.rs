pub fn parse_first_identifier<I>(i: &mut I) -> Option<crate::first_identifier::FirstIdentifier>
where
    I: Iterator<Item = proc_macro2::TokenTree>,
{
    match i.next()? {
        proc_macro2::TokenTree::Ident(identifier) => {
            crate::first_identifier::FirstIdentifier::try_from(identifier.to_string()).ok()
        }
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
