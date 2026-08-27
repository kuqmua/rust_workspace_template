pub fn try_get_macro_attribute<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> Result<super::SynMacroAttrRef<'lt>, super::MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    super::find_macro_attribute(attrs, attr_path).ok_or(super::MacroAttrError::NoAttr)
}
