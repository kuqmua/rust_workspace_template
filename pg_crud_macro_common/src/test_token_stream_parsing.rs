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
            let body = quote::quote! { test_hook_fixture_body() };
            let generated = crate::generate_impl_pg_type_test_cases_for_identifier_token_stream::generate_impl_pg_type_test_cases_for_identifier_token_stream(
                &cfg, &import, &identifier, &identifier,
                    (mask & 1u8 != 0u8).then_some(&body),
                &body, &body, &body, &body, &body, &body, &body, &body, &body, &body, &body,
                    (mask & 2u8 != 0u8).then_some(&body),
                    (mask & 4u8 != 0u8).then_some(&body),
                    (mask & 8u8 != 0u8).then_some(&body),
            );
            syn::parse2::<syn::ItemImpl>(proc_macro2::TokenStream::from(generated)).is_ok_and(|implementation| {
                let functions = implementation.items.iter().filter_map(|item| {
                    if let syn::ImplItem::Fn(method) = item { Some(method) } else { None }
                }).collect::<Vec<_>>();
                let expected_optional_count = optional_methods.iter().filter(|(_, bit)| mask & bit != 0u8).count();
                functions.len() == 11usize + expected_optional_count
                    && functions.iter().all(|method| quote::ToTokens::to_token_stream(&method.block).to_string() == quote::quote! { { #body } }.to_string())
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
