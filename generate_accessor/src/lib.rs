#[cfg(test)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct TestDependencyMarker;

#[proc_macro_derive(Getters, attributes(getters))]
pub fn getters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.into_compile_error().into(),
    };
    let generate = || -> syn::Result<proc_macro2::TokenStream> {
        let parsed_input_ref = &parsed_input;
        let syn::Data::Struct(data) = &parsed_input_ref.data else {
            return Err(syn::Error::new_spanned(
                parsed_input_ref,
                constants_str::GETTERS_REQUIRES_STRUCT,
            ));
        };
        let identifier = &parsed_input_ref.ident;
        let (impl_generics, type_generics, where_clause) =
            parsed_input_ref.generics.split_for_impl();
        let container_get_mut =
            parsed_input_ref
                .attrs
                .iter()
                .try_fold(false, |found, attribute| {
                    if !attribute.path().is_ident(constants_str::GETTERS_ATTRIBUTE) {
                        return Ok(found);
                    }
                    let mut attribute_get_mut = false;
                    attribute.parse_nested_meta(|metadata| {
                        if metadata.path.is_ident(constants_str::GETTERS_GET_MUT) {
                            attribute_get_mut = true;
                            Ok(())
                        } else {
                            Err(metadata.error(constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE))
                        }
                    })?;
                    Ok::<bool, syn::Error>(found || attribute_get_mut)
                })?;
        let methods = data
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let get_mut = field.attrs.iter().try_fold(false, |found, attribute| {
                    if !attribute.path().is_ident(constants_str::GETTERS_ATTRIBUTE) {
                        return Ok(found);
                    }
                    let mut attribute_get_mut = false;
                    attribute.parse_nested_meta(|metadata| {
                        if metadata.path.is_ident(constants_str::GETTERS_GET_MUT) {
                            attribute_get_mut = true;
                            Ok(())
                        } else {
                            Err(metadata.error(constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE))
                        }
                    })?;
                    Ok::<bool, syn::Error>(found || attribute_get_mut)
                })?;
                let (field_member, field_name) = match &field.ident {
                    Some(field_identifier) => (
                        quote::quote!(#field_identifier),
                        quote::format_ident!(
                            "get_{}",
                            field_identifier.to_string().chars().enumerate().fold(
                                String::new(),
                                |mut snake_case, (character_index, character)| {
                                    if character.is_uppercase() {
                                        if character_index != constants_usize::ZERO {
                                            snake_case.push('_');
                                        }
                                        snake_case.extend(character.to_lowercase());
                                    } else {
                                        snake_case.push(character);
                                    }
                                    snake_case
                                }
                            )
                        ),
                    ),
                    None if data.fields.len() == constants_usize::ONE => {
                        let syn_index = syn::Index::from(index);
                        (quote::quote!(#syn_index), quote::format_ident!("get_inner"))
                    }
                    None => {
                        return Err(syn::Error::new_spanned(
                            field,
                            constants_str::GETTERS_REQUIRES_NAMED_OR_SINGLE_FIELD,
                        ));
                    }
                };
                let field_type = &field.ty;
                let immutable = if let syn::Type::Path(type_path) = field_type
                    && type_path.qself.is_none()
                    && type_path.path.segments.len() == constants_usize::ONE
                    && let Some(option_segment) = type_path.path.segments.first()
                    && option_segment.ident == constants_str::OPTION_TYPE
                    && let syn::PathArguments::AngleBracketed(arguments) = &option_segment.arguments
                    && let Some(syn::GenericArgument::Type(inner_type)) = arguments.args.first()
                {
                    quote::quote! {
                        pub(crate) const fn #field_name(&self) -> Option<&#inner_type> {
                            self.#field_member.as_ref()
                        }
                    }
                } else {
                    quote::quote! {
                        pub(crate) const fn #field_name(&self) -> &#field_type {
                            &self.#field_member
                        }
                    }
                };
                let mutable = (container_get_mut || get_mut).then(|| {
                    let mutable_name = quote::format_ident!("{}_mut", field_name);
                    quote::quote! {
                        pub(crate) const fn #mutable_name(&mut self) -> &mut #field_type {
                            &mut self.#field_member
                        }
                    }
                });
                Ok(quote::quote!(#immutable #mutable))
            })
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(quote::quote! {
        #[allow(
            dead_code,
            reason = "private fields are intentionally exposed through uniform generated getters"
        )]
        impl #impl_generics #identifier #type_generics #where_clause {
            #(#methods)*
        }
        })
    };
    let generated = generate();
    match generated {
        Ok(value) => value.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
