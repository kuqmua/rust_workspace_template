pub use super::closure_identifier_and_body::*;
pub(crate) use super::collection_max_len::COLLECTION_MAX_LEN;
pub use super::compile_error_token_stream::*;
pub use super::first_comma_stripped::*;
pub(crate) use super::first_ident_max_len::FIRST_IDENT_MAX_LEN;
pub use super::first_identifier::*;
pub use super::first_identifier_at::*;
pub use super::first_identifierifier_try_from_string_error::*;
pub use super::part_at::*;
pub use super::part_index::*;
pub use super::proc_macro2_macro_tokens::*;
pub use super::proc_macro2_top_level_comma_parts::*;
pub use super::split_fat_arrow::*;
pub use super::split_top_level_commas::*;
pub use super::std_unique_option_set_contains::*;
pub use super::std_unique_option_set_is_empty::*;
pub use super::strip_first_comma::*;
pub use super::syn_derive_input_ref::*;
pub use super::syn_fields_named_ref::*;
pub use super::syn_fields_unnamed_ref::*;
pub use super::syn_struct_shape_ref::*;
pub(crate) use super::top_level_comma_part::TopLevelCommaPart;
pub use super::unique_option_b_tree_set::*;
pub use super::*;
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
            super::SynStructShapeRef::try_from(&named),
            Ok(super::SynStructShapeRef::Named(_))
        ));
        assert!(matches!(
            super::SynStructShapeRef::try_from(&tuple),
            Ok(super::SynStructShapeRef::Tuple(_))
        ));
        assert!(matches!(
            super::SynStructShapeRef::try_from(&unit),
            Ok(super::SynStructShapeRef::Unit)
        ));
    }
    #[test]
    fn split_top_level_commas_keeps_generic_type_commas_inside_part() {
        let parts = super::split_top_level_commas(quote::quote! {
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
        let parts = super::split_top_level_commas(quote::quote! {
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
        let tokens = super::ProcMacro2MacroTokens::from(quote::quote! {
            Result<Vec<A>, B>
        });
        assert_eq!(
            quote::quote! {#tokens}.to_string(),
            "Result < Vec < A > , B >"
        );
    }
    #[test]
    fn unique_option_set_preserves_first_span_aware_error() {
        let mut values = super::UniqueOptionBTreeSet::default();
        values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), constants_str::FIRST_ALT)
            })
            .expect(
                "12817d29 unique_option_set_preserves_first_span_aware_error invariant must hold",
            );
        let error = values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), constants_str::DUPLICATE)
            })
            .expect_err(constants_str::CE4826F4);
        assert_eq!(error.to_string(), "duplicate");
        assert!(values.contains(1u8).get());
    }
}
