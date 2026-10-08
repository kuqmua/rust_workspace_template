proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(AddOperatorUnderscore, false => naming::domain_types::AddOperatorSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(ColumnParameterUnderscore, false => naming::domain_types::ColumnSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IncrementParameterUnderscore, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
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
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(IsPrimaryKeyUnderscore, false => naming::domain_types::IsPrimaryKeySnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryBindValueUnderscore, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryPartIncrementUnderscore, false => naming::domain_types::IncrementSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(CreateQueryPartValueUnderscore, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(SelectQueryPartValueUnderscore, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartAccumulatorUnderscore, false => quote::quote! {update_accumulator}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartPathUnderscore, false => quote::quote! {update_path}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartTargetUnderscore, false => quote::quote! {update_target}, true => quote::quote! {_});
proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(UpdateQueryPartValueUnderscore, false => naming::domain_types::VSnakeCase, true => quote::quote! {_});

#[cfg(test)]
mod tests {
    #[test]
    fn test_bool_enum_preserves_nested_branch_delimiters() {
        proc_macro_bool_enum_to_tokens::bool_enum_to_tokens!(
            NestedBoolEnumFixture,
            false => quote::quote! { value, true => value },
            true => quote::quote! { other }
        );
        assert_eq!(
            quote::ToTokens::to_token_stream(&NestedBoolEnumFixture::False).to_string(),
            quote::quote! { value, true => value }.to_string(),
        );
        assert_eq!(
            quote::ToTokens::to_token_stream(&NestedBoolEnumFixture::True).to_string(),
            quote::quote! { other }.to_string(),
        );
    }
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
    #[test]
    fn test_emission_switches_preserve_both_exact_token_branches() {
        assert!([
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::AddOperatorUnderscore::False),
                quote::quote! { add_operator },
                quote::ToTokens::to_token_stream(&crate::emission_types::AddOperatorUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::ColumnParameterUnderscore::False),
                quote::quote! { column },
                quote::ToTokens::to_token_stream(&crate::emission_types::ColumnParameterUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IncrementParameterUnderscore::False),
                quote::quote! { increment },
                quote::ToTokens::to_token_stream(&crate::emission_types::IncrementParameterUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsCreateQueryBindMut::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsCreateQueryBindMut::True),
                quote::quote! { mut },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsQueryBindMut::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsQueryBindMut::True),
                quote::quote! { mut },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectOnlyCreatedIdsQueryBindMut::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectOnlyCreatedIdsQueryBindMut::True),
                quote::quote! { mut },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectOnlyUpdatedIdsQueryBindMut::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectOnlyUpdatedIdsQueryBindMut::True),
                quote::quote! { mut },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartColumnFieldForErrorMessageUsed::False),
                quote::quote! { _ },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartColumnFieldForErrorMessageUsed::True),
                quote::quote! { column_field_for_error_message },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartIsPgTypeUsed::False),
                quote::quote! { _ },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartIsPgTypeUsed::True),
                quote::quote! { is_pg_type },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartSelfSelectUsed::False),
                quote::quote! { _ },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsSelectQueryPartSelfSelectUsed::True),
                quote::quote! { v },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsUpdateQueryBindMut::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsUpdateQueryBindMut::True),
                quote::quote! { mut },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsUpdateQueryPartSelfUpdateUsed::False),
                quote::quote! { _ },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsUpdateQueryPartSelfUpdateUsed::True),
                quote::quote! { v },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::ShouldDSchemarsJsonSchema::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::ShouldDSchemarsJsonSchema::True),
                quote::quote! { , schemars::JsonSchema },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::ShouldDeriveUtoipaToSchema::False),
                quote::quote! {  },
                quote::ToTokens::to_token_stream(&crate::emission_types::ShouldDeriveUtoipaToSchema::True),
                quote::quote! { , utoipa::ToSchema },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::IsPrimaryKeyUnderscore::False),
                quote::quote! { is_primary_key },
                quote::ToTokens::to_token_stream(&crate::emission_types::IsPrimaryKeyUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryBindValueUnderscore::False),
                quote::quote! { v },
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryBindValueUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryPartIncrementUnderscore::False),
                quote::quote! { increment },
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryPartIncrementUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryPartValueUnderscore::False),
                quote::quote! { v },
                quote::ToTokens::to_token_stream(&crate::emission_types::CreateQueryPartValueUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::SelectQueryPartValueUnderscore::False),
                quote::quote! { v },
                quote::ToTokens::to_token_stream(&crate::emission_types::SelectQueryPartValueUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartAccumulatorUnderscore::False),
                quote::quote! { update_accumulator },
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartAccumulatorUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartPathUnderscore::False),
                quote::quote! { update_path },
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartPathUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartTargetUnderscore::False),
                quote::quote! { update_target },
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartTargetUnderscore::True),
                quote::quote! { _ },
            ),
            (
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartValueUnderscore::False),
                quote::quote! { v },
                quote::ToTokens::to_token_stream(&crate::emission_types::UpdateQueryPartValueUnderscore::True),
                quote::quote! { _ },
            ),
        ].into_iter().all(|(disabled, expected_disabled, enabled, expected_enabled)| {
            disabled.to_string() == expected_disabled.to_string()
                && enabled.to_string() == expected_enabled.to_string()
        }));
    }
}
