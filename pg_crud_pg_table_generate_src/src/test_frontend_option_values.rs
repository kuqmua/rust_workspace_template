#[test]
fn test_table_frontend_value_syntax_errors_emit_only_compile_errors() {
    let cases = [
        quote::quote! { label },
        quote::quote! { placeholder },
        quote::quote! { order },
        quote::quote! { label = 0 },
        quote::quote! { placeholder = false },
        quote::quote! { order = "first" },
        quote::quote! { order = -1 },
        quote::quote! { order = 340282366920938463463374607431768211455 },
    ];
    assert!(cases.into_iter().all(|options| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                #[generate_pg_table_frontend(#options)] name: StringAsNonNullText
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac.path.segments.last().is_some_and(|segment| segment.ident == stringify!(compile_error))
                && syn::parse2::<syn::LitStr>(item.mac.tokens).is_ok_and(|message| !message.value().is_empty())
        })
    }));
}

#[test]
fn test_table_frontend_labels_and_placeholders_preserve_nonempty_label_whitespace() {
    let cases = [
        (quote::quote! { "  Friendly  " }, quote::quote! { "" }),
        (quote::quote! { "Friendly" }, quote::quote! { " " }),
    ];
    assert!(cases.into_iter().all(|(label, placeholder)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                #[generate_pg_table_frontend(label = #label, placeholder = #placeholder)] name: StringAsNonNullText
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        assert!(output.contains(&quote::quote! { frontend_contract::field_label::FieldLabel::from(frontend_contract::contract_str::ContractStr::from(#label)) }.to_string()));
        assert!(output.contains(&quote::quote! { frontend_contract::field_placeholder::FieldPlaceholder::Value(frontend_contract::contract_str::ContractStr::from(#placeholder)) }.to_string()));
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| !file.items.is_empty())
    }));
}

#[test]
fn test_table_frontend_sparse_order_keeps_hidden_fields_and_accepts_large_values() {
    assert!(usize::try_from(u32::MAX).is_ok_and(|large_order| {
        let order_literal = proc_macro2::Literal::usize_unsuffixed(large_order);
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
            })]
            struct Table {
                #[generate_pg_table_primary_key] id: I64AsNonNullInt8,
                #[generate_pg_table_frontend(hidden, order = #order_literal)] name: StringAsNonNullText,
                #[generate_pg_table_frontend(order = 5)] created_at: StringAsNonNullText
            }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        let emitted_order = proc_macro2::Literal::usize_suffixed(large_order);
        assert!(output.contains(&quote::quote! { .with_order(frontend_contract::field_order::FieldOrder::from(#emitted_order)) }.to_string()));
        let primary_field = quote::quote! { frontend_contract::field_name::FieldName::from(frontend_contract::contract_str::ContractStr::from("id")) };
        let middle_field = quote::quote! { frontend_contract::field_name::FieldName::from(frontend_contract::contract_str::ContractStr::from("created_at")) };
        let hidden_field = quote::quote! { frontend_contract::field_name::FieldName::from(frontend_contract::contract_str::ContractStr::from("name")) };
        match (
            output.find(&primary_field.to_string()),
            output.find(&middle_field.to_string()),
            output.find(&hidden_field.to_string()),
        ) {
            (Some(primary_position), Some(middle_position), Some(hidden_position)) => {
                primary_position < middle_position && middle_position < hidden_position
            }
            _ => false,
        }
    }));
}
