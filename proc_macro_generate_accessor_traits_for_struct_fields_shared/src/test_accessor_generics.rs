#[test]
fn test_provider_declarations_preserve_generics_and_bounds() {
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct ForwardCollisionFixture<'value, ForwardCollisionFixture0, const COUNT: usize>(
            &'value ForwardCollisionFixture0
        ) where ForwardCollisionFixture0: Copy;
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if matches!(file.items.as_slice(), [syn::Item::Trait(provider), syn::Item::Impl(forwarding)]
            if provider.generics.params.len() == 3usize
                && provider.generics.where_clause.is_some()
                && forwarding.generics.params.len() == 4usize
                && forwarding.generics.where_clause.is_some())));
}

#[test]
fn test_field_provider_implementations_preserve_owner_generics() {
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct ConstStateFixture<const COUNT: usize> {
            value: FixtureValue,
        }
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if file.items.len() == 2usize && file.items.iter().all(|item|
            matches!(item, syn::Item::Impl(implementation)
                if implementation.generics.params.len() == 1usize))));
}
#[test]
fn test_field_providers_accept_raw_identifiers() {
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct RawFieldFixture {
            r#type: FixtureValue,
        }
    });
    let expected = quote::quote! {
        impl app_state::TypeProvider for RawFieldFixture {
            fn r#type(&self) -> &FixtureValue {
                &self.r#type
            }
        }
        impl app_state::TypeProvider for &RawFieldFixture {
            fn r#type(&self) -> &FixtureValue {
                &self.r#type
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}
#[test]
fn test_provider_forwarding_does_not_shadow_field_type() {
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct ForwardFieldFixture(ForwardFieldFixture0);
    });
    assert!(matches!(syn::parse2::<syn::File>(generated), Ok(file)
        if matches!(file.items.as_slice(), [syn::Item::Trait(_), syn::Item::Impl(forwarding)]
            if matches!(forwarding.generics.params.first(), Some(syn::GenericParam::Type(parameter))
                if parameter.ident != quote::format_ident!("ForwardFieldFixture0")))));
}

#[test]
fn test_oversized_case_conversion_emits_compile_error() {
    let identifier = syn::Ident::new(
        constants_str::HELLOWORLD_ALT.repeat(104_858usize).as_str(),
        proc_macro2::Span::call_site(),
    );
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct #identifier(u8);
    });
    assert!(
        generated
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
    );
}

#[test]
fn test_provider_declaration_and_reference_forwarding_preserve_accessor_body() {
    let generated = crate::generate_accessor_trait(quote::quote! {
        struct AccessorBodyFixture(FixtureValue);
    });
    let expected = quote::quote! {
        pub trait AccessorBodyFixtureProvider {
            fn accessor_body_fixture(&self) -> &FixtureValue;
        }
        impl<AccessorBodyFixture0: ?Sized> AccessorBodyFixtureProvider for &AccessorBodyFixture0
        where AccessorBodyFixture0: AccessorBodyFixtureProvider
        {
            fn accessor_body_fixture(&self) -> &FixtureValue {
                <AccessorBodyFixture0 as AccessorBodyFixtureProvider>::accessor_body_fixture(*self)
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn test_multiple_field_providers_preserve_exact_types_accessors_and_owner_generics() {
    let generated = crate::generate_accessor_traits_for_struct_fields(quote::quote! {
        struct MultiFieldProviderFixture<'value, T, const SIZE: usize>
        where T: Copy {
            items: FixtureCollection<T, SIZE>,
            name: &'value FixtureName,
        }
    });
    let expected = quote::quote! {
        impl<'value, T, const SIZE: usize> app_state::ItemsProvider
            for MultiFieldProviderFixture<'value, T, SIZE> where T: Copy {
            fn items(&self) -> &FixtureCollection<T, SIZE> {
                &self.items
            }
        }
        impl<'value, T, const SIZE: usize> app_state::ItemsProvider
            for &MultiFieldProviderFixture<'value, T, SIZE> where T: Copy {
            fn items(&self) -> &FixtureCollection<T, SIZE> {
                &self.items
            }
        }
        impl<'value, T, const SIZE: usize> app_state::NameProvider
            for MultiFieldProviderFixture<'value, T, SIZE> where T: Copy {
            fn name(&self) -> & &'value FixtureName {
                &self.name
            }
        }
        impl<'value, T, const SIZE: usize> app_state::NameProvider
            for &MultiFieldProviderFixture<'value, T, SIZE> where T: Copy {
            fn name(&self) -> & &'value FixtureName {
                &self.name
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn test_accessor_provider_rejects_nonstruct_and_nontuple_shapes() {
    assert!(
        [
            (
                quote::quote! { enum RejectedAccessorEnum { Value } },
                constants_str::PANIC_CD6BBC4E
            ),
            (
                quote::quote! { union RejectedAccessorUnion { value: u8 } },
                constants_str::PANIC_CD6BBC4E
            ),
            (
                quote::quote! { struct RejectedAccessorNamed { value: u8 } },
                constants_str::PANIC_577CB86A
            ),
            (
                quote::quote! { struct RejectedAccessorUnit; },
                constants_str::PANIC_577CB86A
            ),
        ]
        .into_iter()
        .all(|(token_stream, diagnostic)| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::generate_accessor_trait(token_stream)
            }))
            .is_err_and(|error| {
                error
                    .downcast_ref::<&str>()
                    .is_some_and(|message| *message == diagnostic)
            })
        })
    );
}

#[test]
fn test_field_provider_rejects_nonstruct_shapes_and_unnamed_fields() {
    assert!(
        [
            quote::quote! { enum RejectedFieldProviderEnum { Value } },
            quote::quote! { union RejectedFieldProviderUnion { value: u8 } },
        ]
        .into_iter()
        .all(|token_stream| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::generate_accessor_traits_for_struct_fields(token_stream)
            }))
            .is_err_and(|error| {
                error
                    .downcast_ref::<&str>()
                    .is_some_and(|message| *message == constants_str::PANIC_15CD72A2)
            })
        })
    );
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::generate_accessor_traits_for_struct_fields(quote::quote! {
            struct RejectedUnnamedFieldProvider(u8);
        })
    }));
    assert!(result.is_err_and(|error| {
        error
            .downcast_ref::<String>()
            .is_some_and(|message| message.starts_with(constants_str::DIAGNOSTIC_E5C23C45))
    }));
    assert!(
        crate::generate_accessor_traits_for_struct_fields(quote::quote! {
            struct EmptyFieldProviderFixture;
        })
        .is_empty()
    );
}
