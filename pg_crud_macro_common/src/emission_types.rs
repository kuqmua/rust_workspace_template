pub use crate::de_len::*;
pub use crate::default_some_one_or_default_some_one_with_max_page_size::*;
pub use crate::derive_or_impl::*;
pub use crate::dimension::*;
pub use crate::dimension_index_number::*;
pub use crate::dimension_number::*;
pub use crate::eq_operator_variant::*;
pub use crate::eq_or_eq_using_fields::*;
pub use crate::import::*;
pub use crate::import_path_str::*;
pub use crate::import_snake_case_str::*;
pub(crate) use crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN;
pub use crate::is_nullable::*;
pub use crate::is_nullable_prefix_str::*;
pub use crate::is_standard_non_null::*;
pub use crate::non_null_or_nullable_str::*;
pub use crate::panic_uuid_ref::*;
pub use crate::parse_error_id_ref::*;
pub use crate::parse_token_stream_strings::*;
pub use crate::proc_macro2_generated_rust_token_stream_vec::*;
pub use crate::read_or_update::*;
pub use crate::struct_els_len::*;
pub use crate::syn_field_refs::*;
pub use crate::syn_identifier_type_refs::*;
pub use crate::wrap_into_braces::*;

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
