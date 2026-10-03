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
        .all(|input| proc_macro2::TokenStream::from(crate::as_ref_inner(
            crate::proc_macro_input_token_stream::ProcMacroInputTokenStream::from(input)
        ))
        .to_string()
        .contains(constants_str::MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR))
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
