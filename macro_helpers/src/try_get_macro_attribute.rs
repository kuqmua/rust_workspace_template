pub fn try_get_macro_attribute<'lt, A, S>(
    a: A,
    s: S,
) -> Result<crate::syn_macro_attr_ref::SynMacroAttrRef<'lt>, crate::macro_attr_error::MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    crate::find_macro_attribute::find_macro_attribute(a, s)
        .ok_or(crate::macro_attr_error::MacroAttrError::NoAttr)
}
