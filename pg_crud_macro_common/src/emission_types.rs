pub use super::de_len::*;
pub use super::default_some_one_or_default_some_one_with_max_page_size::*;
pub use super::derive_or_impl::*;
pub use super::dimension::*;
pub use super::dimension_index_number::*;
pub use super::dimension_number::*;
pub use super::eq_operator_variant::*;
pub use super::eq_or_eq_using_fields::*;
pub use super::import::*;
pub use super::import_path_str::*;
pub use super::import_snake_case_str::*;
pub(crate) use super::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN;
pub use super::is_nullable::*;
pub use super::is_nullable_prefix_str::*;
pub use super::is_standard_non_null::*;
pub use super::non_null_or_nullable_str::*;
pub use super::panic_uuid_ref::*;
pub use super::parse_error_id_ref::*;
pub use super::parse_token_stream_strings::*;
pub use super::proc_macro2_generated_rust_token_stream_vec::*;
pub use super::read_or_update::*;
pub use super::struct_els_len::*;
pub use super::syn_field_refs::*;
pub use super::syn_identifier_type_refs::*;
pub use super::wrap_into_braces::*;
pg_crud_macro_common_macros::bool_enum_to_tokens!(AddOperatorUndrscr, false => naming::domain_types::AddOperatorSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(ColumnParameterUndrscr, false => naming::domain_types::ColumnSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IncrementParameterUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsCreateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectOnlyCreatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectOnlyUpdatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartColumnFieldForErrorMessageUsed, false => quote::quote! {_}, true => naming::domain_types::ColumnFieldForErrorMessageSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartIsPgTypeUsed, false => quote::quote! {_}, true => quote::quote! {is_pg_type});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsSelectQueryPartSelfSelectUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsUpdateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsUpdateQueryPartSelfUpdateUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
pg_crud_macro_common_macros::bool_enum_to_tokens!(ShouldDSchemarsJsonSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, schemars::JsonSchema});
pg_crud_macro_common_macros::bool_enum_to_tokens!(ShouldDeriveUtoipaToSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, utoipa::ToSchema});
pg_crud_macro_common_macros::bool_enum_to_tokens!(IsPrimaryKeyUndrscr, false => naming::domain_types::IsPrimaryKeySnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryBindValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryPartIncrementUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(CreateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(SelectQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartAccumulatorUndrscr, false => quote::quote! {update_accumulator}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartPathUndrscr, false => quote::quote! {update_path}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartTargetUndrscr, false => quote::quote! {update_target}, true => quote::quote! {_});
pg_crud_macro_common_macros::bool_enum_to_tokens!(UpdateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});

#[cfg(test)]
mod tests {
    #[test]
    fn import_paths_match_their_owners() {
        assert_eq!(
            super::Import::Crate.to_path().to_string(),
            constants_str::CRATE
        );
        assert_eq!(
            super::Import::PgCrudCommon.to_path().to_string(),
            constants_str::PG_CRUD_COMMON_DOMAIN_TYPES
        );
    }
}
