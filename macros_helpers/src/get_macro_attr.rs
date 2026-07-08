#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynMacroAttrRef<'lt>(&'lt syn::Attribute);
impl<'lt> From<&'lt syn::Attribute> for SynMacroAttrRef<'lt> {
    fn from(value: &'lt syn::Attribute) -> Self {
        Self(value)
    }
}
impl quote::ToTokens for SynMacroAttrRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ProcMacro2MacroAttrMetaListTsRef<'lt>(&'lt proc_macro2::TokenStream);
impl<'lt> From<&'lt proc_macro2::TokenStream> for ProcMacro2MacroAttrMetaListTsRef<'lt> {
    fn from(value: &'lt proc_macro2::TokenStream) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for ProcMacro2MacroAttrMetaListTsRef<'_> {
    type Target = proc_macro2::TokenStream;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl quote::ToTokens for ProcMacro2MacroAttrMetaListTsRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.clone());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AttrPathMatches(bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum MacroAttrEr {
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
        .split("::")
        .map(str::trim)
        .filter(|el| !el.is_empty());
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
) -> Result<SynMacroAttrRef<'lt>, MacroAttrEr>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    find_macro_attr(attrs, attr_path).ok_or(MacroAttrEr::NoAttr)
}
pub fn try_get_macro_attr_meta_list_ts<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> Result<ProcMacro2MacroAttrMetaListTsRef<'lt>, MacroAttrEr>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    let attr = try_get_macro_attr(attrs, attr_path)?;
    if let syn::Meta::List(v) = &attr.0.meta {
        Ok(ProcMacro2MacroAttrMetaListTsRef::from(&v.tokens))
    } else {
        Err(MacroAttrEr::AttrNotList)
    }
}
#[must_use]
pub fn get_macro_attr<'lt, A, S>(attrs: A, attr_path: S) -> SynMacroAttrRef<'lt>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    crate::panic_if_err::panic_if_err(try_get_macro_attr(attrs, attr_path), |er| {
        format!("68acaa15:{er}:{}", attr_path.as_ref())
    })
}
#[must_use]
pub fn get_macro_attr_meta_list_ts<'lt, A, S>(
    attrs: A,
    attr_path: S,
) -> ProcMacro2MacroAttrMetaListTsRef<'lt>
where
    A: IntoIterator<Item = &'lt syn::Attribute>,
    S: AsRef<str> + Copy,
{
    crate::panic_if_err::panic_if_err(try_get_macro_attr_meta_list_ts(attrs, attr_path), |er| {
        format!("9d057161:{er}:{}", attr_path.as_ref())
    })
}
#[cfg(test)]
mod tests {
    fn attrs() -> Vec<syn::Attribute> {
        vec![
            syn::parse_quote!(#[sqlx::type_name(name = "x")]),
            syn::parse_quote!(#[serde(default)]),
        ]
    }
    #[test]
    fn get_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = super::get_macro_attr(&attrs, "sqlx :: type_name");
        assert!(
            quote::quote! {#attr}
                .to_string()
                .contains("sqlx :: type_name")
        );
    }
    #[test]
    fn get_macro_attr_meta_list_ts_returns_list_tokens() {
        let attrs = attrs();
        let ts = super::get_macro_attr_meta_list_ts(&attrs, "serde");
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
            Err(super::MacroAttrEr::NoAttr)
        );
    }
    #[test]
    fn try_get_macro_attr_meta_list_ts_returns_error_for_non_list_attr() {
        let attrs = vec![syn::parse_quote!(#[allow])];
        assert!(matches!(
            super::try_get_macro_attr_meta_list_ts(&attrs, "allow"),
            Err(super::MacroAttrEr::AttrNotList)
        ));
    }
    #[test]
    fn find_macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, "sqlx :: type_name");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_accepts_leading_colons_in_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, " :: sqlx :: type_name ");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_empty_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, " :: ");
        assert!(attr.is_none());
    }
    #[test]
    fn find_macro_attr_ignores_empty_segments_between_path_separators() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, "sqlx::::type_name");
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_partial_path_match() {
        let attrs = attrs();
        let attr = super::find_macro_attr(&attrs, "sqlx");
        assert!(attr.is_none());
    }
}
