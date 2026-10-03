#[test]
fn test_table_positive_bulk_limits_emit_independent_schema_and_deserializer_bounds() {
    assert!([
        (None, None),
        (Some(1usize), None),
        (None, Some(2usize)),
        (Some(3usize), Some(4usize)),
    ].into_iter().all(|(create_limit, update_limit)| {
        let create_configuration = create_limit.map(|limit| {
            let literal = proc_macro2::Literal::usize_unsuffixed(limit);
            quote::quote! { , "create_many_max_items": #literal }
        });
        let update_configuration = update_limit.map(|limit| {
            let literal = proc_macro2::Literal::usize_unsuffixed(limit);
            quote::quote! { , "update_many_max_items": #literal }
        });
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
                #create_configuration #update_configuration
            })]
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| {
            [
                (stringify!(TableCreateManyPayload), quote::quote! { TableCreate }, create_limit),
                (stringify!(TableUpdatePayload), quote::quote! { TableUpdate }, update_limit),
            ].into_iter().all(|(payload_name, element_type, limit)| {
                let Some(payload) = file.items.iter().find_map(|item| {
                    let syn::Item::Struct(payload) = item else { return None; };
                    (payload.ident == payload_name).then_some(payload)
                }) else { return false; };
                let syn::Fields::Unnamed(fields) = &payload.fields else { return false; };
                let Some(field) = fields.unnamed.first() else { return false; };
                assert_eq!(fields.unnamed.len(), 1usize);
                let expected_schema = limit.map(|limit_value| quote::quote! { #[schema(max_items = #limit_value)] }.to_string());
                assert!(field.attrs.iter().filter(|attribute| attribute.path().is_ident(stringify!(schema)))
                    .map(|attribute| quote::quote! { #attribute }.to_string()).eq(expected_schema));
                let deserialize_derived = payload.attrs.iter().filter(|attribute| attribute.path().is_ident(stringify!(derive)))
                    .any(|attribute| attribute.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::token::Comma>::parse_terminated)
                        .is_ok_and(|paths| paths.iter().any(|path| path.segments.last().is_some_and(|segment| segment.ident == stringify!(Deserialize)))));
                assert_eq!(deserialize_derived, payload_name == stringify!(TableCreateManyPayload) && limit.is_none());
                limit.map_or_else(
                    || !output.contains(&quote::quote! { PgBoundedVec<#element_type }.to_string()),
                    |limit_value| output.contains(&quote::quote! { pg_crud_common::pg_bounded_vec::PgBoundedVec<#element_type, 0usize, #limit_value> }.to_string()),
                )
            })
        })
    }));
}

#[test]
fn test_table_config_deserialization_rejects_identifier_and_path_byte_overflows() {
    let oversized_identifier = proc_macro2::Literal::string(&constants_str::X.repeat(64usize));
    let oversized_utf8_identifier =
        proc_macro2::Literal::string(&'\u{00e9}'.to_string().repeat(32usize));
    let oversized_path = proc_macro2::Literal::string(&constants_str::X.repeat(513usize));
    let cases = [
        quote::quote! { "route_resource_name": #oversized_identifier },
        quote::quote! { "route_resource_name": #oversized_utf8_identifier },
        quote::quote! { "db_column_type_overrides": [{"column": #oversized_identifier, "data_type": "jsonb"}] },
        quote::quote! { "create_exclude_fields": [""] },
        quote::quote! { "read_exclude_fields": [""] },
        quote::quote! { "create_exclude_fields": [#oversized_identifier] },
        quote::quote! { "read_exclude_fields": [#oversized_identifier] },
        quote::quote! { "read_page": {"search_columns": [], "response": #oversized_path, "enrich": "enrich", "error": "Error"} },
        quote::quote! { "read_page": {"search_columns": [], "response": "Response", "enrich": "", "error": "Error"} },
    ];
    assert!(cases.into_iter().all(|configuration| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False", #configuration
            })]
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac.path.is_ident(stringify!(compile_error))
                && syn::parse2::<syn::LitStr>(item.mac.tokens).is_ok_and(|message| message.value().contains(stringify!(GeneratePgTableConfig)))
        })
    }));
}

#[test]
fn test_table_config_accepts_exact_identifier_exclusion_and_rust_path_limits() {
    let identifier_text = constants_str::X.repeat(63usize);
    let identifier_literal = proc_macro2::Literal::string(&identifier_text);
    let field_identifier =
        proc_macro2::Ident::new(&identifier_text, proc_macro2::Span::call_site());
    let path_text = constants_str::X.repeat(512usize);
    let path_literal = proc_macro2::Literal::string(&path_text);
    let response_identifier = proc_macro2::Ident::new(&path_text, proc_macro2::Span::call_site());
    let input = quote::quote! {
        #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
            "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False",
            "route_resource_name": #identifier_literal,
            "create_exclude_fields": [#identifier_literal],
            "read_exclude_fields": [#identifier_literal],
            "db_column_type_overrides": [{"column": #identifier_literal, "data_type": "jsonb"}],
            "read_page": {"search_columns": [], "response": #path_literal, "enrich": "enrich", "error": "Error"}
        })]
        struct Table {
            #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
            #field_identifier: StringAsNonNullText,
            name: StringAsNonNullText
        }
    };
    let generated = crate::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    );
    let output = generated.to_string();
    assert!(!output.contains(stringify!(compile_error)));
    assert!(output.contains(&quote::quote! { #response_identifier }.to_string()));
    assert!(output.contains(&quote::quote! { pg_crud_common::db_static_schema_text::DbStaticSchemaText::from(#identifier_literal) }.to_string()));
    let mut route_path = '/'.to_string();
    route_path.push_str(&identifier_text);
    route_path.push('/');
    route_path.push_str(stringify!(create_many));
    let route_literal = proc_macro2::Literal::string(&route_path);
    assert!(
        output
            .contains(&quote::quote! { TableOperation::CreateMany => #route_literal }.to_string())
    );
    assert!(
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated))
            .is_ok_and(|file| !file.items.is_empty())
    );
}
