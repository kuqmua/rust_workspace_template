pub fn try_get_macro_attr_meta_list_token_stream<'lt, A, S>(
    a: A,
    s: S,
) -> Result<crate::proc_macro2_macro_attr_meta_list_token_stream_ref::ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>, crate::macro_attr_error::MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    let attr = crate::try_get_macro_attribute::try_get_macro_attribute(a, s)?;
    if let syn::Meta::List(v) = &attr.attr().meta {
        Ok(crate::proc_macro2_macro_attr_meta_list_token_stream_ref::ProcMacro2MacroAttrMetaListTokenStreamRef::from(
            &v.tokens,
        ))
    } else {
        Err(crate::macro_attr_error::MacroAttrError::AttrNotList)
    }
}
