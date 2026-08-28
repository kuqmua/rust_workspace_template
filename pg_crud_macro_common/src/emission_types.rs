#[path = "derive_or_impl.rs"]
mod derive_or_impl;
pub use derive_or_impl::*;
#[path = "proc_macro2_generated_rust_token_stream_vec.rs"]
mod proc_macro2_generated_rust_token_stream_vec;
pub use proc_macro2_generated_rust_token_stream_vec::*;
#[path = "non_null_or_nullable_str.rs"]
mod non_null_or_nullable_str;
pub use non_null_or_nullable_str::*;
#[path = "is_nullable_prefix_str.rs"]
mod is_nullable_prefix_str;
pub use is_nullable_prefix_str::*;
#[path = "import_snake_case_str.rs"]
mod import_snake_case_str;
pub use import_snake_case_str::*;
#[path = "import_path_str.rs"]
mod import_path_str;
pub use import_path_str::*;
#[path = "dimension_number.rs"]
mod dimension_number;
pub use dimension_number::*;
#[path = "struct_els_len.rs"]
mod struct_els_len;
pub use struct_els_len::*;
#[path = "de_len.rs"]
mod de_len;
pub use de_len::*;
#[path = "wrap_into_braces.rs"]
mod wrap_into_braces;
pub use wrap_into_braces::*;
#[path = "parse_token_stream_strings.rs"]
mod parse_token_stream_strings;
pub use parse_token_stream_strings::*;
#[path = "parse_error_id_ref.rs"]
mod parse_error_id_ref;
pub use parse_error_id_ref::*;
#[path = "panic_uuid_ref.rs"]
mod panic_uuid_ref;
pub use panic_uuid_ref::*;
#[path = "syn_identifier_type_refs.rs"]
mod syn_identifier_type_refs;
pub use syn_identifier_type_refs::*;
#[path = "syn_field_refs.rs"]
mod syn_field_refs;
pub use syn_field_refs::*;
#[path = "is_standard_non_null.rs"]
mod is_standard_non_null;
pub use is_standard_non_null::*;
#[path = "is_nullable.rs"]
mod is_nullable;
pub use is_nullable::*;
#[path = "import.rs"]
mod import;
pub use import::*;
#[path = "read_or_update.rs"]
mod read_or_update;
pub use read_or_update::*;
#[path = "default_some_one_or_default_some_one_with_max_page_size.rs"]
mod default_some_one_or_default_some_one_with_max_page_size;
pub use default_some_one_or_default_some_one_with_max_page_size::*;
#[path = "eq_or_eq_using_fields.rs"]
mod eq_or_eq_using_fields;
pub use eq_or_eq_using_fields::*;
#[path = "eq_operator_variant.rs"]
mod eq_operator_variant;
pub use eq_operator_variant::*;
#[path = "dimension.rs"]
mod dimension;
pub use dimension::*;
#[path = "dimension_index_number.rs"]
mod dimension_index_number;
pub use dimension_index_number::*;
#[path = "is_nl_prefix_str_max_len.rs"]
mod is_nl_prefix_str_max_len;
use is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN;

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
