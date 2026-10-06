#[test]
fn test_bounded_string_missing_max_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(min = 1)]
        struct Value(String);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "29f8ddc2");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            constants_str::MACRO_DIAGNOSTICS_BOUNDED_STRING_MAX_ERROR
        );
    }
}
#[test]
fn test_bounded_string_utoipa_byte_length_returns_compile_error() {
    let input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, utoipa)]
        struct Value(bounded_types::bounded_string::BoundedString<0usize, 4usize, false>);
    };
    let result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&input),
    );
    assert!(result.is_err(), "da6f2151");
    if let Err(error) = result {
        assert_eq!(
            error.to_string(),
            constants_str::BOUNDEDSTRING_UTOIPA_REQUIRES_CHARS_SO_OPENAPI_LENGTH_SEMANTICS_MATCH_RUNTIME
        );
    }
}
#[test]
fn test_duplicate_options_preserve_attribute_diagnostic() {
    let bounded_input = syn::parse_quote! {
        #[derive(BoundedString)]
        #[bounded_string(max = 4, trim, trim)]
        struct BoundedValue(String);
    };
    let bounded_result = crate::generate_bounded_string_token_stream(
        crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&bounded_input),
    );
    if let Err(error) = bounded_result {
        assert_eq!(
            error.to_string(),
            constants_str::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR
        );
    } else {
        std::panic::panic_any(constants_str::PANIC_D03CED5C);
    }
}

#[test]
fn test_enum_from_str_rejects_non_enum_and_non_unit_variants() {
    assert!(
        [
            (
                quote::quote!(
                    struct Value;
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS
            ),
            (
                quote::quote!(
                    enum Value {
                        First(Inner),
                    }
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS
            ),
            (
                quote::quote!(
                    enum Value {
                        First { value: Inner },
                    }
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS
            ),
        ]
        .into_iter()
        .all(
            |(input, expected)| proc_macro2::TokenStream::from(crate::enum_from_str(
                crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)
            ))
            .to_string()
            .contains(expected)
        )
    );
}

#[test]
fn test_enum_from_str_preserves_case_insensitive_snake_names_and_allowed_order() {
    let first = syn::LitStr::new(stringify!(first_value), proc_macro2::Span::call_site());
    let second = syn::LitStr::new(stringify!(second_value), proc_macro2::Span::call_site());
    let allowed = syn::LitStr::new(
        format!(
            "{}{}{}",
            first.value(),
            constants_str::TEXT_ALT_6,
            second.value()
        )
        .as_str(),
        proc_macro2::Span::call_site(),
    );
    let output = proc_macro2::TokenStream::from(crate::enum_from_str(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
            enum Value {
                FirstValue,
                SecondValue,
            }
        )),
    ));
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote! { v if v.eq_ignore_ascii_case(#first) => Ok(Self::FirstValue), }
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote! { v if v.eq_ignore_ascii_case(#second) => Ok(Self::SecondValue), }
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(allowed_values = #allowed)
                .to_string()
                .as_str()
        )
    );
    assert!(
        syn::parse2::<syn::ItemImpl>(output)
            .is_ok_and(|implementation| implementation.self_ty == syn::parse_quote!(Value))
    );
}

#[test]
fn test_display_const_requires_exactly_one_attribute() {
    assert!(
        [
            (
                quote::quote!(
                    struct Value;
                ),
                constants_str::DISPLAY_CONST_REQUIRES_ATTRIBUTE
            ),
            (
                quote::quote!(
                    #[display_const(First)]
                    #[display_const(Second)]
                    struct Value;
                ),
                constants_str::DISPLAY_CONST_REQUIRES_ONE_ATTRIBUTE
            ),
        ]
        .into_iter()
        .all(
            |(input, expected)| proc_macro2::TokenStream::from(crate::display_const(
                crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)
            ))
            .to_string()
            .contains(expected)
        )
    );
}

#[test]
fn test_display_const_preserves_generics_and_expression() {
    let output = proc_macro2::TokenStream::from(crate::display_const(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[display_const(constants_str::TEST_FIRST)] struct Value<T>(T) where T: Copy;
        }),
    ));
    assert_eq!(output.to_string(), quote::quote! {
        impl<T> std::fmt::Display for Value<T> where T: Copy {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(constants_str::TEST_FIRST) }
        }
    }.to_string());
}

#[test]
fn test_wire_enum_rejects_missing_configuration_and_non_unit_shapes() {
    assert!(
        proc_macro2::TokenStream::from(crate::wire_enum(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
                enum Value {
                    First,
                }
            ))
        ))
        .to_string()
        .contains(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE)
    );
    assert!(
        [
            quote::quote!(
                struct Value;
            ),
            quote::quote!(
                enum Value {
                    First(Inner),
                }
            ),
            quote::quote!(
                enum Value {
                    First { value: Inner },
                }
            )
        ]
        .into_iter()
        .all(|item| proc_macro2::TokenStream::from(crate::wire_enum(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[wire_enum(ref_type = Ref, error_message = Message)] #item
            })
        ))
        .to_string()
        .contains(constants_str::WIRE_ENUM_SUPPORTS_UNIT_VARIANTS))
    );
}

#[test]
fn test_wire_enum_requires_unique_string_values_on_every_variant() {
    let value = syn::LitStr::new(constants_str::TEST_FIRST, proc_macro2::Span::call_site());
    assert!([
        (quote::quote!(First), constants_str::WIRE_ENUM_VARIANT_REQUIRES_WIRE),
        (quote::quote!(#[wire(#value)] First, #[wire(#value)] Second), constants_str::WIRE_ENUM_DUPLICATE_VALUE),
    ].into_iter().all(|(variants, expected)| { let diagnostic = syn::LitStr::new(expected, proc_macro2::Span::call_site()); proc_macro2::TokenStream::from(crate::wire_enum(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
        #[wire_enum(ref_type = Ref, error_message = Message)] enum Value { #variants }
    }))).to_string().contains(quote::quote!(#diagnostic).to_string().as_str())}));
}

#[test]
fn test_wire_enum_generates_ordered_catalog_and_exact_string_conversion() {
    let first = syn::LitStr::new(constants_str::TEST_FIRST, proc_macro2::Span::call_site());
    let second = syn::LitStr::new(constants_str::TEST_LAST, proc_macro2::Span::call_site());
    let output = proc_macro2::TokenStream::from(crate::wire_enum(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[wire_enum(ref_type = Ref, error_message = Message)]
            enum Value { #[wire(#first)] Second, #[wire(#second)] First }
        }),
    ));
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(
                pub const ALL: [Self; 2usize] = [Self::Second, Self::First];
            )
            .to_string()
            .as_str()
        )
    );
    assert!(text.contains(quote::quote! { match value { #first => Ok(Self::Second), #second => Ok(Self::First), _value => Err(ValueTryFromStrError), } }.to_string().as_str()));
    assert!(
        text.contains(
            quote::quote! { Self::Second => Ref::from(#first), Self::First => Ref::from(#second) }
                .to_string()
                .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 4usize));
}

#[test]
fn test_bounded_string_wrapper_rejects_wrong_storage_and_generics() {
    assert!(
        [
            (
                quote::quote!(
                    struct Value(String);
                ),
                constants_str::BOUNDEDSTRING_SUPPORTS_ONLY_STRING_TUPLE_STRUCTS
            ),
            (
                quote::quote!(
                    struct Value<T>(
                        bounded_types::bounded_string::BoundedString<0usize, 4usize, false>,
                    );
                ),
                constants_str::BOUNDEDSTRING_DOES_NOT_SUPPORT_GENERICS
            ),
            (
                quote::quote!(
                    struct Value {
                        value: String,
                    }
                ),
                constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR
            ),
            (
                quote::quote!(
                    struct Value(String, String);
                ),
                constants_str::NEWTYPE_SUPPORTS_ONLY_ONE_FIELD_TUPLE_STRUCTS
            ),
        ]
        .into_iter()
        .all(
            |(item, expected)| proc_macro2::TokenStream::from(crate::bounded_string_wrapper(
                crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(
                    quote::quote! {
                        #[bounded_string(max = 4)] #item
                    }
                )
            ))
            .to_string()
            .contains(expected)
        )
    );
}

#[test]
fn test_bounded_string_wrapper_rejects_unknown_options_and_duplicate_validator() {
    assert!([
        (quote::quote!(max = 4, unknown), constants_str::UNKNOWN_BOUNDED_STRING_OPTION),
        (quote::quote!(max = 4, validator = First, validator = Second), constants_str::NEWTYPE_TRY_FROM_VALIDATOR_DUPLICATE),
    ].into_iter().all(|(options, expected)| proc_macro2::TokenStream::from(crate::bounded_string_wrapper(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
        #[bounded_string(#options)] struct Value(bounded_types::bounded_string::BoundedString<0usize, 4usize, false>);
    }))).to_string().contains(expected)));
}

#[test]
fn test_bounded_string_wrapper_rejects_each_duplicate_flag() {
    assert!([
        quote::quote!(chars), quote::quote!(error), quote::quote!(nul_free), quote::quote!(serde),
        quote::quote!(trim), quote::quote!(utoipa), quote::quote!(write_only),
    ].into_iter().all(|option| proc_macro2::TokenStream::from(crate::bounded_string_wrapper(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
        #[bounded_string(max = 4, #option, #option)] struct Value(bounded_types::bounded_string::BoundedString<0usize, 4usize, false>);
    }))).to_string().contains(constants_str::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)));
}

#[test]
fn test_as_ref_inner_requires_shared_reference_storage() {
    assert!(
        [
            quote::quote!(
                struct Value(Inner);
            ),
            quote::quote!(
                struct Value<'a>(&'a mut Inner);
            )
        ]
        .into_iter()
        .all(|input| {
            let parsed_input: syn::DeriveInput = syn::parse_quote!(#input);
            crate::tuple_struct_one_field_ty(
                crate::newtype_syn_derive_input_ref::NewtypeSynDeriveInputRef::from(&parsed_input),
            )
            .is_ok_and(|inner_type| {
                let expected = syn::Error::new_spanned(
                    inner_type.as_ref(),
                    constants_str::MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR,
                )
                .into_compile_error();
                let output = proc_macro2::TokenStream::from(crate::as_ref_inner(
                    crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input),
                ));
                output.to_string() == expected.to_string()
            })
        })
    );
}

#[test]
fn test_as_ref_owned_rejects_shared_and_mutable_reference_storage() {
    assert!(
        [
            quote::quote!(
                struct Value<'a>(&'a Inner);
            ),
            quote::quote!(
                struct Value<'a>(&'a mut Inner);
            )
        ]
        .into_iter()
        .all(|input| proc_macro2::TokenStream::from(crate::as_ref_owned(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)
        ))
        .to_string()
        .contains(
            constants_str::NEWTYPE_AS_REF_OWNED_DOES_NOT_SUPPORT_REFERENCE_INNER_TYPES_USE_AS
        ))
    );
}

#[test]
fn test_as_mut_requires_mutable_reference_storage() {
    assert!(
        [
            quote::quote!(
                struct Value(Inner);
            ),
            quote::quote!(
                struct Value<'a>(&'a Inner);
            )
        ]
        .into_iter()
        .all(|input| proc_macro2::TokenStream::from(crate::as_mut(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)
        ))
        .to_string()
        .contains(constants_str::NEWTYPE_AS_MUT_REQUIRES_MUTABLE_REFERENCE_INNER_TYPE))
    );
    let output = proc_macro2::TokenStream::from(crate::as_mut(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
            struct Value<'a>(&'a mut Inner);
        )),
    ));
    assert!(
        output.to_string().contains(
            quote::quote! { fn as_mut(&mut self) -> &mut Inner { self.0 } }
                .to_string()
                .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 1usize));
}

#[test]
fn test_from_inner_rejects_unvalidated_string_storage() {
    assert!([quote::quote!(struct Value(String);), quote::quote!(struct Value(std::string::String);)].into_iter().all(|input| proc_macro2::TokenStream::from(crate::from_inner(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input))).to_string().contains(constants_str::NEWTYPE_FROM_INNER_CANNOT_BE_USED_FOR_STRING_WRAPPERS_IMPLEMENT_TRYFROM_STRING)));
}

#[test]
fn test_from_getter_validates_required_attribute_and_options() {
    assert!([
        (quote::quote!(), constants_str::FROM_GETTER_REQUIRES_ATTRIBUTE),
        (quote::quote!(#[from_getter(source = Source, getter = inner)] #[from_getter(source = Source, getter = inner)]), constants_str::FROM_GETTER_REQUIRES_ONE_ATTRIBUTE),
        (quote::quote!(#[from_getter(getter = inner)]), constants_str::MACRO_DIAGNOSTICS_FROM_GETTER_SOURCE_ERROR),
        (quote::quote!(#[from_getter(source = Source)]), constants_str::MACRO_DIAGNOSTICS_FROM_GETTER_GETTER_ERROR),
        (quote::quote!(#[from_getter(unknown = Source)]), constants_str::FROM_GETTER_REQUIRES_ATTRIBUTE),
    ].into_iter().all(|(attributes, expected)| proc_macro2::TokenStream::from(crate::from_getter(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! { #attributes struct Value(Inner); }))).to_string().contains(expected)));
}

#[test]
fn test_from_getter_requires_exactly_one_tuple_field() {
    assert!(
        [
            quote::quote!(
                struct Value;
            ),
            quote::quote!(
                struct Value();
            ),
            quote::quote!(
                struct Value(Inner, Inner);
            ),
            quote::quote!(
                struct Value {
                    value: Inner,
                }
            ),
            quote::quote!(
                enum Value {
                    First,
                }
            ),
        ]
        .into_iter()
        .all(|item| proc_macro2::TokenStream::from(crate::from_getter(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[from_getter(source = Source, getter = inner)] #item
            })
        ))
        .to_string()
        .contains(constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR))
    );
}

#[test]
fn test_from_getter_preserves_generic_source_and_getter_expression() {
    assert_eq!(
        proc_macro2::TokenStream::from(crate::from_getter(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[from_getter(source = Source<T>, getter = inner)] struct Value<T>(T) where T: Copy;
            })
        ))
        .to_string(),
        quote::quote! {
            impl<T> From<Source<T> > for Value<T> where T: Copy {
                fn from(value: Source<T>) -> Self { Self(value.inner()) }
            }
        }
        .to_string()
    );
}

#[test]
fn test_clone_fields_rejects_enum_input() {
    assert!(
        proc_macro2::TokenStream::from(crate::clone_fields(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
                enum Value {
                    First,
                }
            ))
        ))
        .to_string()
        .contains(constants_str::CLONE_FIELDS_SUPPORTS_ONLY_STRUCTS)
    );
}

#[test]
fn test_clone_fields_initializes_each_struct_shape() {
    assert!([
        (quote::quote!(struct Value { first: Inner, second: Other }), quote::quote! { Self { first: Clone::clone(&self.first), second: Clone::clone(&self.second), } }),
        (quote::quote!(struct Value(Inner, Other);), quote::quote! { Self(Clone::clone(&self.0), Clone::clone(&self.1),) }),
        (quote::quote!(struct Value;), quote::quote!(Self)),
    ].into_iter().all(|(input, initialization)| {
        let output = proc_macro2::TokenStream::from(crate::clone_fields(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)));
        let expected_body: syn::Block = syn::parse_quote!({ #initialization });
        syn::parse2::<syn::ItemImpl>(output).is_ok_and(|implementation| {
            matches!(implementation.items.as_slice(), [syn::ImplItem::Fn(function)] if function.block == expected_body)
        })
    }));
}

#[test]
fn test_clone_fields_adds_field_bounds_and_preserves_existing_where_clause() {
    let output = proc_macro2::TokenStream::from(crate::clone_fields(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
            struct Value<T>(T)
            where
                T: Copy;
        )),
    ));
    assert!(syn::parse2::<syn::ItemImpl>(output).is_ok_and(|implementation| {
        implementation.generics.where_clause.is_some_and(|where_clause| {
            let predicates = where_clause.predicates.into_iter().collect::<Vec<_>>();
            matches!(predicates.as_slice(), [copy, clone] if *copy == syn::parse_quote!(T: Copy) && *clone == syn::parse_quote!(T: Clone))
        })
    }));
}

#[test]
fn test_newtype_schema_forwards_explicit_and_inferred_schema_types() {
    assert!([false, true].into_iter().all(|explicit| {
        let attribute = if explicit {
            quote::quote!(#[utoipa_schema(SchemaType)])
        } else {
            quote::quote!()
        };
        let schema_type = if explicit {
            quote::quote!(SchemaType)
        } else {
            quote::quote!(Inner)
        };
        let output = proc_macro2::TokenStream::from(crate::utoipa_schema(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #attribute struct Value(Inner);
            }),
        ));
        let text = output.to_string();
        text.contains(
            quote::quote!(#schema_type: utoipa::ToSchema)
                .to_string()
                .as_str(),
        ) && text.contains(
            quote::quote!(<#schema_type as utoipa::PartialSchema>::schema())
                .to_string()
                .as_str(),
        ) && syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 2usize)
    }));
}

#[test]
fn test_newtype_schema_rejects_duplicate_schema_attributes() {
    assert!(
        proc_macro2::TokenStream::from(crate::utoipa_schema(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[utoipa_schema(First)] #[utoipa_schema(Second)] struct Value(Inner);
            })
        ))
        .to_string()
        .contains(constants_str::DUPLICATE_NEWTYPE_OPTION)
    );
}

#[test]
fn test_newtype_schema_rejects_invalid_schema_type_and_inferred_shape() {
    assert!(
        proc_macro2::TokenStream::from(crate::utoipa_schema(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[utoipa_schema(1)] struct Value(Inner);
            })
        ))
        .to_string()
        .contains(stringify!(compile_error))
    );
    assert!(
        proc_macro2::TokenStream::from(crate::utoipa_schema(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
                struct Value;
            ))
        ))
        .to_string()
        .contains(constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR)
    );
}

#[test]
fn test_newtype_iterator_preserves_associated_types_and_ownership_forwarding() {
    let output = proc_macro2::TokenStream::from(crate::into_iterator(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote!(
            struct Value<T>(Vec<T>);
        )),
    ));
    assert!(syn::parse2::<syn::ItemImpl>(output).is_ok_and(|implementation| {
        matches!(implementation.items.as_slice(), [syn::ImplItem::Type(iterator), syn::ImplItem::Type(item), syn::ImplItem::Fn(function)]
            if iterator.ident == stringify!(IntoIter)
                && iterator.ty == syn::parse_quote!(<Vec<T> as IntoIterator>::IntoIter)
                && item.ident == stringify!(Item)
                && item.ty == syn::parse_quote!(<Vec<T> as IntoIterator>::Item)
                && function.block == syn::parse_quote!({ self.0.into_iter() }))
    }));
}

#[test]
fn test_get_inner_selects_owned_borrowed_and_reference_return_shapes() {
    assert!(
        [
            (
                quote::quote!(
                    pub struct Value(Inner);
                ),
                quote::quote! { pub const fn get(self) -> Inner { self.0 } }
            ),
            (
                quote::quote!(
                    #[borrow]
                    pub struct Value(Inner);
                ),
                quote::quote! { pub const fn get(&self) -> &Inner { &self.0 } }
            ),
            (
                quote::quote!(
                    #[borrow]
                    pub struct Value<'a>(&'a Inner);
                ),
                quote::quote! { pub const fn get(&self) -> &'a Inner { self.0 } }
            ),
        ]
        .into_iter()
        .all(|(input, expected)| {
            let output = proc_macro2::TokenStream::from(crate::get_inner(
                crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input),
            ));
            output.to_string().contains(expected.to_string().as_str())
                && syn::parse2::<syn::ItemImpl>(output)
                    .is_ok_and(|implementation| implementation.items.len() == 1usize)
        })
    );
}

#[test]
fn test_get_inner_accessor_attribute_overrides_struct_visibility() {
    let output = proc_macro2::TokenStream::from(crate::get_inner(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[accessor(pub(crate))] pub struct Value(Inner);
        }),
    ));
    assert!(syn::parse2::<syn::ItemImpl>(output).is_ok_and(|implementation| {
        matches!(implementation.items.as_slice(), [syn::ImplItem::Fn(function)] if function.vis == syn::parse_quote!(pub(crate)))
    }));
}

#[test]
fn test_get_inner_rejects_duplicate_and_invalid_accessor_visibility() {
    assert!(
        proc_macro2::TokenStream::from(crate::get_inner(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[accessor(pub)] #[accessor(pub(crate))] struct Value(Inner);
            })
        ))
        .to_string()
        .contains(constants_str::DUPLICATE_NEWTYPE_OPTION)
    );
    assert!(
        proc_macro2::TokenStream::from(crate::get_inner(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #[accessor(1)] struct Value(Inner);
            })
        ))
        .to_string()
        .contains(stringify!(compile_error))
    );
}

#[test]
fn test_snake_identifier_preserves_byte_boundaries_and_error_diagnostics() {
    let maximum_length = crate::snake_ident_max_len::SNAKE_IDENT_MAX_LEN;
    let character_count = 524_288usize;
    assert_eq!(
        character_count.saturating_mul('\u{00e9}'.len_utf8()),
        maximum_length
    );
    let accepted_result =
        crate::snake_identifier::SnakeIdentifier::try_from(constants_str::X.repeat(maximum_length));
    assert!(accepted_result.is_ok());
    let Ok(accepted) = accepted_result else {
        return;
    };
    assert_eq!(accepted.as_ref().len(), maximum_length);
    let rejected_result = crate::snake_identifier::SnakeIdentifier::try_from(
        constants_str::X.repeat(maximum_length.saturating_add(1usize)),
    );
    assert!(rejected_result.is_err());
    let Err(rejected) = rejected_result else {
        return;
    };
    let expected = crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError::from(
        crate::snake_identifierifier_len::SnakeIdentifierifierLen::from(
            maximum_length.saturating_add(1usize),
        ),
    );
    assert_eq!(rejected.to_string(), expected.to_string());
    assert_eq!(format!("{rejected:?}"), format!("{expected:?}"));
    let unicode_result = crate::snake_identifier::SnakeIdentifier::try_from(
        '\u{00e9}'.to_string().repeat(character_count),
    );
    assert!(unicode_result.is_ok());
    let Ok(unicode) = unicode_result else {
        return;
    };
    assert_eq!(unicode.as_ref().len(), maximum_length);
    assert_eq!(unicode.as_ref().chars().count(), character_count);
    let unicode_overflow_result = crate::snake_identifier::SnakeIdentifier::try_from(
        '\u{00e9}'
            .to_string()
            .repeat(character_count.saturating_add(1usize)),
    );
    assert!(unicode_overflow_result.is_err());
    let Err(unicode_overflow) = unicode_overflow_result else {
        return;
    };
    let expected_unicode = crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError::from(
        crate::snake_identifierifier_len::SnakeIdentifierifierLen::from(
            maximum_length.saturating_add(2usize),
        ),
    );
    assert_eq!(unicode_overflow.to_string(), expected_unicode.to_string());
}

#[test]
fn test_snake_identifier_preserves_empty_text_display_and_tokens() {
    [
        String::new(),
        constants_str::X.to_owned(),
        ['x', '"', '\\', '\n'].into_iter().collect::<String>(),
    ]
    .into_iter()
    .fold((), |(), text| {
        let expected = proc_macro2::Literal::string(&text).to_string();
        let identifier_result = crate::snake_identifier::SnakeIdentifier::try_from(text);
        assert!(identifier_result.is_ok());
        let Ok(identifier) = identifier_result else {
            return;
        };
        assert_eq!(identifier.to_string(), identifier.as_ref());
        let mut tokens = proc_macro2::TokenStream::new();
        quote::ToTokens::to_tokens(&identifier, &mut tokens);
        assert_eq!(tokens.to_string(), expected);
    });
}

#[test]
fn test_wire_enum_attributes_preserve_required_fields_and_unknown_name_diagnostics() {
    [
        quote::quote!(),
        quote::quote!(ref_type = Ref),
        quote::quote!(error_message = Message),
        quote::quote!(unknown = Ref),
    ]
    .into_iter()
    .fold((), |(), tokens| {
        let parsed = syn::parse2::<crate::wire_enum_attrs::WireEnumAttrs>(tokens);
        assert!(parsed.is_err());
        let Err(error) = parsed else {
            return;
        };
        let diagnostic = error.to_string();
        let message = if diagnostic.starts_with(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE) {
            diagnostic.as_str()
        } else {
            diagnostic
                .split_once(',')
                .map_or(diagnostic.as_str(), |(_, tail)| tail.trim())
        };
        assert_eq!(message, constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE);
    });
}

#[test]
fn test_wire_enum_attributes_preserve_parser_syntax_diagnostics() {
    [
        (
            quote::quote!(1 = Ref),
            syn::parse2::<syn::Ident>(quote::quote!(1)).err(),
        ),
        (
            quote::quote!(ref_type Ref),
            syn::parse2::<syn::Token![=]>(quote::quote!(Ref)).err(),
        ),
        (
            quote::quote!(ref_type = 1),
            syn::parse2::<syn::Type>(quote::quote!(1)).err(),
        ),
        (
            quote::quote!(error_message = ;),
            syn::parse2::<syn::Expr>(quote::quote!(;)).err(),
        ),
        (
            quote::quote!(ref_type =),
            syn::parse2::<syn::Type>(quote::quote!()).err(),
        ),
        (
            quote::quote!(error_message =),
            syn::parse2::<syn::Expr>(quote::quote!()).err(),
        ),
        (
            quote::quote!(ref_type = Ref error_message = Message),
            syn::parse2::<syn::Token![,]>(quote::quote!(error_message)).err(),
        ),
    ]
    .into_iter()
    .fold((), |(), (tokens, expected)| {
        let parsed = syn::parse2::<crate::wire_enum_attrs::WireEnumAttrs>(tokens);
        assert!(parsed.is_err());
        assert!(expected.is_some());
        assert_eq!(
            parsed.err().map(|error| error.to_string()),
            expected.map(|error| error.to_string())
        );
    });
}

#[test]
fn test_wire_enum_attributes_preserve_order_independence_and_last_duplicate_value() {
    [
        quote::quote!(ref_type = Option<Ref>, error_message = Message),
        quote::quote!(error_message = Message, ref_type = Option<Ref>,),
        quote::quote!(ref_type = Previous, error_message = PreviousMessage, ref_type = Option<Ref>, error_message = Message),
    ].into_iter().fold((), |(), tokens| {
        let parsed = syn::parse2::<crate::wire_enum_attrs::WireEnumAttrs>(tokens);
        assert!(parsed.is_ok());
        let Ok(attributes) = parsed else { return; };
        assert_eq!(quote::ToTokens::to_token_stream(attributes.get_error_message()).to_string(), quote::quote!(Message).to_string());
        assert_eq!(quote::ToTokens::to_token_stream(attributes.get_ref_type().as_ref()).to_string(), quote::quote!(Option<Ref>).to_string());
    });
}

#[test]
fn test_newtype_entrypoints_preserve_derive_input_parse_diagnostics() {
    let first_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::as_mut;
    let entrypoints = [
        first_entrypoint,
        crate::as_ref,
        crate::as_ref_inner,
        crate::as_ref_owned,
        crate::as_ref_str,
        crate::as_ref_target,
        crate::as_slice,
        crate::borrow_inner,
        crate::borrow_owned,
        crate::borrow_path,
        crate::borrow_str,
        crate::clone_inner,
        crate::clone_fields,
        crate::debug_redacted,
        crate::debug_transparent,
        crate::deref_inner,
        crate::deref_mut_inner,
        crate::deref_mut_target,
        crate::deref_target,
        crate::display,
        crate::default_inner,
        crate::debug_display,
        crate::display_const,
        crate::from_inner,
        crate::from_getter,
        crate::accessor,
        crate::get_inner,
        crate::into_inner,
        crate::into_inner_from,
        crate::into_iterator,
        crate::into_vec,
        crate::not_inner,
        crate::partial_eq_inner,
        crate::to_tokens,
        crate::to_err_string,
        crate::to_err_string_as_ref_str,
        crate::to_err_string_debug,
        crate::enum_from_str,
        crate::utoipa_schema,
        crate::wire_enum,
        crate::bounded_string_wrapper,
    ];
    let inputs = [quote::quote!(), quote::quote!(1), quote::quote!(struct)];
    entrypoints.into_iter().fold((), |(), entrypoint| {
        inputs.iter().fold((), |(), input| {
            let expected = syn::parse2::<syn::DeriveInput>(input.clone()).err();
            assert!(expected.is_some());
            let output = proc_macro2::TokenStream::from(entrypoint(
                crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(
                    input.clone(),
                ),
            ));
            assert_eq!(
                Some(output.to_string()),
                expected.map(|error| error.into_compile_error().to_string())
            );
        });
    });
}

#[test]
fn test_newtype_trait_forwarding_preserves_generic_bounds_and_associated_outputs() {
    let clone_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::clone_inner;
    [
        (clone_entrypoint, quote::quote! {
            impl<T> Clone for Value<T> where T: Existing, Inner<T>: Clone {
                fn clone(&self) -> Self { Self(Clone::clone(&self.0)) }
            }
        }),
        (crate::default_inner, quote::quote! {
            impl<T> Default for Value<T> where T: Existing, Inner<T>: Default {
                fn default() -> Self { Self(Default::default()) }
            }
        }),
        (crate::not_inner, quote::quote! {
            impl<T> std::ops::Not for Value<T> where T: Existing, Inner<T>: std::ops::Not {
                type Output = <Inner<T> as std::ops::Not>::Output;
                fn not(self) -> Self::Output { std::ops::Not::not(self.0) }
            }
        }),
        (crate::debug_transparent, quote::quote! {
            impl<T> std::fmt::Debug for Value<T> where T: Existing, Inner<T>: std::fmt::Debug {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Debug::fmt(&self.0, f) }
            }
        }),
        (crate::partial_eq_inner, quote::quote! {
            impl<T> PartialEq<Inner<T>> for Value<T> where T: Existing, Inner<T>: PartialEq {
                fn eq(&self, other: &Inner<T>) -> bool { self.0.eq(other) }
            }
        }),
    ].into_iter().fold((), |(), (entrypoint, expected)| {
        let output = proc_macro2::TokenStream::from(entrypoint(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<T>(Inner<T>) where T: Existing;
        })));
        let parsed_result = syn::parse2::<syn::ItemImpl>(output);
        assert!(parsed_result.is_ok());
        let expected_result = syn::parse2::<syn::ItemImpl>(expected);
        assert!(expected_result.is_ok());
        assert_eq!(parsed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_newtype_error_text_modes_preserve_conversion_and_bounded_fallback() {
    let display_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::to_err_string;
    let debug_format = ['{', ':', '?', '}'].into_iter().collect::<String>();
    let debug_literal = syn::LitStr::new(debug_format.as_str(), proc_macro2::Span::call_site());
    [
        (display_entrypoint, quote::quote!(self.0.to_string())),
        (crate::to_err_string_as_ref_str, quote::quote!(AsRef::<str>::as_ref(&self.0).to_owned())),
        (crate::to_err_string_debug, quote::quote!(format!(#debug_literal, self.0))),
    ].into_iter().fold((), |(), (entrypoint, text)| {
        let output = proc_macro2::TokenStream::from(entrypoint(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<T>(Inner<T>) where T: Existing;
        })));
        let observed_result = syn::parse2::<syn::ItemImpl>(output);
        assert!(observed_result.is_ok());
        let expected: syn::ItemImpl = syn::parse_quote! {
            impl<T> to_err_string::to_err_string::ToErrString for Value<T> where T: Existing {
                fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
                    to_err_string::error_text::ErrorText::try_from(#text).unwrap_or_else(to_err_string::error_text::ErrorText::from)
                }
            }
        };
        assert_eq!(observed_result.ok(), Some(expected));
    });
}

#[test]
fn test_newtype_redacted_debug_preserves_generic_shape_without_formatting_inner_value() {
    let output = proc_macro2::TokenStream::from(crate::debug_redacted(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<T>(Inner<T>) where T: Existing;
        }),
    ));
    let parsed_result = syn::parse2::<syn::ItemImpl>(output);
    assert!(parsed_result.is_ok());
    let Ok(implementation) = parsed_result else {
        return;
    };
    let redacted = syn::LitStr::new(
        constants_str::REDACTED_ALT_3,
        proc_macro2::Span::call_site(),
    );
    let expected_body: syn::Block = syn::parse_quote! {
        { f.debug_tuple(stringify!(Value)).field(&#redacted).finish() }
    };
    let expected_signature: syn::Signature = syn::parse_quote! {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    };
    assert!(
        implementation
            .trait_
            .as_ref()
            .is_some_and(|(path, _)| *path == syn::parse_quote!(std::fmt::Debug))
    );
    assert_eq!(
        implementation.self_ty.as_ref(),
        &syn::parse_quote!(Value<T>)
    );
    assert_eq!(
        implementation.generics.where_clause,
        Some(syn::parse_quote!(where T: Existing))
    );
    assert!(
        matches!(implementation.items.as_slice(), [syn::ImplItem::Fn(function)] if function.sig == expected_signature && function.block == expected_body)
    );
}

#[test]
fn test_newtype_borrowing_forwarding_preserves_target_types_and_bound_locations() {
    let target_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::as_ref_target;
    [
        (target_entrypoint, quote::quote! {
            impl<T> AsRef<<Inner<T> as std::ops::Deref>::Target> for Value<T> where T: Existing, Inner<T>: std::ops::Deref {
                fn as_ref(&self) -> &<Inner<T> as std::ops::Deref>::Target { std::ops::Deref::deref(&self.0) }
            }
        }),
        (crate::as_slice, quote::quote! {
            impl<T> Value<T> where T: Existing {
                #[must_use]
                pub fn as_slice(&self) -> &<Inner<T> as std::ops::Deref>::Target where Inner<T>: std::ops::Deref, { std::ops::Deref::deref(&self.0) }
            }
        }),
        (crate::borrow_path, quote::quote! {
            impl<T> std::borrow::Borrow<std::path::Path> for Value<T> where T: Existing, Inner<T>: std::borrow::Borrow<std::path::Path> {
                fn borrow(&self) -> &std::path::Path { std::borrow::Borrow::<std::path::Path>::borrow(&self.0) }
            }
        }),
        (crate::deref_target, quote::quote! {
            impl<T> std::ops::Deref for Value<T> where T: Existing {
                type Target = <Inner<T> as std::ops::Deref>::Target;
                fn deref(&self) -> &Self::Target { &self.0 }
            }
        }),
        (crate::deref_mut_target, quote::quote! {
            impl<T> std::ops::DerefMut for Value<T> where T: Existing {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
            }
        }),
    ].into_iter().fold((), |(), (entrypoint, expected_tokens)| {
        let output = proc_macro2::TokenStream::from(entrypoint(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<T>(Inner<T>) where T: Existing;
        })));
        let observed_result = syn::parse2::<syn::ItemImpl>(output);
        assert!(observed_result.is_ok());
        let Ok(observed) = observed_result else { return; };
        let expected_result = syn::parse2::<syn::ItemImpl>(expected_tokens);
        assert!(expected_result.is_ok());
        let Ok(expected) = expected_result else { return; };
        assert_eq!(observed.generics, expected.generics);
        assert_eq!(observed.trait_, expected.trait_);
        assert_eq!(observed.self_ty, expected.self_ty);
        assert_eq!(observed.items, expected.items);
    });
}

#[test]
fn test_newtype_bounded_storage_forwarding_preserves_string_views_and_ownership_transfer() {
    let owned_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::as_ref_owned;
    [
        (owned_entrypoint, quote::quote! {
            impl<const MAX: usize> AsRef<String> for Value<MAX> where Bounds<MAX>: Existing {
                fn as_ref(&self) -> &String { self.0.as_string() }
            }
        }),
        (crate::deref_inner, quote::quote! {
            impl<const MAX: usize> std::ops::Deref for Value<MAX> where Bounds<MAX>: Existing {
                type Target = String;
                fn deref(&self) -> &Self::Target { self.0.as_string() }
            }
        }),
        (crate::into_inner, quote::quote! {
            impl<const MAX: usize> Value<MAX> where Bounds<MAX>: Existing {
                #[must_use]
                pub fn into_inner(self) -> String { self.0.into_string() }
            }
        }),
        (crate::into_inner_from, quote::quote! {
            impl<const MAX: usize> From<Value<MAX>> for String where Bounds<MAX>: Existing {
                fn from(value: Value<MAX>) -> Self { value.0.into_string() }
            }
        }),
        (crate::accessor, quote::quote! {
            pub trait ValueProvider<const MAX: usize> where Bounds<MAX>: Existing {
                fn value(&self) -> &String;
            }
            impl<const MAX: usize> ValueProvider<MAX> for Value<MAX> where Bounds<MAX>: Existing {
                fn value(&self) -> &String { self.0.as_string() }
            }
            impl<const MAX: usize> ValueProvider<MAX> for &Value<MAX> where Bounds<MAX>: Existing {
                fn value(&self) -> &String { self.0.as_string() }
            }
        }),
    ].into_iter().fold((), |(), (entrypoint, expected)| {
        let output = proc_macro2::TokenStream::from(entrypoint(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<const MAX: usize>(bounded_types::bounded_string::BoundedString<1usize, MAX, false>) where Bounds<MAX>: Existing;
        })));
        let observed_result = syn::parse2::<syn::File>(output);
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::File>(expected);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_bounded_string_generation_preserves_validated_serde_and_character_schema() {
    let output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[bounded_string(min = 1usize, max = 16usize, chars, serde, utoipa, write_only)]
            struct Value(bounded_types::bounded_string::BoundedString<1usize, 16usize, true>);
        }),
    ));
    let observed_result = syn::parse2::<syn::File>(output);
    assert!(observed_result.is_ok());
    let Ok(observed) = observed_result else {
        return;
    };
    let expected: syn::File = syn::parse_quote! {
        impl serde::Serialize for Value {
            fn serialize<Serializer>(&self, serializer: Serializer) -> Result<Serializer::Ok, Serializer::Error>
            where Serializer: serde::Serializer, {
                serde::Serialize::serialize(&self.0, serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for Value {
            fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
            where Deserializer: serde::Deserializer<'de>, {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                Self::try_from(value).map_err(serde::de::Error::custom)
            }
        }
        impl utoipa::PartialSchema for Value {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String)
                    .min_length(Some(1usize))
                    .max_length(Some(16usize))
                    .write_only(Some(true))
                    .build()
                    .into()
            }
        }
        impl utoipa::ToSchema for Value {}
    };
    expected.items.into_iter().fold((), |(), expected_item| {
        assert!(observed.items.contains(&expected_item));
    });
    let conversion_trait: syn::Path = syn::parse_quote!(From<ValueTryFromStringError>);
    assert!(!observed.items.iter().any(|item| matches!(item, syn::Item::Impl(implementation) if implementation.trait_.as_ref().is_some_and(|(path, _)| *path == conversion_trait))));
}

#[test]
fn test_bounded_string_generation_preserves_validation_order_and_storage_error_sources() {
    [false, true].into_iter().fold((), |(), count_chars| {
        let chars = if count_chars { quote::quote!(chars,) } else { quote::quote!() };
        let length = if count_chars { quote::quote!(value.chars().count()) } else { quote::quote!(value.len()) };
        let storage = quote::quote!(bounded_types::bounded_string::BoundedString<2usize, 8usize, #count_chars>);
        let output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[bounded_string(min = 2usize, max = 8usize, trim, nul_free, validator = validate, #chars)]
            struct Value(#storage);
        })));
        let observed_result = syn::parse2::<syn::File>(output);
        assert!(observed_result.is_ok());
        let Ok(observed) = observed_result else { return; };
        let expected: syn::Item = syn::parse_quote! {
            impl TryFrom<String> for Value {
                type Error = ValueTryFromStringError;
                fn try_from(value: String) -> Result<Self, Self::Error> {
                    let value = value.trim().to_owned();
                    if 2usize > 8usize {
                        return Err(Self::Error::InvalidBounds { min: 2usize, max: 8usize });
                    }
                    if value.contains('\0') { return Err(Self::Error::ContainsNul); }
                    let len = #length;
                    if len < 2usize { return Err(Self::Error::TooShort { len, min: 2usize }); }
                    if len > 8usize { return Err(Self::Error::TooLong { len, max: 8usize, }); }
                    if !(validate)(&value) { return Err(Self::Error::InvalidValue); }
                    match <#storage>::try_from(value) {
                        Ok(value) => Ok(Self(value)),
                        Err(bounded_types::bounded_string_error::BoundedStringError::BelowMinimum { actual_length, minimum_length, }) => Err(Self::Error::TooShort { len: actual_length.get(), min: minimum_length.get(), }),
                        Err(bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { actual_length, maximum_length, }) => Err(Self::Error::TooLong { len: actual_length.get(), max: maximum_length.get(), }),
                    }
                }
            }
        };
        assert!(observed.items.contains(&expected));
    });
}

#[test]
fn test_bounded_string_generation_without_minimum_preserves_truncated_error_conversion() {
    let output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[bounded_string(max = 16usize)]
            struct Value(bounded_types::bounded_string::BoundedString<0usize, 16usize, false>);
        }),
    ));
    let observed_result = syn::parse2::<syn::File>(output);
    assert!(observed_result.is_ok());
    let Ok(observed) = observed_result else {
        return;
    };
    let expected: syn::Item = syn::parse_quote! {
        impl From<ValueTryFromStringError> for Value {
            fn from(value: ValueTryFromStringError) -> Self {
                Self(<bounded_types::bounded_string::BoundedString<0usize, 16usize, false>>::from_truncated(value.to_string()))
            }
        }
    };
    assert!(observed.items.contains(&expected));
    let conversion_trait: syn::Path = syn::parse_quote!(TryFrom<String>);
    let zero_minimum: syn::Stmt = syn::parse_quote! {
        if 0usize > 16usize { return Err(Self::Error::InvalidBounds { min: 0usize, max: 16usize }); }
    };
    assert!(observed.items.iter().any(|item| matches!(item, syn::Item::Impl(implementation) if implementation.trait_.as_ref().is_some_and(|(path, _)| *path == conversion_trait) && implementation.items.iter().any(|member| matches!(member, syn::ImplItem::Fn(function) if function.block.stmts.first() == Some(&zero_minimum))))));
}

#[test]
fn test_bounded_string_attribute_values_preserve_nested_parser_diagnostics() {
    [quote::quote!(max), quote::quote!(min), quote::quote!(description), quote::quote!(validator)].into_iter().fold((), |(), field| {
        [quote::quote!(), quote::quote!(=), quote::quote!(= ;), quote::quote!(= Value Extra)].into_iter().fold((), |(), tail| {
            let attribute: syn::Attribute = syn::parse_quote!(#[bounded_string(#field #tail)]);
            let expected = attribute.parse_nested_meta(|meta| {
                let _expression = meta.value()?.parse::<syn::Expr>()?;
                Ok(())
            }).err();
            assert!(expected.is_some());
            let output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                #attribute
                struct Value(bounded_types::bounded_string::BoundedString<0usize, 16usize, false>);
            })));
            assert_eq!(Some(output.to_string()), expected.map(|error| error.into_compile_error().to_string()));
        });
    });
}

#[test]
fn test_bounded_string_repeated_metadata_preserves_last_values_across_attributes() {
    let output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            #[bounded_string(min = 1usize, max = 4usize, description = constants_str::X)]
            #[bounded_string(min = 2usize, max = 8usize, description = constants_str::CURSOR_TEST_PAYLOAD, chars, utoipa, error)]
            struct Value(bounded_types::bounded_string::BoundedString<2usize, 8usize, true>);
        }),
    ));
    let observed_result = syn::parse2::<syn::File>(output);
    assert!(observed_result.is_ok());
    let Ok(observed) = observed_result else {
        return;
    };
    let expected_schema: syn::Item = syn::parse_quote! {
        impl utoipa::PartialSchema for Value {
            fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String)
                    .min_length(Some(2usize))
                    .max_length(Some(8usize))
                    .build().into()
            }
        }
    };
    assert!(observed.items.contains(&expected_schema));
    let enum_result = observed.items.iter().find_map(|item| {
        if let syn::Item::Enum(value) = item {
            Some(value)
        } else {
            None
        }
    });
    assert!(enum_result.is_some());
    let Some(error_enum) = enum_result else {
        return;
    };
    assert_eq!(error_enum.variants.len(), 5usize);
    assert!(error_enum.variants.iter().all(|variant| variant.attrs.iter().any(|attribute| {
        attribute.path().is_ident(stringify!(error)) && attribute.parse_args_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated).is_ok_and(|arguments| arguments.last().is_some_and(|expression| *expression == syn::parse_quote!(constants_str::CURSOR_TEST_PAYLOAD)))
    })));
}
#[test]
fn test_newtype_debug_display_preserves_generic_self_bound_and_debug_forwarding() {
    let output = proc_macro2::TokenStream::from(crate::debug_display(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value<T>(Inner<T>) where T: Existing;
        }),
    ));
    let observed_result = syn::parse2::<syn::ItemImpl>(output);
    assert!(observed_result.is_ok());
    let Ok(observed) = observed_result else {
        return;
    };
    let expected: syn::ItemImpl = syn::parse_quote! {
        impl<T> std::fmt::Display for Value<T> where T: Existing, Value<T>: std::fmt::Debug {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    };
    assert_eq!(observed, expected);
}

#[test]
fn test_newtype_shared_reference_forwarding_preserves_lifetimes_and_target_types() {
    let reference_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::as_ref_inner;
    [
        (
            reference_entrypoint,
            quote::quote! {
                impl<'a, T> AsRef<Inner<T>> for Value<'a, T> where T: Existing {
                    fn as_ref(&self) -> &Inner<T> { self.0 }
                }
            },
        ),
        (
            crate::borrow_inner,
            quote::quote! {
                impl<'a, T> std::borrow::Borrow<Inner<T>> for Value<'a, T> where T: Existing {
                    fn borrow(&self) -> &Inner<T> { self.0 }
                }
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (entrypoint, expected_tokens)| {
        let output = proc_macro2::TokenStream::from(entrypoint(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                struct Value<'a, T>(&'a Inner<T>) where T: Existing;
            }),
        ));
        let observed_result = syn::parse2::<syn::ItemImpl>(output);
        assert!(observed_result.is_ok());
        let Ok(observed) = observed_result else {
            return;
        };
        let expected_result = syn::parse2::<syn::ItemImpl>(expected_tokens);
        assert!(expected_result.is_ok());
        let Ok(expected) = expected_result else {
            return;
        };
        assert_eq!(observed.generics, expected.generics);
        assert_eq!(observed.trait_, expected.trait_);
        assert_eq!(observed.self_ty, expected.self_ty);
        assert_eq!(observed.items, expected.items);
    });
}

#[test]
fn test_newtype_owned_storage_forwarding_preserves_generic_shape_and_borrowing() {
    let owned_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::as_ref_owned;
    [
        (
            owned_entrypoint,
            quote::quote! {
                impl<T> AsRef<Inner<T>> for Value<T> where T: Existing {
                    fn as_ref(&self) -> &Inner<T> { &self.0 }
                }
            },
        ),
        (
            crate::as_ref_str,
            quote::quote! {
                impl<T> AsRef<str> for Value<T> where T: Existing {
                    fn as_ref(&self) -> &str { AsRef::<str>::as_ref(&self.0) }
                }
            },
        ),
        (
            crate::as_ref,
            quote::quote! {
                impl<T> Value<T> where T: Existing {
                    #[must_use]
                    pub const fn as_ref(&self) -> &Inner<T> { &self.0 }
                }
            },
        ),
        (
            crate::borrow_owned,
            quote::quote! {
                impl<T> std::borrow::Borrow<Inner<T>> for Value<T> where T: Existing {
                    fn borrow(&self) -> &Inner<T> { &self.0 }
                }
            },
        ),
        (
            crate::borrow_str,
            quote::quote! {
                impl<T> std::borrow::Borrow<str> for Value<T> where T: Existing {
                    fn borrow(&self) -> &str { std::borrow::Borrow::<str>::borrow(&self.0) }
                }
            },
        ),
        (
            crate::deref_inner,
            quote::quote! {
                impl<T> std::ops::Deref for Value<T> where T: Existing {
                    type Target = Inner<T>;
                    fn deref(&self) -> &Self::Target { &self.0 }
                }
            },
        ),
        (
            crate::deref_mut_inner,
            quote::quote! {
                impl<T> std::ops::DerefMut for Value<T> where T: Existing {
                    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
                }
            },
        ),
        (
            crate::from_inner,
            quote::quote! {
                impl<T> From<Inner<T>> for Value<T> where T: Existing {
                    fn from(value: Inner<T>) -> Self { Self(value) }
                }
            },
        ),
        (
            crate::into_vec,
            quote::quote! {
                impl<T> Value<T> where T: Existing {
                    #[must_use]
                    pub fn into_vec(self) -> Inner<T> { self.0 }
                }
            },
        ),
        (
            crate::to_tokens,
            quote::quote! {
                impl<T> quote::ToTokens for Value<T> where T: Existing {
                    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                        quote::ToTokens::to_tokens(&self.0, tokens);
                    }
                }
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (entrypoint, expected_tokens)| {
        let output = proc_macro2::TokenStream::from(entrypoint(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                struct Value<T>(Inner<T>) where T: Existing;
            }),
        ));
        let observed_result = syn::parse2::<syn::ItemImpl>(output);
        assert!(observed_result.is_ok());
        let Ok(observed) = observed_result else {
            return;
        };
        let expected_result = syn::parse2::<syn::ItemImpl>(expected_tokens);
        assert!(expected_result.is_ok());
        let Ok(expected) = expected_result else {
            return;
        };
        assert_eq!(observed.generics, expected.generics);
        assert_eq!(observed.trait_, expected.trait_);
        assert_eq!(observed.self_ty, expected.self_ty);
        assert_eq!(observed.items, expected.items);
    });
}

#[test]
fn test_newtype_ordinary_storage_provider_and_conversion_preserve_generic_contracts() {
    let provider_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::accessor;
    [
        (
            provider_entrypoint,
            quote::quote! {
                pub trait ValueProvider<T> where T: Existing {
                    fn value(&self) -> &Inner<T>;
                }
                impl<T> ValueProvider<T> for Value<T> where T: Existing {
                    fn value(&self) -> &Inner<T> { &self.0 }
                }
                impl<T> ValueProvider<T> for &Value<T> where T: Existing {
                    fn value(&self) -> &Inner<T> { &self.0 }
                }
            },
        ),
        (
            crate::into_inner_from,
            quote::quote! {
                impl<T> From<Value<T>> for Inner<T> where T: Existing {
                    fn from(value: Value<T>) -> Self { value.0 }
                }
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (entrypoint, expected_tokens)| {
        let output = proc_macro2::TokenStream::from(entrypoint(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
                struct Value<T>(Inner<T>) where T: Existing;
            }),
        ));
        let observed_result = syn::parse2::<syn::File>(output);
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::File>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_newtype_nested_attributes_preserve_parser_diagnostics() {
    let display_entrypoint: fn(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream,
    ) -> crate::proc_macro2_generated_token_stream::ProcMacro2GeneratedTokenStream = crate::display_const;
    [
        (
            display_entrypoint,
            quote::quote! {
                #[display_const()] struct Value(Inner);
            },
            syn::parse2::<syn::Expr>(proc_macro2::TokenStream::new()).err(),
        ),
        (
            crate::wire_enum,
            quote::quote! {
                #[wire_enum(ref_type =)] enum Value { First }
            },
            syn::parse2::<crate::wire_enum_attrs::WireEnumAttrs>(quote::quote!(ref_type =)).err(),
        ),
        (
            crate::wire_enum,
            quote::quote! {
                #[wire_enum(ref_type = Ref, error_message = Message)] enum Value { #[wire()] First }
            },
            syn::parse2::<syn::LitStr>(proc_macro2::TokenStream::new()).err(),
        ),
    ]
    .into_iter()
    .fold((), |(), (entrypoint, input, expected_error)| {
        assert!(expected_error.is_some());
        let Some(error) = expected_error else {
            return;
        };
        let output = proc_macro2::TokenStream::from(entrypoint(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input),
        ));
        assert_eq!(output.to_string(), error.into_compile_error().to_string());
    });
}

#[test]
fn test_from_getter_metadata_preserves_nested_syntax_diagnostics() {
    [quote::quote!(source), quote::quote!(getter)]
        .into_iter()
        .fold((), |(), field| {
            [
                quote::quote!(),
                quote::quote!(=),
                quote::quote!(= ;),
                quote::quote!(= Value Extra),
            ]
            .into_iter()
            .fold((), |(), tail| {
                let attribute: syn::Attribute = syn::parse_quote!(#[from_getter(#field #tail)]);
                let expected_error = attribute
                    .parse_nested_meta(|meta| {
                        if meta.path.is_ident(constants_str::SOURCE) {
                            let _source = meta.value()?.parse::<syn::Type>()?;
                        } else {
                            let _getter = meta.value()?.parse::<syn::Ident>()?;
                        }
                        Ok(())
                    })
                    .err();
                assert!(expected_error.is_some());
                let Some(error) = expected_error else {
                    return;
                };
                let output = proc_macro2::TokenStream::from(crate::from_getter(
                    crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(
                        quote::quote! {
                            #attribute struct Value(Inner);
                        },
                    ),
                ));
                assert_eq!(output.to_string(), error.into_compile_error().to_string());
            });
        });
}

#[test]
fn test_newtype_tuple_shape_rejections_preserve_enum_and_union_diagnostics() {
    [
        quote::quote!(
            enum Value {
                First,
            }
        ),
        quote::quote!(union Value { value: Inner }),
    ]
    .into_iter()
    .fold((), |(), item| {
        let input: syn::DeriveInput = syn::parse_quote!(#item);
        let expected =
            syn::Error::new_spanned(&input, constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR)
                .into_compile_error();
        let output = proc_macro2::TokenStream::from(crate::as_ref(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(item.clone()),
        ));
        assert_eq!(output.to_string(), expected.to_string());
        let bounded_input: syn::DeriveInput = syn::parse_quote!(#[bounded_string(max = 16)] #item);
        let bounded_expected = syn::Error::new_spanned(
            &bounded_input,
            constants_str::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
        )
        .into_compile_error();
        let bounded_output = proc_macro2::TokenStream::from(crate::bounded_string_wrapper(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(
                quote::quote!(#bounded_input),
            ),
        ));
        assert_eq!(bounded_output.to_string(), bounded_expected.to_string());
    });
}

#[test]
fn test_borrow_inner_rejects_owned_storage_with_exact_diagnostic() {
    let inner_type: syn::Type = syn::parse_quote!(Inner);
    let expected = syn::Error::new_spanned(
        inner_type,
        constants_str::MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR,
    )
    .into_compile_error();
    let output = proc_macro2::TokenStream::from(crate::borrow_inner(
        crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(quote::quote! {
            struct Value(Inner);
        }),
    ));
    assert_eq!(output.to_string(), expected.to_string());
}
