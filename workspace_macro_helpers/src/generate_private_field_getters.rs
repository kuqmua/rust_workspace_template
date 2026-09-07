pub fn generate_private_field_getters(
    proc_macro2_macro_tokens: crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens,
) -> crate::proc_macro2_macro_tokens::ProcMacro2MacroTokens {
    let parsed_input = match syn::parse2::<syn::DeriveInput>(proc_macro2::TokenStream::from(
        proc_macro2_macro_tokens,
    )) {
        Ok(value) => value,
        Err(error) => return error.into_compile_error().into(),
    };
    let generate = || {
        let syn::Data::Struct(data) = &parsed_input.data else {
            return Err(syn::Error::new_spanned(
                &parsed_input,
                constants_str::GETTERS_REQUIRES_STRUCT,
            ));
        };
        let identifier = &parsed_input.ident;
        let visibility = &parsed_input.vis;
        let (impl_generics, type_generics, where_clause) = parsed_input.generics.split_for_impl();
        let (container_bare, container_get_mut, container_legacy_refs) = parsed_input
            .attrs
            .iter()
            .try_fold((false, false, false), |found, attribute| {
                if !attribute.path().is_ident(constants_str::GETTERS_ATTRIBUTE) {
                    return Ok(found);
                }
                let mut attribute_bare = false;
                let mut attribute_get_mut = false;
                let mut attribute_legacy_refs = false;
                attribute.parse_nested_meta(|metadata| {
                    if metadata.path.is_ident(constants_str::GETTERS_BARE) {
                        attribute_bare = true;
                        Ok(())
                    } else if metadata.path.is_ident(constants_str::GETTERS_GET_MUT) {
                        attribute_get_mut = true;
                        Ok(())
                    } else if metadata.path.is_ident(constants_str::GETTERS_LEGACY_REFS) {
                        attribute_legacy_refs = true;
                        Ok(())
                    } else {
                        Err(metadata.error(constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE))
                    }
                })?;
                Ok::<(bool, bool, bool), syn::Error>((
                    found.0 || attribute_bare,
                    found.1 || attribute_get_mut,
                    found.2 || attribute_legacy_refs,
                ))
            })?;
        if container_legacy_refs && !container_bare {
            return Err(syn::Error::new_spanned(
                &parsed_input,
                constants_str::GETTERS_UNSUPPORTED_ATTRIBUTE,
            ));
        }
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
                let (field_member, field_name, legacy_reference_name) = match &field.ident {
                    Some(field_identifier) => {
                        let prefixed_name = quote::format_ident!(
                            "{}{}", constants_str::GETTER_PREFIX,
                            field_identifier.to_string().chars().enumerate().fold(
                                String::new(),
                                |mut snake_case, (character_index, character)| {
                                    if character.is_uppercase() {
                                        if character_index > constants_usize::ZERO {
                                            snake_case.push('_');
                                        }
                                        snake_case.extend(character.to_lowercase());
                                    } else {
                                        snake_case.push(character);
                                    }
                                    snake_case
                                },
                            ),
                        );
                        (quote::quote!(#field_identifier), if container_bare { field_identifier.clone() } else { prefixed_name.clone() }, Some(prefixed_name))
                    }
                    None if data.fields.len() == constants_usize::ONE => {
                        let syn_index = syn::Index::from(index);
                        (quote::quote!(#syn_index), quote::format_ident!("{}", constants_str::GET_INNER), None)
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
                    quote::format_ident!("{}", constants_str::GETTERS_REFERENCE_NAME)
                } else {
                    quote::format_ident!("{}{}", constants_str::GETTERS_REFERENCE_PREFIX, field_name.to_string().trim_start_matches(constants_str::GETTER_PREFIX))
                };
                let value_name = if is_single_tuple_field {
                    quote::format_ident!("{}", constants_str::GETTERS_VALUE_NAME)
                } else {
                    quote::format_ident!("{}{}", constants_str::GETTERS_VALUE_PREFIX, field_name.to_string().trim_start_matches(constants_str::GETTER_PREFIX))
                };
                let generate_reference = |syn_ident| { if let syn::Type::Path(type_path) = field_type
                    && type_path.qself.is_none()
                    && type_path.path.segments.len() == constants_usize::ONE
                    && let Some(option_segment) = type_path.path.segments.first()
                    && option_segment.ident == constants_str::OPTION_TYPE
                    && let syn::PathArguments::AngleBracketed(arguments) = &option_segment.arguments
                    && let Some(syn::GenericArgument::Type(inner_type)) = arguments.args.first()
                {
                    quote::quote! {
                        #visibility const fn #syn_ident(&self) -> Option<&#inner_type> {
                            self.#field_member.as_ref()
                        }
                    }
                } else {
                    quote::quote! {
                        #visibility const fn #syn_ident(&self) -> &#field_type {
                            &self.#field_member
                        }
                    }
                } };
                let reference = generate_reference(&reference_name);
                let legacy_reference = legacy_reference_name.as_ref().filter(|_name| container_legacy_refs).map(generate_reference);
                let must_use = container_legacy_refs.then(|| quote::quote!(#[must_use]));
                let value = copy.then(|| {
                    quote::quote! {
                        #visibility const fn #value_name(&self) -> #field_type {
                            self.#field_member
                        }
                    }
                });
                let compatibility = if copy && !is_single_tuple_field {
                    quote::quote! {
                        #must_use
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
                        #must_use
                        #visibility const fn #field_name(&self) -> Option<&#inner_type> {
                            self.#field_member.as_ref()
                        }
                    }
                } else {
                    quote::quote! {
                        #must_use
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
                    let mutable_name = quote::format_ident!("{}{}", field_name, constants_str::GETTERS_MUTABLE_SUFFIX);
                    quote::quote! {
                        #visibility const fn #mutable_name(&mut self) -> &mut #field_type {
                            &mut self.#field_member
                        }
                    }
                });
                Ok(quote::quote!(#reference #legacy_reference #value #compatibility #tuple_copy_compatibility #mutable))
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
