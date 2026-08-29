#[cfg(test)]
mod tests {
    #[test]
    fn struct_shape_preserves_named_tuple_and_unit_forms() {
        let named = syn::parse_quote!(
            struct Named {
                value: u8,
            }
        );
        let tuple = syn::parse_quote!(
            struct Tuple(u8);
        );
        let unit = syn::parse_quote!(
            struct Unit;
        );
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&named),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Named(_))
        ));
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&tuple),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Tuple(_))
        ));
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&unit),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Unit)
        ));
    }
    #[test]
    fn split_top_level_commas_keeps_generic_type_commas_inside_part() {
        let parts = crate::split_top_level_commas::split_top_level_commas(quote::quote! {
            Vec<Result<A, B>>,
            Option<C>
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some("Vec < Result < A , B > >".to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some("Option < C >".to_owned())
        );
    }
    #[test]
    fn split_top_level_commas_keeps_fat_arrow_pair_as_single_part() {
        let parts = crate::split_top_level_commas::split_top_level_commas(quote::quote! {
            SomeType => "message",
            OtherType => format!("{}" , value)
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some("SomeType => \"message\"".to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some("OtherType => format ! (\"{}\" , value)".to_owned())
        );
    }
    #[test]
    fn proc_macro2_macro_tokens_to_tokens_preserves_stream() {
        let tokens = crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(quote::quote! {
            Result<Vec<A>, B>
        });
        assert_eq!(
            quote::quote! {#tokens}.to_string(),
            "Result < Vec < A > , B >"
        );
    }
    #[test]
    fn unique_option_set_preserves_first_span_aware_error() {
        let mut values = crate::unique_option_b_tree_set::UniqueOptionBTreeSet::default();
        values
            .try_insert_with(1u8, || {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    constants_str::catalog::FIRST_ALT,
                )
            })
            .expect(
                "12817d29 unique_option_set_preserves_first_span_aware_error invariant must hold",
            );
        let error = values
            .try_insert_with(1u8, || {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    constants_str::catalog::DUPLICATE,
                )
            })
            .expect_err(constants_str::catalog::CE4826F4);
        assert_eq!(error.to_string(), "duplicate");
        assert!(values.contains(1u8).get());
    }
}
