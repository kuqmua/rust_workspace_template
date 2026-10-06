#[cfg(test)]
mod tests {
    #[test]
    fn test_first_identifier_parser_preserves_nested_groups_and_outer_remainder() {
        let transparent =
            |proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens| {
                proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                    proc_macro2::Delimiter::None,
                    proc_macro2_macro_tokens.into_inner(),
                ))
            };
        let inner = transparent(quote::quote! { inner ignored }.into());
        let middle = transparent(quote::quote! { #inner discarded }.into());
        let outer = transparent(quote::quote! { #middle ignored }.into());
        let empty_group = transparent(proc_macro2::TokenStream::new().into());
        let unicode_name = '\u{00e9}'.to_string();
        let unicode_identifier =
            proc_macro2::Ident::new(unicode_name.as_str(), proc_macro2::Span::call_site());
        let oversized_name =
            constants_str::X.repeat(crate::first_ident_max_len::FIRST_IDENT_MAX_LEN + 1usize);
        let oversized_identifier =
            proc_macro2::Ident::new(oversized_name.as_str(), proc_macro2::Span::call_site());
        [
            (quote::quote! { #oversized_identifier }, None),
            (
                quote::quote! { #unicode_identifier },
                Some(unicode_name.as_str()),
            ),
            (quote::quote! { value }, Some(stringify!(value))),
            (quote::quote! { r#type }, Some(stringify!(r#type))),
            (quote::quote! { #outer }, Some(stringify!(inner))),
            (quote::quote! { #empty_group }, None),
            (quote::quote! { (value) }, None),
            (quote::quote! { [value] }, None),
            (quote::quote! { { value } }, None),
            (quote::quote! { + }, None),
            (quote::quote! { 1 }, None),
        ]
        .into_iter()
        .fold((), |(), (first, expected)| {
            let mut tokens = quote::quote! { #first following }.into_iter();
            assert_eq!(
                crate::parse_first_identifier::parse_first_identifier(&mut tokens)
                    .map(|first_identifier| first_identifier.to_string()),
                expected.map(str::to_owned)
            );
            assert_eq!(
                crate::parse_first_identifier::parse_first_identifier(&mut tokens)
                    .map(|first_identifier| first_identifier.to_string()),
                Some(stringify!(following).to_owned())
            );
            assert!(crate::parse_first_identifier::parse_first_identifier(&mut tokens).is_none());
        });
        let mut empty = proc_macro2::TokenStream::new().into_iter();
        assert!(crate::parse_first_identifier::parse_first_identifier(&mut empty).is_none());
    }

    #[test]
    fn test_indexed_identifier_selection_preserves_parts_and_out_of_range_results() {
        let parts =
            crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts::try_from(vec![
                quote::quote! { first trailing },
                proc_macro2::TokenStream::new(),
                quote::quote! { + rejected },
                quote::quote! { r#type },
            ]);
        assert!(parts.is_ok());
        if let Ok(proc_macro2_top_level_comma_parts) = parts {
            let original = proc_macro2_top_level_comma_parts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            [
                Some(stringify!(first)),
                None,
                None,
                Some(stringify!(r#type)),
                None,
            ]
            .into_iter()
            .enumerate()
            .fold((), |(), (index, expected)| {
                let part_index = crate::part_index::PartIndex::from(index);
                assert_eq!(
                    crate::first_identifier_at::first_identifier_at(
                        &proc_macro2_top_level_comma_parts,
                        part_index
                    )
                    .map(|first_identifier| first_identifier.to_string()),
                    expected.map(str::to_owned)
                );
                assert_eq!(
                    crate::part_at::part_at(&proc_macro2_top_level_comma_parts, part_index)
                        .is_some(),
                    index < original.len()
                );
            });
            assert!(
                proc_macro2_top_level_comma_parts
                    .iter()
                    .map(ToString::to_string)
                    .eq(original)
            );
        }
    }

    #[test]
    fn test_first_identifier_text_byte_limits_preserve_content_and_error_fallback() {
        let maximum = crate::first_ident_max_len::FIRST_IDENT_MAX_LEN;
        [
            constants_str::EMPTY,
            stringify!(r#type),
            constants_str::NON_ASCII_U_E9,
            constants_str::TEST_TEXT_WITH_NUL,
        ]
        .into_iter()
        .fold((), |(), text| {
            assert!(
                crate::first_identifier::FirstIdentifier::try_from(text.to_owned())
                    .is_ok_and(|first_identifier| first_identifier.to_string() == text)
            );
        });
        let mut exact = constants_str::X.repeat(maximum - '\u{00e9}'.len_utf8());
        exact.push('\u{00e9}');
        assert!(
            crate::first_identifier::FirstIdentifier::try_from(exact).is_ok_and(
                |first_identifier| {
                    let output = first_identifier.to_string();
                    output.len() == maximum && output.ends_with('\u{00e9}')
                }
            )
        );
        let oversized_length = maximum + '\u{00e9}'.len_utf8();
        let mut oversized = constants_str::X.repeat(maximum);
        oversized.push('\u{00e9}');
        let observed_length = oversized_length.to_string();
        let expected_maximum = maximum.to_string();
        assert!(crate::first_identifier::FirstIdentifier::try_from(oversized).is_err_and(|error| {
            error == crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError::from(oversized_length)
                && error.to_string().split_ascii_whitespace().eq([stringify!(first), stringify!(identifier), stringify!(length), observed_length.as_str(), stringify!(exceeds), stringify!(maximum), expected_maximum.as_str()])
                && crate::first_identifier::FirstIdentifier::from(error).to_string() == error.to_string()
        }));
    }

    #[test]
    fn test_comma_parts_constructor_enforces_exact_collection_limit() {
        let validate = |part_index: crate::part_index::PartIndex| {
            crate::proc_macro2_top_level_comma_parts::ProcMacro2TopLevelCommaParts::try_from(
                std::iter::repeat_with(proc_macro2::TokenStream::new)
                    .take(part_index.get())
                    .collect::<Vec<_>>(),
            )
        };
        let maximum = crate::collection_max_len::COLLECTION_MAX_LEN;
        assert!(
            matches!(validate(crate::part_index::PartIndex::from(maximum)), Ok(parts) if parts.len() == maximum)
        );
        assert!(matches!(
            validate(crate::part_index::PartIndex::from(maximum + constants_usize::ONE)),
            Err(error) if !error.to_string().is_empty()
        ));
    }
    #[test]
    fn test_closure_parser_rejects_unsupported_parameters() {
        assert!(
            [
                quote::quote! { |left, right| left },
                quote::quote! { || value },
                quote::quote! { |value: u8| value },
                quote::quote! { |mut value| value },
            ]
            .into_iter()
            .all(|tokens| {
                crate::closure_identifier_and_body::closure_identifier_and_body(
                    crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(tokens),
                )
                .is_none()
            })
        );
    }
    #[test]
    fn test_identifier_parser_enforces_exact_text_limit() {
        let validate = |part_index: crate::part_index::PartIndex| {
            let text = constants_str::X.repeat(part_index.get());
            let identifier = proc_macro2::Ident::new(text.as_str(), proc_macro2::Span::call_site());
            let mut tokens = quote::quote! { #identifier }.into_iter();
            let parsed = crate::parse_first_identifier::parse_first_identifier(&mut tokens);
            let closure = crate::closure_identifier_and_body::closure_identifier_and_body(
                crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                    quote::quote! { |#identifier| value },
                ),
            );
            assert_eq!(closure.is_some(), parsed.is_some());
            parsed
        };
        let maximum = crate::first_ident_max_len::FIRST_IDENT_MAX_LEN;
        assert!(
            validate(crate::part_index::PartIndex::from(maximum))
                .is_some_and(|identifier| identifier.to_string().len() == maximum)
        );
        assert!(
            validate(crate::part_index::PartIndex::from(
                maximum + constants_usize::ONE
            ))
            .is_none()
        );
    }
    #[test]
    fn test_bool_enum_preserves_delimiter_inside_string_literal() {
        let delimiter = constants_str::TRUE_FAT_ARROW;
        let generated = crate::generate_bool_enum_to_tokens::generate_bool_enum_to_tokens(
            quote::quote! {
                BoolEnumLiteralFixture,
                false => quote::quote! { #delimiter },
                true => quote::quote! { value }
            }
            .into(),
        );
        assert!(
            matches!(syn::parse2::<syn::File>(generated.into_inner()), Ok(file)
            if matches!(file.items.as_slice(), [syn::Item::Enum(_), syn::Item::Impl(_)]))
        );
    }
    #[test]
    fn test_bool_enum_rejects_reversed_branch_keywords() {
        assert!(
            [
                quote::quote! { InvalidBoolEnumFixture, true => value, true => other },
                quote::quote! { InvalidBoolEnumFixture, false => value, false => other },
            ]
            .into_iter()
            .all(|tokens| {
                let generated = crate::generate_bool_enum_to_tokens::generate_bool_enum_to_tokens(
                    tokens.into(),
                );
                matches!(syn::parse2::<syn::File>(generated.into_inner()), Ok(file)
                if matches!(file.items.as_slice(), [syn::Item::Macro(_)]))
            })
        );
    }
    #[test]
    fn test_trait_alias_rejects_invalid_names_without_panicking() {
        let expected = crate::compile_error_token_stream::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_079,
        );
        assert!(
            [
                quote::quote! { Name Extra = std::fmt::Debug },
                quote::quote! { 42 = std::fmt::Debug },
                quote::quote! { (Name) = std::fmt::Debug },
                quote::quote! { Name std::fmt::Debug },
                quote::quote! { = std::fmt::Debug },
            ]
            .into_iter()
            .all(|tokens| {
                crate::generate_trait_alias::generate_trait_alias(tokens.into()).to_string()
                    == expected.to_string()
            })
        );
    }
    #[test]
    fn test_trait_alias_preserves_raw_names_and_associated_type_bounds() {
        let generated = crate::generate_trait_alias::generate_trait_alias(
            quote::quote! {
                r#TraitAliasFixture = Iterator<Item = u8> + std::fmt::Debug
            }
            .into(),
        );
        let expected = quote::quote! {
            pub trait r#TraitAliasFixture: Iterator<Item = u8> + std::fmt::Debug {}
            impl<T: Iterator<Item = u8> + std::fmt::Debug> r#TraitAliasFixture for T {}
        };
        assert_eq!(generated.to_string(), expected.to_string());
        assert!(
            matches!(syn::parse2::<syn::File>(generated.into_inner()), Ok(file)
            if matches!(file.items.as_slice(), [syn::Item::Trait(_), syn::Item::Impl(_)]))
        );
    }
    #[test]
    fn test_trait_alias_accepts_forwarded_name_without_delimiters() {
        let name = proc_macro2::Group::new(
            proc_macro2::Delimiter::None,
            quote::quote! { ForwardedTraitAliasFixture },
        );
        let generated = crate::generate_trait_alias::generate_trait_alias(
            quote::quote! { #name = std::fmt::Debug }.into(),
        );
        assert!(
            matches!(syn::parse2::<syn::File>(generated.into_inner()), Ok(file)
            if matches!(file.items.as_slice(), [syn::Item::Trait(_), syn::Item::Impl(_)]))
        );
    }
    #[test]
    fn test_struct_shape_preserves_named_tuple_and_unit_forms() {
        let named = syn::parse_quote!(
            struct Named {
                value: u8,
            }
        );
        let tuple = syn::parse_quote!(
            struct Tuple(u8);
        );
        let unit = syn::parse_quote!(
            struct Unit;
        );
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&named),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Named(view))
            if matches!(&named.data, syn::Data::Struct(data)
                if matches!(&data.fields, syn::Fields::Named(fields)
                    if std::ptr::eq(view.get(), fields)
                        && std::ptr::eq(&raw const *view, fields)
                        && view.named.len() == 1usize
                        && view.named.first().and_then(|field| field.ident.as_ref()).is_some_and(|identifier| identifier == stringify!(value))))
        ));
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&tuple),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Tuple(view))
            if matches!(&tuple.data, syn::Data::Struct(data)
                if matches!(&data.fields, syn::Fields::Unnamed(fields)
                    if std::ptr::eq(view.get(), fields)
                        && std::ptr::eq(&raw const *view, fields)
                        && view.unnamed.len() == 1usize
                        && view.unnamed.first().is_some_and(|field| field.ident.is_none())))
        ));
        assert!(matches!(
            crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&unit),
            Ok(crate::syn_struct_shape_ref::SynStructShapeRef::Unit)
        ));
        [
            syn::parse_quote! { enum ShapeRejectionFixture { Value } },
            syn::parse_quote! { union ShapeUnionRejectionFixture { value: u8 } },
        ]
        .into_iter()
        .fold((), |(), derive_input| {
            assert!(
                crate::syn_struct_shape_ref::SynStructShapeRef::try_from(&derive_input)
                    .is_err_and(|error| error.to_string() == constants_str::EXPECTED_A_STRUCT)
            );
        });
    }
    #[test]
    fn test_split_top_level_commas_keeps_generic_type_commas_inside_part() {
        let parts = crate::split_top_level_commas::split_top_level_commas(quote::quote! {
            Vec<Result<A, B>>,
            Option<C>
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some(constants_str::VALUE_4ECF628C.to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some(constants_str::VALUE_E516A36D.to_owned())
        );
    }
    #[test]
    fn test_split_top_level_commas_keeps_fat_arrow_pair_as_single_part() {
        let parts = crate::split_top_level_commas::split_top_level_commas(quote::quote! {
            SomeType => "message",
            OtherType => format!("{}" , value)
        });
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts.first().map(ToString::to_string),
            Some(constants_str::VALUE_F70B3C4A.to_owned())
        );
        assert_eq!(
            parts.get(1).map(ToString::to_string),
            Some(constants_str::VALUE_86A27403.to_owned())
        );
    }
    #[test]
    fn test_proc_macro2_macro_tokens_to_tokens_preserves_stream() {
        let tokens = crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(quote::quote! {
            Result<Vec<A>, B>
        });
        assert_eq!(
            quote::quote! {#tokens}.to_string(),
            constants_str::VALUE_7BF22A2B
        );
    }
    #[test]
    fn test_unique_option_set_preserves_first_span_aware_error() {
        let mut values = crate::unique_option_b_tree_set::UniqueOptionBTreeSet::default();
        values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), constants_str::FIRST_ALT)
            })
            .expect(constants_str::DIAGNOSTIC_12817D29);
        let error = values
            .try_insert_with(1u8, || {
                syn::Error::new(proc_macro2::Span::call_site(), constants_str::DUPLICATE)
            })
            .expect_err(constants_str::CE4826F4);
        assert_eq!(error.to_string(), constants_str::DUPLICATE);
        assert!(values.contains(1u8).get());
    }
    #[test]
    fn test_getters_accept_raw_field_identifiers() {
        let validate =
            |proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens| {
                let generated =
                    crate::generate_private_field_getters::generate_private_field_getters(
                        proc_macro2_macro_tokens,
                    );
                let parsed = syn::parse2::<syn::File>(generated.into_inner());
                assert!(matches!(
                    parsed,
                    Ok(file) if matches!(file.items.as_slice(), [syn::Item::Impl(_)])
                ));
            };
        validate(
            quote::quote! {
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                #[getters(get_mut)]
                struct GetterRawDefaultFixture {
                    #[getters(copy)]
                    r#type: crate::part_index::PartIndex,
                }
            }
            .into(),
        );
        validate(
            quote::quote! {
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                #[getters(bare, get_mut)]
                struct GetterRawBareFixture {
                    #[getters(copy)]
                    r#type: crate::part_index::PartIndex,
                }
            }
            .into(),
        );
    }
    #[test]
    fn test_getters_preserve_distinct_prefixed_field_names() {
        let validate =
            |proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens| {
                let generated =
                    crate::generate_private_field_getters::generate_private_field_getters(
                        proc_macro2_macro_tokens,
                    );
                let parsed = syn::parse2::<syn::File>(generated.into_inner()).map(|file| {
                    file.items.iter().any(|item| {
                        if let syn::Item::Impl(implementation) = item {
                            let mut identifiers = std::collections::HashSet::new();
                            implementation.items.iter().all(|implementation_item| {
                                if let syn::ImplItem::Fn(method) = implementation_item {
                                    identifiers.insert(method.sig.ident.to_string())
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        }
                    })
                });
                assert!(matches!(parsed, Ok(true)));
            };
        validate(
            quote::quote! {
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                struct GetterPrefixDefaultFixture {
                    #[getters(copy)]
                    value: crate::part_index::PartIndex,
                    #[getters(copy)]
                    get_value: crate::part_index::PartIndex,
                }
            }
            .into(),
        );
        validate(
            quote::quote! {
                #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
                #[getters(bare)]
                struct GetterPrefixBareFixture {
                    #[getters(copy)]
                    value: crate::part_index::PartIndex,
                    #[getters(copy)]
                    get_value: crate::part_index::PartIndex,
                }
            }
            .into(),
        );
    }
}
