#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner, to_tokens)]
pub struct SynMacroAttrRef<'lt>(&'lt syn::Attribute);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(deref_target, from_inner, to_tokens)]
pub struct ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>(&'lt proc_macro2::TokenStream);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttrPathMatches(bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum MacroAttrError {
    #[error("attr_not_list")]
    AttrNotList,
    #[error("no_attr")]
    NoAttr,
}
#[allow(clippy::single_call_fn)] // helper keeps segment comparison logic isolated and reusable for future attr queries
fn attr_path_matches<S>(attr: SynMacroAttrRef<'_>, attr_path: S) -> AttrPathMatches
where
    S: AsRef<str>,
{
    let mut attr_segments = attr.0.path().segments.iter();
    let mut expected_segments = attr_path
        .as_ref()
        .split(str_constants::PATH_SEPARATOR)
        .map(str::trim)
        .filter(|element| !element.is_empty());
    loop {
        match (attr_segments.next(), expected_segments.next()) {
            (Some(attr_segment), Some(expected_segment)) => {
                if attr_segment.ident != expected_segment {
                    return AttrPathMatches(false);
                }
            }
            (None, None) => {
                return AttrPathMatches(true);
            }
            (Some(_), None) | (None, Some(_)) => {
                return AttrPathMatches(false);
            }
        }
    }
}
#[must_use]
pub fn find_macro_attr<'lt, A, S>(attrs: A, attr_path: S) -> Option<SynMacroAttrRef<'lt>>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    attrs
        .into_iter()
        .map(SynMacroAttrRef)
        .find(|attr| attr_path_matches(*attr, attr_path).0)
}
pub fn try_get_macro_attr<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> Result<SynMacroAttrRef<'lt>, MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    find_macro_attr(attrs, attr_path).ok_or(MacroAttrError::NoAttr)
}
pub fn try_get_macro_attr_meta_list_token_stream<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> Result<ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>, MacroAttrError>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    let attr = try_get_macro_attr(attrs, attr_path)?;
    if let syn::Meta::List(v) = &attr.0.meta {
        Ok(ProcMacro2MacroAttrMetaListTokenStreamRef::from(&v.tokens))
    } else {
        Err(MacroAttrError::AttrNotList)
    }
}
#[must_use]
pub fn get_macro_attr<'lt, A, S>(attrs: A, attr_path: S) -> SynMacroAttrRef<'lt>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    crate::panic_if_err::panic_if_err(try_get_macro_attr(attrs, attr_path), |error| {
        format!("68acaa15:{error}:{}", attr_path.as_ref())
    })
}
#[must_use]
pub fn get_macro_attr_meta_list_token_stream<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> ProcMacro2MacroAttrMetaListTokenStreamRef<'lt>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    crate::panic_if_err::panic_if_err(
        try_get_macro_attr_meta_list_token_stream(attrs, attr_path),
        |error| format!("9d057161:{error}:{}", attr_path.as_ref()),
    )
}
#[cfg(test)]
mod tests {
    fn attrs() -> Vec<syn::Attribute> {
        vec![
            syn::parse_quote!(#[sqlx::type_name(name = str_constants::X)]),
            syn::parse_quote!(#[serde(default)]),
        ]
    }
    #[test]
    fn get_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = super::get_macro_attr(&attrs, str_constants::SQLX_PATH_TYPE_NAME);
        assert!(
            quote::quote! {#attr}
                .to_string()
                .contains("sqlx :: type_name")
        );
    }
    #[test]
    fn get_macro_attr_meta_list_token_stream_returns_list_tokens() {
        let attrs = attrs();
        let ts = super::get_macro_attr_meta_list_token_stream(&attrs, str_constants::SERDE);
        assert_eq!(ts.to_string(), "default");
    }
    #[test]
    fn find_macro_attr_returns_none_when_path_not_present() {
        let attrs = attrs();
        assert!(super::find_macro_attr(&attrs, "missing::attr").is_none());
    }
    #[test]
    fn try_get_macro_attr_returns_error_when_attr_not_found() {
        let attrs = attrs();
        assert_eq!(
            super::try_get_macro_attr(&attrs, "missing::attr"),
            Err(super::MacroAttrError::NoAttr)
        );
    }
    #[test]
    fn try_get_macro_attr_meta_list_token_stream_returns_error_for_non_list_attr() {
        let attrs = vec![syn::parse_quote!(#[allow])];
        assert!(matches!(
            super::try_get_macro_attr_meta_list_token_stream(&attrs, "allow"),
            Err(super::MacroAttrError::AttrNotList)
        ));
    }
    #[test]
    fn find_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, str_constants::SQLX_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_accepts_leading_colons_in_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, str_constants::PATH_SQLX_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_empty_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, str_constants::PATH);
        assert!(attr.is_none());
    }
    #[test]
    fn find_macro_attr_ignores_empty_segments_between_path_separators() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, str_constants::SQLX_PATH_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_partial_path_match() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, str_constants::SQLX);
        assert!(attr.is_none());
    }
}
