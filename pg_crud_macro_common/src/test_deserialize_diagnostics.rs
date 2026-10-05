#[test]
fn test_deserialize_diagnostic_literals_preserve_identifier_and_element_count() {
    assert!([stringify!(TestDeserializeDiagnostic), stringify!(r#type)]
        .into_iter()
        .all(|identifier_text| {
            let Ok(identifier) = syn::parse_str::<syn::Ident>(identifier_text) else {
                return false;
            };
            [0usize, 1usize, usize::MAX].into_iter().all(|length| {
                let (structure, counted_structure, identifier_literal) =
                    crate::generate_deserialize_double_quoted_token_stream::generate_deserialize_double_quoted_token_stream(
                        &identifier,
                        crate::deserialize_length::DeserializeLength::from(length),
                    );
                let expected_structure = format!("{} {identifier}", constants_str::STRUCT);
                let expected_counted_structure = format!(
                    "{} {identifier} {} {length} {}",
                    constants_str::STRUCT,
                    stringify!(with),
                    stringify!(elements),
                );
                [
                    (structure, expected_structure),
                    (counted_structure, expected_counted_structure),
                    (identifier_literal, identifier.to_string()),
                ].into_iter().all(|(generated, expected)| {
                    syn::parse2::<syn::LitStr>(proc_macro2::TokenStream::from(generated))
                        .is_ok_and(|literal| literal.value() == expected)
                })
            })
        }));
}

#[test]
fn test_field_deserializer_applies_type_mapping_in_declaration_order() {
    let Ok(source) = syn::parse2::<syn::ItemStruct>(quote::quote! {
        struct TestMappedDeserializer { first: i64, second: String }
    }) else {
        std::panic::panic_any(constants_str::PANIC_EDC94D17);
    };
    let source_fields = source
        .fields
        .into_iter()
        .map(|field| {
            field.ident.map(|identifier| {
                macro_helpers::syn_field::SynField::new(
                    macro_helpers::syn_field_identifier::SynFieldIdentifier::from(identifier),
                    macro_helpers::syn_field_type::SynFieldType::from(field.ty),
                    macro_helpers::syn_field_vis::SynFieldVis::from(field.vis),
                )
            })
        })
        .collect::<Option<Vec<_>>>();
    assert!(source_fields.is_some_and(|fields| {
        let generated = crate::generate_impl_deserialize_for_struct_by_fields_token_stream::generate_impl_deserialize_for_struct_by_fields_token_stream(
            &source.ident,
            crate::syn_field_refs::SynFieldRefs::from(fields.as_slice()),
            crate::deserialize_length::DeserializeLength::from(fields.len()),
            &|identifier, field_type| quote::quote! { Option<(#field_type, #identifier)> }.into(),
        );
        let Ok(file) = syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)) else {
            return false;
        };
        let Some(syn::Item::Struct(raw)) = file.items.first() else {
            return false;
        };
        raw.ident == stringify!(TestMappedDeserializerRaw)
            && raw.fields.len() == 2usize
            && raw.fields.iter().zip([
                (stringify!(first), quote::quote! { Option<(i64, first)> }),
                (stringify!(second), quote::quote! { Option<(String, second)> }),
            ]).all(|(field, (name, expected_type))| {
                matches!(field.vis, syn::Visibility::Inherited)
                    && field.ident.as_ref().is_some_and(|identifier| identifier == name)
                    && quote::quote! { #expected_type }.to_string()
                        == {
                            let field_type = &field.ty;
                            quote::quote! { #field_type }.to_string()
                        }
            })
    }));
}
#[test]
fn test_identifier_type_deserializer_preserves_fields_and_validated_constructor_order() {
    let parsed_source = syn::parse2::<syn::ItemStruct>(quote::quote! {
        struct TestIdentifierTypeDeserializer { first: i64, second: String }
    });
    assert!(parsed_source.is_ok_and(|source| {
    let field_refs = source
        .fields
        .iter()
        .map(|field| {
            field
                .ident
                .as_ref()
                .map(|identifier| (identifier, &field.ty))
        })
        .collect::<Option<Vec<_>>>();
    field_refs.is_some_and(|fields| {
        let generated = crate::generate_impl_deserialize_for_struct_token_stream::generate_impl_deserialize_for_struct_token_stream(
            &source.ident,
            crate::syn_identifier_type_refs::SynIdentifierTypeRefs::from(fields.as_slice()),
            crate::deserialize_length::DeserializeLength::from(fields.len()),
            &|identifier, field_type| quote::quote! { Option<(#field_type, #identifier)> }.into(),
        );
        let Ok(file) = syn::parse2::<syn::File>(proc_macro2::TokenStream::from(generated)) else { return false; };
        let Some(syn::Item::Struct(raw)) = file.items.first() else { return false; };
        let error_format = syn::LitStr::new(&format!("{{{}:?}}", stringify!(error)), proc_macro2::Span::call_site());
        let implementation = file.items.iter().find_map(|item| {
            let syn::Item::Const(item_const) = item else { return None; };
            let syn::Expr::Block(block) = item_const.expr.as_ref() else { return None; };
            block.block.stmts.iter().find_map(|statement| {
                if let syn::Stmt::Item(syn::Item::Impl(item_impl)) = statement { Some(item_impl) } else { None }
            })
        });
        raw.ident == stringify!(TestIdentifierTypeDeserializerRaw)
            && raw.fields.len() == 2usize
            && raw.fields.iter().zip([
                (stringify!(first), quote::quote! { Option<(i64, first)> }),
                (stringify!(second), quote::quote! { Option<(String, second)> }),
            ]).all(|(field, (name, expected_type))| {
                let actual_type = &field.ty;
                matches!(field.vis, syn::Visibility::Inherited)
                    && field.ident.as_ref().is_some_and(|identifier| identifier == name)
                    && quote::quote! { #actual_type }.to_string() == expected_type.to_string()
            })
            && implementation.is_some_and(|item_impl| {
                item_impl.items.iter().any(|item| {
                    let syn::ImplItem::Fn(method) = item else { return false; };
                    let Some(syn::Stmt::Expr(syn::Expr::MethodCall(error_mapping), None)) = method.block.stmts.last() else { return false; };
                    let syn::Expr::Call(constructor) = error_mapping.receiver.as_ref() else { return false; };
                    method.sig.ident == stringify!(deserialize)
                        && error_mapping.method == stringify!(map_err)
                        && constructor.args.len() == 2usize
                        && {
                            let constructor_path = &constructor.func;
                            quote::quote! { #constructor_path }.to_string() == quote::quote! { Self::try_new }.to_string()
                        }
                        && constructor.args.iter().zip([quote::quote! { raw.first }, quote::quote! { raw.second }])
                            .all(|(argument, expected_argument)| quote::quote! { #argument }.to_string() == expected_argument.to_string())
                        && error_mapping.args.len() == 1usize
                        && error_mapping.args.iter().all(|argument| quote::quote! { #argument }.to_string()
                            == quote::quote! { |error| _serde::de::Error::custom(format!(#error_format)) }.to_string())
                })
            })
    })
    }));
}
