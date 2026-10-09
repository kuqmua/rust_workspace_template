#[test]
fn test_enum_naming_impls_preserve_const_generics_and_where_clauses() {
    let input = quote::quote! {
        enum NamingGenericFixture<const LIMIT: usize = 4usize>
        where [(); LIMIT]: Sized {
            HelloWorld,
        }
    };
    let generated = [
        crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone()),
        crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
        crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone()),
        crate::enum_with_unit_fields_to_snake_case_str(input.clone()),
        crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
        crate::enum_with_unit_fields_to_upper_snake_case_str(input),
    ];
    assert!(generated.into_iter().all(|tokens| {
        syn::parse2::<syn::File>(tokens).is_ok_and(|file| {
            file.items.len() == constants_usize::ONE && file.items.into_iter().all(|item| {
                matches!(item, syn::Item::Impl(item_impl)
                    if item_impl.generics.params.len() == constants_usize::ONE
                        && matches!(item_impl.generics.params.first(), Some(syn::GenericParam::Const(parameter)) if parameter.default.is_none())
                        && item_impl.generics.where_clause.as_ref().is_some_and(|where_clause| where_clause.predicates.len() == constants_usize::ONE)
                        && matches!(item_impl.self_ty.as_ref(), syn::Type::Path(path) if path.path.segments.last().is_some_and(|segment| matches!(&segment.arguments, syn::PathArguments::AngleBracketed(arguments) if arguments.args.len() == constants_usize::ONE))))
            })
        })
    }));
}

#[test]
fn test_enum_naming_impls_report_oversized_case_conversion() {
    let variant_identifier = syn::Ident::new(
        constants_str::HELLOWORLD_ALT.repeat(104_858usize).as_str(),
        proc_macro2::Span::call_site(),
    );
    let input = quote::quote! {
        enum OversizedCaseFixture { #variant_identifier }
    };
    assert!(
        [
            crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone()),
            crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
            crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone()),
            crate::enum_with_unit_fields_to_snake_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_snake_case_str(input),
        ]
        .into_iter()
        .all(|tokens| tokens.to_string().contains(constants_str::VALUE_2EDAC0BF))
    );
}

#[test]
fn test_naming_enum_generators_emit_exact_case_values_and_methods() {
    let input = quote::quote! { enum NamingCaseValueFixture { HelloWorld } };
    assert!([
        (crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone()), constants_str::HELLO_WORLD_ALT.to_owned(), stringify!(case), true),
        (crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone()), constants_str::HELLOWORLD.to_owned(), stringify!(case), true),
        (crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone()), constants_str::HELLO_WORLD_ALT.to_ascii_uppercase(), stringify!(case), true),
        (crate::enum_with_unit_fields_to_snake_case_str(input.clone()), constants_str::HELLO_WORLD_ALT.to_owned(), stringify!(as_snake_case_str), false),
        (crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()), constants_str::HELLOWORLD.to_owned(), stringify!(as_upper_camel_case_str), false),
        (crate::enum_with_unit_fields_to_upper_snake_case_str(input), constants_str::HELLO_WORLD_ALT.to_ascii_uppercase(), stringify!(as_upper_snake_case_str), false),
    ].into_iter().all(|(tokens, expected, method_name, allocated)| {
        syn::parse2::<syn::ItemImpl>(tokens).is_ok_and(|implementation| {
            implementation.items.len() == 1usize && implementation.items.first().is_some_and(|item| {
                let syn::ImplItem::Fn(method) = item else { return false; };
                let Some(syn::Stmt::Expr(syn::Expr::Match(expression), None)) = method.block.stmts.first() else { return false; };
                method.sig.ident == method_name && method.sig.constness.is_some() != allocated
                    && expression.arms.len() == 1usize && expression.arms.first().is_some_and(|arm| {
                        let value = if allocated {
                            let syn::Expr::Call(call) = arm.body.as_ref() else { return false; };
                            if call.args.len() != 1usize { return false; }
                            call.args.first()
                        } else { Some(arm.body.as_ref()) };
                        value.is_some_and(|value_expression| matches!(value_expression, syn::Expr::Lit(literal) if matches!(&literal.lit, syn::Lit::Str(text) if text.value() == expected)))
                    })
            })
        })
    }));
}

#[test]
fn test_inherent_naming_generators_preserve_conditional_variant_match_arms() {
    let input = quote::quote! { enum NamingConditionalCaseFixture { #[cfg(all())] HelloWorld, #[cfg(any())] GoodbyeWorld } };
    let expected = [
        quote::quote! { #[cfg(all())] },
        quote::quote! { #[cfg(any())] },
    ];
    assert!(
        [
            crate::enum_with_unit_fields_to_snake_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_snake_case_str(input),
        ]
        .into_iter()
        .all(|tokens| {
            syn::parse2::<syn::ItemImpl>(tokens).is_ok_and(|implementation| {
                implementation.items.first().is_some_and(|item| {
                    let syn::ImplItem::Fn(method) = item else {
                        return false;
                    };
                    let Some(syn::Stmt::Expr(syn::Expr::Match(expression), None)) =
                        method.block.stmts.first()
                    else {
                        return false;
                    };
                    expression.arms.len() == expected.len()
                        && expression
                            .arms
                            .iter()
                            .zip(&expected)
                            .all(|(arm, expected_attribute)| {
                                arm.attrs.len() == 1usize
                                    && arm.attrs.first().is_some_and(|attribute| {
                                        quote::quote! { #attribute }.to_string()
                                            == expected_attribute.to_string()
                                    })
                            })
                })
            })
        })
    );
}

#[test]
fn test_naming_trait_generators_reject_non_enum_and_field_bearing_variants() {
    assert!([
        (quote::quote! { struct NamingRejectedStructFixture; }, false),
        (quote::quote! { enum NamingRejectedTupleFixture { First(crate::proc_macro2_naming_input_tokens::ProcMacro2NamingInputTokens) } }, true),
        (quote::quote! { enum NamingRejectedNamedFixture { First { value: crate::proc_macro2_naming_input_tokens::ProcMacro2NamingInputTokens } } }, true),
    ].into_iter().all(|(input, has_fields)| {
        [
            (std::panic::catch_unwind(|| crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone())), if has_fields { constants_str::PANIC_B3EF2657 } else { constants_str::PANIC_ED6EFE2E }),
            (std::panic::catch_unwind(|| crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone())), if has_fields { constants_str::PANIC_4955C50D } else { constants_str::PANIC_D26BF85E }),
            (std::panic::catch_unwind(|| crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone())), if has_fields { constants_str::PANIC_B6FEDCFF } else { constants_str::PANIC_B2263E7E }),
        ].into_iter().all(|(result, expected)| result.is_err_and(|payload| {
            payload.downcast_ref::<&str>().is_some_and(|text| *text == expected)
                || payload.downcast_ref::<String>().is_some_and(|text| text == expected)
        }))
    }));
}

#[test]
fn test_inherent_naming_generators_reject_non_enum_input() {
    let input = quote::quote! { struct NamingRejectedInherentFixture; };
    assert!(
        [
            std::panic::catch_unwind(|| crate::enum_with_unit_fields_to_snake_case_str(
                input.clone()
            )),
            std::panic::catch_unwind(|| crate::enum_with_unit_fields_to_upper_camel_case_str(
                input.clone()
            )),
            std::panic::catch_unwind(|| crate::enum_with_unit_fields_to_upper_snake_case_str(
                input.clone()
            )),
        ]
        .into_iter()
        .all(|result| result.is_err_and(|payload| {
            payload
                .downcast_ref::<&str>()
                .is_some_and(|text| *text == constants_str::PANIC_3FDBA6EF)
                || payload
                    .downcast_ref::<String>()
                    .is_some_and(|text| text == constants_str::PANIC_3FDBA6EF)
        }))
    );
}

#[test]
fn test_trait_naming_generators_preserve_distinct_malformed_input_diagnostics() {
    assert!(
        [
            (
                std::panic::catch_unwind(|| {
                    crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(
                        quote::quote! { enum },
                    )
                }),
                constants_str::DIAGNOSTIC_DEA5CBCF,
            ),
            (
                std::panic::catch_unwind(|| {
                    crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(
                        quote::quote! { enum },
                    )
                }),
                constants_str::DIAGNOSTIC_A8F22481,
            ),
            (
                std::panic::catch_unwind(|| {
                    crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(
                        quote::quote! { enum },
                    )
                }),
                constants_str::DIAGNOSTIC_EDABBC24,
            ),
        ]
        .into_iter()
        .all(|(result, diagnostic)| result.is_err_and(|payload| {
            payload
                .downcast_ref::<String>()
                .is_some_and(|message| message.starts_with(diagnostic))
        }))
    );
}
