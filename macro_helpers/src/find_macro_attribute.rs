#[must_use]
pub fn find_macro_attribute<'lt, A, S>(
    a: A,
    s: S,
) -> Option<crate::syn_macro_attr_ref::SynMacroAttrRef<'lt>>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    a.into_iter()
        .map(crate::syn_macro_attr_ref::SynMacroAttrRef::from)
        .find(|attr| {
            let mut attr_segments = attr.attr().path().segments.iter();
            let mut expected_segments = s
                .as_ref()
                .split(constants_str::PATH_SEPARATOR)
                .map(str::trim)
                .filter(|element| !element.is_empty());
            loop {
                match (attr_segments.next(), expected_segments.next()) {
                    (Some(attr_segment), Some(expected_segment)) => {
                        if attr_segment.ident != expected_segment {
                            break false;
                        }
                    }
                    (None, None) => break true,
                    (Some(_), None) | (None, Some(_)) => break false,
                }
            }
        })
}
