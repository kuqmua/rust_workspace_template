#[test]
fn test_token_string_parser_preserves_empty_input_and_declaration_order() {
    let parse_error_id =
        crate::parse_error_id_ref::ParseErrorIdRef::from(constants_str::DIAGNOSTIC_D1846F3A);
    let empty = crate::parse_token_stream_strings_into_generated_vec::parse_token_stream_strings_into_generated_vec(
        crate::parse_token_stream_strings::ParseTokenStreamStrings::from(Vec::new()),
        parse_error_id,
    );
    assert!(quote::quote! { #empty }.is_empty());
    let declarations = [
        quote::quote! { struct TestParsedFirst; },
        quote::quote! { struct TestParsedSecond; },
    ];
    let input = declarations
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let generated = crate::parse_token_stream_strings::ParseTokenStreamStrings::from(input)
        .into_generated_vec(parse_error_id);
    assert_eq!(
        quote::quote! { #generated }.to_string(),
        quote::quote! { #(#declarations)* }.to_string()
    );
}

#[test]
fn test_token_string_parser_emits_identified_errors_between_valid_items() {
    let first = quote::quote! { struct TestParsedBeforeError; };
    let last = quote::quote! { struct TestParsedAfterError; };
    let malformed = '('.to_string();
    let generated = crate::parse_token_stream_strings_into_generated_vec::parse_token_stream_strings_into_generated_vec(
        crate::parse_token_stream_strings::ParseTokenStreamStrings::from(vec![
            first.to_string(),
            malformed.clone(),
            last.to_string(),
        ]),
        crate::parse_error_id_ref::ParseErrorIdRef::from(constants_str::DIAGNOSTIC_D1846F3A),
    );
    assert!(
        syn::parse2::<syn::File>(quote::quote! { #generated }).is_ok_and(|file| {
            if file.items.len() != 3usize {
                return false;
            }
            let [
                syn::Item::Struct(before),
                syn::Item::Macro(error),
                syn::Item::Struct(after),
            ] = file.items.as_slice()
            else {
                return false;
            };
            before.ident == stringify!(TestParsedBeforeError)
                && after.ident == stringify!(TestParsedAfterError)
                && error.mac.path.is_ident(stringify!(compile_error))
                && error.semi_token.is_some()
                && syn::parse2::<syn::LitStr>(error.mac.tokens.clone()).is_ok_and(|message| {
                    malformed
                        .parse::<proc_macro2::TokenStream>()
                        .err()
                        .is_some_and(|parse_error| {
                            message.value()
                                == format!("{}: {parse_error}", constants_str::DIAGNOSTIC_D1846F3A)
                        })
                })
        })
    );
}
#[test]
fn test_pg_type_test_case_emission_preserves_optional_hook_matrix_and_import_paths() {
    let names = crate::names_context::NamesContext::new();
    let optional_methods = [
        (names.get_optional_vec_create_snake_case().to_string(), 1u8),
        (
            names
                .get_read_ids_and_create_into_optional_vec_where_eq_to_field_snake_case()
                .to_string(),
            2u8,
        ),
        (
            names
                .get_pg_type_optional_vec_where_greater_than_test_snake_case()
                .to_string(),
            4u8,
        ),
        (
            names
                .get_read_ids_and_table_type_into_pg_type_optional_where_greater_than_snake_case()
                .to_string(),
            8u8,
        ),
    ];
    assert!([crate::import::Import::Crate, crate::import::Import::PgCrudCommon].into_iter().all(|import| {
        (0u8..16u8).all(|mask| {
            let cfg = quote::quote! { #[cfg(any())] };
            let identifier = quote::quote! { TestGeneratedHookOwner };
            let bodies = [
                quote::quote! { 0u8 },
                quote::quote! { 1u8 },
                quote::quote! { 2u8 },
                quote::quote! { 3u8 },
                quote::quote! { 4u8 },
                quote::quote! { 5u8 },
                quote::quote! { 6u8 },
                quote::quote! { 7u8 },
                quote::quote! { 8u8 },
                quote::quote! { 9u8 },
                quote::quote! { 10u8 },
                quote::quote! { 11u8 },
                quote::quote! { 12u8 },
                quote::quote! { 13u8 },
                quote::quote! { 14u8 }
            ];
            let expected_methods = [
                quote::quote! { fn optional_vec_create() -> Option<Vec<<Self::PgType as #import::pg_type::PgType>::Create>> { 0u8 } },
                quote::quote! { fn read_ids_to_2_dimensions_vec_read_inner(read_ids: &<Self::PgType as #import::pg_type::PgType>::ReadIds) -> Vec<Vec<<Self::PgType as #import::pg_type::PgType>::ReadInner>> { 1u8 } },
                quote::quote! { fn read_inner_into_read_with_new_or_try_new_unwraped(v: TestGeneratedHookOwner) -> <Self::PgType as #import::pg_type::PgType>::Read { 2u8 } },
                quote::quote! { fn read_inner_into_update_with_new_or_try_new_unwraped(v: TestGeneratedHookOwner) -> <Self::PgType as #import::pg_type::PgType>::Update { 3u8 } },
                quote::quote! { fn update_to_read_ids(v: &<Self::PgType as #import::pg_type::PgType>::Update) -> <Self::PgType as #import::pg_type::PgType>::ReadIds { 4u8 } },
                quote::quote! { fn read_ids_to_optional_explicit_value_read_default_some_one_element(v: &<Self::PgType as #import::pg_type::PgType>::ReadIds) -> Option<#import::explicit_value::ExplicitValue<<Self::PgType as #import::pg_type::PgType>::Read>> { 5u8 } },
                quote::quote! { fn previous_read_and_optional_update_into_read(read: <Self::PgType as #import::pg_type::PgType>::Read, optional_update: Option<<Self::PgType as #import::pg_type::PgType>::Update>,) -> <Self::PgType as #import::pg_type::PgType>::Read { 6u8 } },
                quote::quote! { fn read_ids_and_create_into_read(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create,) -> <Self::PgType as #import::pg_type::PgType>::Read { 7u8 } },
                quote::quote! { fn read_ids_and_create_into_optional_explicit_value_read(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create,) -> Option<#import::explicit_value::ExplicitValue<<Self::PgType as #import::pg_type::PgType>::Read>> { 8u8 } },
                quote::quote! { fn read_ids_and_create_into_table_type(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create) -> <Self::PgType as #import::pg_type::PgType>::TableType { 9u8 } },
                quote::quote! { fn read_ids_and_create_into_where_eq(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create) -> <Self::PgType as #import::pg_type::PgType>::Where { 10u8 } },
                quote::quote! { fn read_ids_and_create_into_vec_where_eq_using_fields(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create) -> #import::not_empty_unique_vec::NotEmptyUniqueVec<<Self::PgType as #import::pg_type::PgType>::Where> { 11u8 } },
                quote::quote! { fn read_ids_and_create_into_optional_vec_where_eq_to_field(read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, create: <Self::PgType as #import::pg_type::PgType>::Create) -> Option<#import::not_empty_unique_vec::NotEmptyUniqueVec<<Self::PgType as #import::pg_type::PgType>::Where>> { 12u8 } },
                quote::quote! { fn pg_type_optional_vec_where_greater_than_test() -> Option<#import::not_empty_unique_vec::NotEmptyUniqueVec<#import::pg_type_greater_than_test::PgTypeGreaterThanTest<Self::PgType>>> { 13u8 } },
                quote::quote! { fn read_ids_and_table_type_into_pg_type_optional_where_greater_than(greater_than_variant: #import::pg_type_greater_than_variant::PgTypeGreaterThanVariant, read_ids: <Self::PgType as #import::pg_type::PgType>::ReadIds, table_type: <Self::PgType as #import::pg_type::PgType>::TableType,) -> Option<<Self::PgType as #import::pg_type::PgType>::Where> { 14u8 } }
            ].map(syn::parse2::<syn::ImplItemFn>);
            let generated = crate::generate_impl_pg_type_test_cases_for_identifier_token_stream::generate_impl_pg_type_test_cases_for_identifier_token_stream(
                &cfg, &import, &identifier, &identifier,
                    (mask & 1u8 != 0u8).then_some(&bodies[0]),
                &bodies[1], &bodies[2], &bodies[3], &bodies[4], &bodies[5], &bodies[6], &bodies[7], &bodies[8], &bodies[9], &bodies[10], &bodies[11],
                    (mask & 2u8 != 0u8).then_some(&bodies[12]),
                    (mask & 4u8 != 0u8).then_some(&bodies[13]),
                    (mask & 8u8 != 0u8).then_some(&bodies[14]),
            );
            syn::parse2::<syn::ItemImpl>(proc_macro2::TokenStream::from(generated)).is_ok_and(|implementation| {
                let functions = implementation.items.iter().filter_map(|item| {
                    if let syn::ImplItem::Fn(method) = item { Some(method) } else { None }
                }).collect::<Vec<_>>();
                let expected_optional_count = optional_methods.iter().filter(|(_, bit)| mask & bit != 0u8).count();
                functions.len() == 11usize + expected_optional_count
                    && functions.iter().all(|method| {
                        expected_methods.iter().filter_map(|expected| expected.as_ref().ok())
                            .find(|expected| expected.sig.ident == method.sig.ident)
                            .is_some_and(|expected| {
                                method.sig.ident == expected.sig.ident
                                    && method.sig.output == expected.sig.output
                                    && method.sig.generics == expected.sig.generics
                                    && method.sig.inputs.iter().eq(expected.sig.inputs.iter())
                                    && method.block == expected.block
                            })
                    })
                    && implementation.items.iter().filter_map(|item| {
                        if let syn::ImplItem::Type(associated_type) = item { Some(associated_type) } else { None }
                    }).eq([
                        syn::parse_quote! { type PgType = Self; },
                        syn::parse_quote! { type Select = TestGeneratedHookOwnerSelect; },
                    ].iter())
                    && optional_methods.iter().all(|(method_name, bit)| {
                        functions.iter().any(|method| method.sig.ident == method_name.as_str()) == (mask & bit != 0u8)
                    })
                    && implementation.attrs.iter().any(|attribute| quote::quote! { #attribute }.to_string() == cfg.to_string())
                    && quote::ToTokens::to_token_stream(implementation.self_ty.as_ref()).to_string() == identifier.to_string()
                        && implementation.trait_.as_ref().is_some_and(|(path, _)| {
                        path.segments.last().is_some_and(|segment| segment.ident == names.get_pg_type_test_cases_upper_camel_case().to_string())
                            && syn::parse2::<syn::Path>(quote::quote! { #import }).is_ok_and(|root| {
                                path.segments.first().map(|segment| &segment.ident) == root.segments.first().map(|segment| &segment.ident)
                            })
                    })
            })
        })
    }));
}

#[test]
fn test_pg_type_emitter_preserves_associated_types_signatures_and_supplied_bodies() {
    [
        (crate::import::Import::Crate, quote::quote!(crate)),
        (crate::import::Import::PgCrudCommon, quote::quote!(pg_crud_common)),
    ].into_iter().fold((), |(), (import, root)| {
        let generated = crate::generate_impl_pg_type_token_stream::generate_impl_pg_type_token_stream(
            &import, &quote::quote!(TestGeneratedPgType), &quote::quote!(domain::Table),
            &crate::emission_types::IsPrimaryKeyUnderscore::False, &quote::quote!(0u8),
            &quote::quote!(domain::Create),
            &crate::emission_types::CreateQueryPartValueUnderscore::False,
            &crate::emission_types::CreateQueryPartIncrementUnderscore::False, &quote::quote!(1u8),
            &crate::emission_types::CreateQueryBindValueUnderscore::False,
            &crate::emission_types::IsCreateQueryBindMut::False, &quote::quote!(2u8),
            &quote::quote!(domain::Select),
            &crate::emission_types::SelectQueryPartValueUnderscore::False, &quote::quote!(3u8),
            &quote::quote!(domain::Where), &quote::quote!(domain::Read), &quote::quote!(4u8),
            &quote::quote!(domain::ReadIds), &quote::quote!(5u8),
            &quote::quote!(domain::ReadInner), &quote::quote!(6u8),
            &quote::quote!(domain::Update), &quote::quote!(domain::UpdateForQuery),
            &crate::emission_types::UpdateQueryPartValueUnderscore::False,
            &crate::emission_types::UpdateQueryPartAccumulatorUnderscore::False,
            &crate::emission_types::UpdateQueryPartTargetUnderscore::False,
            &crate::emission_types::UpdateQueryPartPathUnderscore::False, &quote::quote!(7u8),
            &crate::emission_types::IsUpdateQueryBindMut::False, &quote::quote!(8u8),
            &quote::quote!(9u8),
            &crate::emission_types::IsSelectOnlyUpdatedIdsQueryBindMut::False, &quote::quote!(10u8),
        );
        let observed_result = syn::parse2::<syn::ItemImpl>(proc_macro2::TokenStream::from(generated));
        assert!(observed_result.is_ok());
        let Ok(observed) = observed_result else { return; };
        assert_eq!(observed.self_ty.as_ref(), &syn::parse_quote!(TestGeneratedPgType));
        assert!(observed.trait_.as_ref().is_some_and(|(path, _)| path == &syn::parse_quote!(#root::pg_type::PgType)));
        assert_eq!(observed.generics, syn::Generics::default());
        let associated_types: [syn::ImplItemType; 9] = [
            syn::parse_quote!(type TableType = domain::Table;),
            syn::parse_quote!(type Create = domain::Create;),
            syn::parse_quote!(type Select = domain::Select;),
            syn::parse_quote!(type Where = domain::Where;),
            syn::parse_quote!(type Read = domain::Read;),
            syn::parse_quote!(type ReadIds = domain::ReadIds;),
            syn::parse_quote!(type ReadInner = domain::ReadInner;),
            syn::parse_quote!(type Update = domain::Update;),
            syn::parse_quote!(type UpdateForQuery = domain::UpdateForQuery;),
        ];
        assert!(observed.items.iter().filter_map(|impl_item| {
            if let syn::ImplItem::Type(associated_type) = impl_item { Some(associated_type) } else { None }
        }).eq(associated_types.iter()));
        let methods: [syn::ImplItemFn; 11] = [
            syn::parse_quote!(fn create_table_column_query_part(column: #root::sql_column_ref::SqlColumnRef<'_>, is_primary_key: #root::pg_is_primary_key::PgIsPrimaryKey) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 0u8 }),
            syn::parse_quote!(fn create_query_part(v: &Self::Create, increment: &mut dyn #root::query_part_increment_mut::QueryPartIncrementMut) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 1u8 }),
            syn::parse_quote!(fn create_query_bind(v: Self::Create, query: #root::sqlx_postgres_query::SqlxPostgresQuery<'_>) -> Result<#root::sqlx_postgres_query::SqlxPostgresQuery<'_>, #root::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> { 2u8 }),
            syn::parse_quote!(fn select_query_part(v: &Self::Select, column: #root::sql_column_ref::SqlColumnRef<'_>) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 3u8 }),
            syn::parse_quote!(fn normalize(v: Self::Read) -> Self::Read { 4u8 }),
            syn::parse_quote!(fn select_only_ids_query_part(column: #root::sql_column_ref::SqlColumnRef<'_>) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 5u8 }),
            syn::parse_quote!(fn into_inner(v: Self::Read) -> Self::ReadInner { 6u8 }),
            syn::parse_quote!(fn update_query_part(v: &Self::UpdateForQuery, update_accumulator: #root::sql_column_ref::SqlColumnRef<'_>, update_target: #root::sql_column_ref::SqlColumnRef<'_>, update_path: #root::sql_column_ref::SqlColumnRef<'_>, increment: &mut dyn #root::query_part_increment_mut::QueryPartIncrementMut) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 7u8 }),
            syn::parse_quote!(fn update_query_bind(v: Self::UpdateForQuery, query: #root::sqlx_postgres_query::SqlxPostgresQuery<'_>) -> Result<#root::sqlx_postgres_query::SqlxPostgresQuery<'_>, #root::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> { 8u8 }),
            syn::parse_quote!(fn select_only_updated_ids_query_part(v: &Self::UpdateForQuery, column: #root::sql_column_ref::SqlColumnRef<'_>, increment: &mut dyn #root::query_part_increment_mut::QueryPartIncrementMut) -> Result<#root::query_part_fragment::QueryPartFragment, #root::query_part_error::QueryPartError> { 9u8 }),
            syn::parse_quote!(fn select_only_updated_ids_query_bind<'lt>(v: &'lt Self::UpdateForQuery, query: #root::sqlx_postgres_query::SqlxPostgresQuery<'lt>) -> Result<#root::sqlx_postgres_query::SqlxPostgresQuery<'lt>, #root::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError> { 10u8 }),
        ];
        let observed_methods = observed.items.iter().filter_map(|impl_item| {
            if let syn::ImplItem::Fn(method) = impl_item { Some(method) } else { None }
        });
        assert_eq!(observed.items.len(), associated_types.len() + methods.len());
        assert_eq!(observed_methods.clone().count(), methods.len());
        assert!(observed_methods.zip(methods.iter()).all(|(observed_method, expected_method)| {
            observed_method.sig.ident == expected_method.sig.ident
                && observed_method.sig.generics == expected_method.sig.generics
                && observed_method.sig.output == expected_method.sig.output
                && observed_method.sig.inputs.iter().eq(expected_method.sig.inputs.iter())
                && observed_method.block == expected_method.block
        }));
    });
}
