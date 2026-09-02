proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(AddOperatorUndrscr, false => naming::domain_types::AddOperatorSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(ColumnParameterUndrscr, false => naming::domain_types::ColumnSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IncrementParameterUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsCreateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsSelectOnlyCreatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsSelectOnlyUpdatedIdsQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsSelectQueryPartColumnFieldForErrorMessageUsed, false => quote::quote! {_}, true => naming::domain_types::ColumnFieldForErrorMessageSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsSelectQueryPartIsPgTypeUsed, false => quote::quote! {_}, true => quote::quote! {is_pg_type});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsSelectQueryPartSelfSelectUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsUpdateQueryBindMut, false => proc_macro2::TokenStream::new(), true => naming::domain_types::MutSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsUpdateQueryPartSelfUpdateUsed, false => quote::quote! {_}, true => naming::domain_types::VSnakeCase);
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(ShouldDSchemarsJsonSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, schemars::JsonSchema});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(ShouldDeriveUtoipaToSchema, false => proc_macro2::TokenStream::new(), true => quote::quote! {, utoipa::ToSchema});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsPrimaryKeyUndrscr, false => naming::domain_types::IsPrimaryKeySnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryBindValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryPartIncrementUndrscr, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(SelectQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartAccumulatorUndrscr, false => quote::quote! {update_accumulator}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartPathUndrscr, false => quote::quote! {update_path}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartTargetUndrscr, false => quote::quote! {update_target}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartValueUndrscr, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});

#[cfg(test)]
mod tests {
    #[test]
    fn test_import_paths_match_their_owners() {
        assert_eq!(
            crate::import::Import::Crate.to_path().to_string(),
            constants_str::CRATE
        );
        assert_eq!(
            crate::import::Import::PgCrudCommon.to_path().to_string(),
            constants_str::PG_CRUD_COMMON_DOMAIN_TYPES
        );
    }
}
