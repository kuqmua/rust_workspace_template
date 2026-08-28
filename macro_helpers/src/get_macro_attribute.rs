pub use crate::find_macro_attribute::find_macro_attribute;
pub use crate::macro_attr_error::MacroAttrError;
pub use crate::proc_macro2_macro_attr_meta_list_token_stream_ref::ProcMacro2MacroAttrMetaListTokenStreamRef;
pub use crate::syn_macro_attr_ref::SynMacroAttrRef;
pub use crate::try_get_macro_attr_meta_list_token_stream::try_get_macro_attr_meta_list_token_stream;
pub use crate::try_get_macro_attribute::try_get_macro_attribute;
#[cfg(test)]
mod tests {
    fn attrs() -> Vec<syn::Attribute> {
        vec![
            syn::parse_quote!(#[sqlx::type_name(name = constants_str::X)]),
            syn::parse_quote!(#[serde(default)]),
        ]
    }
    #[test]
    fn macro_attr_ignores_spaces_in_lookup_path() {
        let attrs = attrs();
        let attr = super::try_get_macro_attribute(&attrs, constants_str::SQLX_PATH_TYPE_NAME)
            .expect("193fa8d2 get_macro_attr_ignores_spaces_in_lookup_path invariant must hold");
        assert!(
            quote::quote! {#attr}
                .to_string()
                .contains("sqlx :: type_name")
        );
    }
    #[test]
    fn macro_attr_meta_list_token_stream_returns_list_tokens() {
        let attrs = attrs();
        let ts = super::try_get_macro_attr_meta_list_token_stream(&attrs, constants_str::SERDE)
            .expect("647b0c3e get_macro_attr_meta_list_token_stream_returns_list_tokens invariant must hold");
        assert_eq!(ts.to_string(), "default");
    }
    #[test]
    fn find_macro_attr_returns_none_when_path_not_present() {
        let attrs = attrs();
        assert!(super::find_macro_attribute(&attrs, "missing::attr").is_none());
    }
    #[test]
    fn try_get_macro_attr_returns_error_when_attr_not_found() {
        let attrs = attrs();
        assert_eq!(
            super::try_get_macro_attribute(&attrs, "missing::attr"),
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
        let attr = super::find_macro_attribute(&attrs, constants_str::SQLX_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_accepts_leading_colons_in_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attribute(&attrs, constants_str::PATH_SQLX_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_empty_lookup_path() {
        let attrs = attrs();
        let attr = super::find_macro_attribute(&attrs, constants_str::PATH);
        assert!(attr.is_none());
    }
    #[test]
    fn find_macro_attr_ignores_empty_segments_between_path_separators() {
        let attrs = attrs();
        let attr = super::find_macro_attribute(&attrs, constants_str::SQLX_PATH_PATH_TYPE_NAME);
        assert!(attr.is_some());
    }
    #[test]
    fn find_macro_attr_returns_none_for_partial_path_match() {
        let attrs = attrs();
        let attr = super::find_macro_attribute(&attrs, constants_str::SQLX);
        assert!(attr.is_none());
    }
}
