#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules stay paired with their facade imports and reexports"
)]
#[path = "domain_types/first_ident_max_len.rs"]
mod first_ident_max_len;
use first_ident_max_len::FIRST_IDENT_MAX_LEN;
#[path = "domain_types/collection_max_len.rs"]
mod collection_max_len;
use collection_max_len::COLLECTION_MAX_LEN;
#[path = "domain_types/syn_derive_input_ref.rs"]
mod syn_derive_input_ref;
pub use syn_derive_input_ref::*;
#[path = "domain_types/syn_struct_shape_ref.rs"]
mod syn_struct_shape_ref;
pub use syn_struct_shape_ref::*;
#[path = "domain_types/syn_fields_named_ref.rs"]
mod syn_fields_named_ref;
pub use syn_fields_named_ref::*;
#[path = "domain_types/syn_fields_unnamed_ref.rs"]
mod syn_fields_unnamed_ref;
pub use syn_fields_unnamed_ref::*;
#[path = "domain_types/proc_macro2_macro_tokens.rs"]
mod proc_macro2_macro_tokens;
pub use proc_macro2_macro_tokens::*;
#[path = "domain_types/proc_macro2_top_level_comma_parts.rs"]
mod proc_macro2_top_level_comma_parts;
pub use proc_macro2_top_level_comma_parts::*;
#[path = "domain_types/top_level_comma_part.rs"]
mod top_level_comma_part;
use top_level_comma_part::TopLevelCommaPart;
#[path = "domain_types/first_identifier.rs"]
mod first_identifier;
pub use first_identifier::*;
#[path = "domain_types/first_identifierifier_try_from_string_error.rs"]
mod first_identifierifier_try_from_string_error;
pub use first_identifierifier_try_from_string_error::*;
#[path = "domain_types/unique_option_b_tree_set.rs"]
mod unique_option_b_tree_set;
pub use unique_option_b_tree_set::*;
#[path = "domain_types/std_unique_option_set_contains.rs"]
mod std_unique_option_set_contains;
pub use std_unique_option_set_contains::*;
#[path = "domain_types/std_unique_option_set_is_empty.rs"]
mod std_unique_option_set_is_empty;
pub use std_unique_option_set_is_empty::*;
#[path = "domain_types/first_comma_stripped.rs"]
mod first_comma_stripped;
pub use first_comma_stripped::*;
#[path = "domain_types/part_index.rs"]
mod part_index;
pub use part_index::*;
#[path = "domain_types/compile_error_token_stream.rs"]
mod compile_error_token_stream;
pub use compile_error_token_stream::*;
#[path = "domain_types/split_top_level_commas.rs"]
mod split_top_level_commas;
pub use split_top_level_commas::*;
#[path = "domain_types/functions.rs"]
mod functions;
pub use functions::*;
#[path = "domain_types/strip_first_comma.rs"]
mod strip_first_comma;
pub use strip_first_comma::*;
#[path = "domain_types/part_at.rs"]
mod part_at;
pub use part_at::*;
#[path = "domain_types/first_identifier_at.rs"]
mod first_identifier_at;
pub use first_identifier_at::*;
#[path = "domain_types/split_fat_arrow.rs"]
mod split_fat_arrow;
pub use split_fat_arrow::*;
#[path = "domain_types/closure_identifier_and_body.rs"]
mod closure_identifier_and_body;
pub use closure_identifier_and_body::*;

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
