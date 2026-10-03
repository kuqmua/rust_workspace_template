#[test]
fn test_table_error_annotations_keep_operation_variants_separate_and_common_variants_shared() {
    let input = quote::quote! {
        #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
            "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
        })]
        #[proc_macro_generate_pg_table::common_error_variants(enum CommonErrorVariants {
            CustomCommonFailure {
                #[error_field_to_err_string] source: macro_helpers::std_tool_io_error::StdToolIoError,
                location: location_lib::location::Location
            }
        })]
        #[proc_macro_generate_pg_table::create_many_error_variants(enum CreateManyErrorVariants {
            CustomCreateFailure { #[error_field_to_err_string] source: macro_helpers::std_tool_io_error::StdToolIoError }
        })]
        #[proc_macro_generate_pg_table::read_many_error_variants(enum ReadManyErrorVariants {
            CustomReadFailure { #[error_field_to_err_string] source: macro_helpers::std_tool_io_error::StdToolIoError }
        })]
        #[proc_macro_generate_pg_table::update_many_error_variants(enum UpdateManyErrorVariants {
            CustomUpdateFailure { #[error_field_to_err_string] source: macro_helpers::std_tool_io_error::StdToolIoError }
        })]
        #[proc_macro_generate_pg_table::delete_many_error_variants(enum DeleteManyErrorVariants {
            CustomDeleteFailure { #[error_field_to_err_string] source: macro_helpers::std_tool_io_error::StdToolIoError }
        })]
        struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
    };
    let generated = crate::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
    );
    assert!(!generated.to_string().contains(stringify!(compile_error)));
    assert!(syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| {
        let operation_variants = [
            stringify!(CustomCreateFailure), stringify!(CustomReadFailure),
            stringify!(CustomUpdateFailure), stringify!(CustomDeleteFailure),
        ];
        [
            (stringify!(TableCreateManyError), stringify!(CustomCreateFailure)),
            (stringify!(TableReadError), stringify!(CustomReadFailure)),
            (stringify!(TableUpdateError), stringify!(CustomUpdateFailure)),
            (stringify!(TableDeleteManyError), stringify!(CustomDeleteFailure)),
        ].into_iter().all(|(error_name, expected_variant)| {
            let Some(error_enum) = file.items.iter().find_map(|item| {
                let syn::Item::Enum(error_enum) = item else { return None; };
                (error_enum.ident == error_name).then_some(error_enum)
            }) else { return false; };
            assert_eq!(error_enum.variants.iter().filter(|variant| variant.ident == stringify!(CustomCommonFailure)).count(), 1usize);
            assert!(error_enum.variants.iter().filter(|variant| operation_variants.iter().any(|name| variant.ident == *name))
                .map(|variant| variant.ident.to_string()).eq([expected_variant]));
            error_enum.variants.iter().filter(|variant| variant.ident == expected_variant || variant.ident == stringify!(CustomCommonFailure)).all(|variant| {
                let syn::Fields::Named(fields) = &variant.fields else { return false; };
                fields.named.iter().any(|field| field.ident.as_ref().is_some_and(|identifier| identifier == stringify!(source))
                    && { let field_type = &field.ty;
                        quote::quote! { #field_type }.to_string() == quote::quote! { macro_helpers::std_tool_io_error::StdToolIoError }.to_string()
                    })
            })
        })
    }));
}

#[test]
fn test_table_logic_annotations_forward_each_operation_and_common_body() {
    let cases = [
        quote::quote! { #[proc_macro_generate_pg_table::create_many_logic(let custom_create_marker = 1usize;)] },
        quote::quote! { #[proc_macro_generate_pg_table::read_many_logic(let custom_read_marker = 1usize;)] },
        quote::quote! { #[proc_macro_generate_pg_table::update_many_logic(let custom_update_marker = 1usize;)] },
        quote::quote! { #[proc_macro_generate_pg_table::delete_many_logic(let custom_delete_marker = 1usize;)] },
        quote::quote! { #[proc_macro_generate_pg_table::common_logic(let custom_common_marker = 1usize;)] },
    ];
    let markers = [
        quote::quote! { let custom_create_marker = 1usize; },
        quote::quote! { let custom_read_marker = 1usize; },
        quote::quote! { let custom_update_marker = 1usize; },
        quote::quote! { let custom_delete_marker = 1usize; },
        quote::quote! { let custom_common_marker = 1usize; },
    ];
    let operations = [
        Some(stringify!(create_many)),
        Some(stringify!(read)),
        Some(stringify!(update)),
        Some(stringify!(delete_many)),
        None,
    ];
    assert!(cases.into_iter().zip(markers).zip(operations).all(|((attribute, marker), operation)| {
        let input = quote::quote! {
            #[proc_macro_generate_pg_table_generate_pg_table_config::generate_pg_table_config({
                "tests_write_into_file": "False", "common_write_into_file": "False", "whole_write_into_file": "False"
            })]
            #attribute
            struct Table { #[generate_pg_table_primary_key] id: I64AsNonNullInt8, name: StringAsNonNullText }
        };
        let generated = crate::generate_pg_table::generate_pg_table(
            macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input),
        );
        let output = generated.to_string();
        assert!(!output.contains(stringify!(compile_error)));
        assert!(output.contains(&marker.to_string()));
        syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)).is_ok_and(|file| {
            let methods = file.items.iter().filter_map(|item| {
                let syn::Item::Impl(implementation) = item else { return None; };
                let syn::Type::Path(self_type) = implementation.self_ty.as_ref() else { return None; };
                self_type.path.is_ident(stringify!(Table)).then_some(implementation)
            }).flat_map(|implementation| implementation.items.iter()).filter_map(|item| {
                let syn::ImplItem::Fn(method) = item else { return None; };
                let block = &method.block;
                quote::quote! { #block }.to_string().contains(&marker.to_string()).then(|| method.sig.ident.to_string())
            }).collect::<Vec<_>>();
            operation.map_or_else(
                || methods.len() == 4usize && [stringify!(create_many), stringify!(read), stringify!(update), stringify!(delete_many)].into_iter().all(|operation_prefix| methods.iter().filter(|name| name.starts_with(operation_prefix)).count() == 1usize),
                |operation_prefix| methods.len() == 1usize && methods.iter().all(|name| name.starts_with(operation_prefix)),
            )
        })
    }));
}
