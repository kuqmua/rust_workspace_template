#[cfg(test)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "the marker makes the integration-test-only derive dependency explicit"
)]
struct GenerateAccessorTestDependencyMarker;

#[proc_macro_derive(Getters, attributes(getters))]
pub fn getters(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = match syn::parse::<syn::DeriveInput>(token_stream) {
        Ok(value) => value,
        Err(error) => return error.into_compile_error().into(),
    };
    let generate = || -> syn::Result<proc_macro2::TokenStream> {
        let syn::Data::Struct(data) = &parsed_input.data else {
            return Err(syn::Error::new_spanned(
                &parsed_input,
                constants_str::GETTERS_REQUIRES_STRUCT,
            ));
        };
        let identifier = &parsed_input.ident;
        let visibility = &parsed_input.vis;
        let (impl_generics, type_generics, where_clause) = parsed_input.generics.split_for_impl();
        let (container_bare, container_get_mut) =
            parsed_input
                .attrs
                .iter()
                .try_fold((false, false), |found, attribute| {
                    if !attribute.path().is_ident(constants_str::GETTERS_ATTRIBUTE) {
                        return Ok(found);
                    }
                    let mut attribute_bare = false;
                    let mut attribute_get_mut = false;
                    attribute.parse_nested_meta(|metadata| {
                        if metadata.path.is_ident(constants_str::GETTERS_BARE) {
                            attribute_bare = true;
                            Ok(())
                        } else if metadata.path.is_ident(constants_str::GETTERS_GET_MUT) {
                            attribute_get_mut = true;
                            Ok(())
                        } else {
                            Err(metadata.error(constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE))
                        }
                    })?;
                    Ok::<(bool, bool), syn::Error>((
                        found.0 || attribute_bare,
                        found.1 || attribute_get_mut,
                    ))
                })?;
        let methods = data
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let (copy, get_mut, skip) =
                    field
                        .attrs
                        .iter()
                        .try_fold((false, false, false), |found, attribute| {
                            if !attribute.path().is_ident(constants_str::GETTERS_ATTRIBUTE) {
                                return Ok(found);
                            }
                            let mut attribute_copy = false;
                            let mut attribute_get_mut = false;
                            let mut attribute_skip = false;
                            attribute.parse_nested_meta(|metadata| {
                                if metadata.path.is_ident(constants_str::GETTERS_COPY) {
                                    attribute_copy = true;
                                    Ok(())
                                } else if metadata.path.is_ident(constants_str::GETTERS_GET_MUT) {
                                    attribute_get_mut = true;
                                    Ok(())
                                } else if metadata.path.is_ident(constants_str::GETTERS_SKIP) {
                                    attribute_skip = true;
                                    Ok(())
                                } else {
                                    Err(metadata
                                        .error(constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE))
                                }
                            })?;
                            Ok::<(bool, bool, bool), syn::Error>((
                                found.0 || attribute_copy,
                                found.1 || attribute_get_mut,
                                found.2 || attribute_skip,
                            ))
                        })?;
                if skip {
                    return Ok(quote::quote!());
                }
                let (field_member, field_name) = match &field.ident {
                    Some(field_identifier) => (
                        quote::quote!(#field_identifier),
                        if container_bare {
                            field_identifier.clone()
                        } else {
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
                            )
                        },
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
                let is_single_tuple_field = field.ident.is_none();
                let reference_name = if is_single_tuple_field {
                    quote::format_ident!("get_ref")
                } else {
                    quote::format_ident!("get_ref_{}", field_name.to_string().trim_start_matches("get_"))
                };
                let value_name = if is_single_tuple_field {
                    quote::format_ident!("get_value")
                } else {
                    quote::format_ident!("get_value_{}", field_name.to_string().trim_start_matches("get_"))
                };
                let reference = if let syn::Type::Path(type_path) = field_type
                    && type_path.qself.is_none()
                    && type_path.path.segments.len() == constants_usize::ONE
                    && let Some(option_segment) = type_path.path.segments.first()
                    && option_segment.ident == constants_str::OPTION_TYPE
                    && let syn::PathArguments::AngleBracketed(arguments) = &option_segment.arguments
                    && let Some(syn::GenericArgument::Type(inner_type)) = arguments.args.first()
                {
                    quote::quote! {
                        #visibility const fn #reference_name(&self) -> Option<&#inner_type> {
                            self.#field_member.as_ref()
                        }
                    }
                } else {
                    quote::quote! {
                        #visibility const fn #reference_name(&self) -> &#field_type {
                            &self.#field_member
                        }
                    }
                };
                let value = copy.then(|| {
                    quote::quote! {
                        #visibility const fn #value_name(&self) -> #field_type {
                            self.#field_member
                        }
                    }
                });
                let compatibility = if copy && !is_single_tuple_field {
                    quote::quote! {
                        #visibility const fn #field_name(&self) -> #field_type {
                            self.#field_member
                        }
                    }
                } else if let syn::Type::Path(type_path) = field_type
                    && type_path.qself.is_none()
                    && type_path.path.segments.len() == constants_usize::ONE
                    && let Some(option_segment) = type_path.path.segments.first()
                    && option_segment.ident == constants_str::OPTION_TYPE
                    && let syn::PathArguments::AngleBracketed(arguments) = &option_segment.arguments
                    && let Some(syn::GenericArgument::Type(inner_type)) = arguments.args.first()
                {
                    quote::quote! {
                        #visibility const fn #field_name(&self) -> Option<&#inner_type> {
                            self.#field_member.as_ref()
                        }
                    }
                } else {
                    quote::quote! {
                        #visibility const fn #field_name(&self) -> &#field_type {
                            &self.#field_member
                        }
                    }
                };
                let tuple_copy_compatibility = (copy && is_single_tuple_field).then(|| {
                    quote::quote! {
                        #visibility const fn get(self) -> #field_type {
                            self.#field_member
                        }
                    }
                });
                let mutable = (container_get_mut || get_mut).then(|| {
                    let mutable_name = quote::format_ident!("{}_mut", field_name);
                    quote::quote! {
                        #visibility const fn #mutable_name(&mut self) -> &mut #field_type {
                            &mut self.#field_member
                        }
                    }
                });
                Ok(quote::quote!(#reference #value #compatibility #tuple_copy_compatibility #mutable))
            })
            .collect::<syn::Result<Vec<_>>>()?;
        Ok(quote::quote! {
        #[allow(
            dead_code,
            clippy::same_name_method,
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
