pub fn try_get_macro_attr_meta_list_token_stream<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> Result<super::ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>, super::MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    let attr = super::try_get_macro_attribute(attrs, attr_path)?;
    if let syn::Meta::List(v) = &attr.0.meta {
        Ok(super::ProcMacro2MacroAttrMetaListTokenStreamRef::from(
            &v.tokens,
        ))
    } else {
        Err(super::MacroAttrError::AttrNotList)
    }
}
