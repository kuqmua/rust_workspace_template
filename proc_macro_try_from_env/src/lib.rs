#[proc_macro_derive(TryFromEnv, attributes(config))]
pub fn try_from_env(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_location::panic_location();
    let dotenv_snake_case = naming::domain_types::DotenvSnakeCase;
    let dotenv_upper_camel_case = naming::domain_types::DotenvUpperCamelCase;
    let env_var_name_snake_case = naming::domain_types::EnvVarNameSnakeCase;
    let std_env_var_error_snake_case = naming::domain_types::StdEnvVarErrorSnakeCase;
    let std_env_var_error_upper_camel_case = naming::domain_types::StdEnvVarErrorUpperCamelCase;
    let di: syn::DeriveInput = syn::parse(token_stream).expect(constants_str::DIAGNOSTIC_E45F75C2);
    let identifier = &di.ident;
    let generate_env_example = di.attrs.iter().any(|attribute| {
        attribute.path().is_ident(constants_str::CONFIG)
            && attribute
                .parse_args::<syn::Ident>()
                .is_ok_and(|value| value == constants_str::CONFIG_ENV_EXAMPLE_ATTRIBUTE)
    });
    let identifier_try_from_env_error_upper_camel_case =
        naming::parameter::SelfTryFromEnvErrorUpperCamelCase::from_tokens(&identifier);
    let data_struct = match di.data {
        syn::Data::Struct(v0) => v0,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            std::panic::panic_any(constants_str::PANIC_54289AD5)
        }
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(v0) => v0.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            std::panic::panic_any(constants_str::PANIC_330B2512)
        }
    };
    let config_field_attributes = |field: &syn::Field| {
        let mut example = None;
        let mut accessor = false;
        let mut secret = false;
        field
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident(constants_str::CONFIG))
            .try_for_each(|attribute| {
                attribute.parse_nested_meta(|meta| {
                    if meta.path.is_ident(constants_str::EXAMPLE) {
                        example = Some(meta.value()?.parse::<syn::LitStr>()?);
                        Ok(())
                    } else if meta.path.is_ident(constants_str::ACCESSOR) {
                        accessor = true;
                        Ok(())
                    } else if meta.path.is_ident(constants_str::SECRET) {
                        secret = true;
                        Ok(())
                    } else {
                        Err(meta.error(constants_str::UNSUPPORTED_CONFIG_FIELD_ATTRIBUTE))
                    }
                })
            })?;
        Ok((example, accessor, secret))
    };
    let field_attributes = match fields_named
        .iter()
        .map(config_field_attributes)
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let field_identifier = |field: &syn::Field, exp_id: &'static str| {
        field.ident.clone().unwrap_or_else(|| {
            std::panic::panic_any(constants_str::PANIC_D8C45567.replacen(
                constants_str::PANIC_PLACEHOLDER_D8C45567,
                exp_id,
                1usize,
            ))
        })
    };
    let config_descriptors = fields_named
        .iter()
        .zip(field_attributes.iter())
        .map(|(field, attributes)| {
        let descriptor_field_identifier =
            field_identifier(field, constants_str::VALUE_8B79A379);
        let field_type = &field.ty;
        let env_name = syn::LitStr::new(
            &naming_common::domain_types::ToTokensToUpperSnakeCaseStr::case(&descriptor_field_identifier),
            identifier.span(),
        );
        let sensitivity = if attributes.2 {
            quote::quote!(config_lib::config_field_sensitivity::ConfigFieldSensitivity::Secret)
        } else {
            quote::quote!(config_lib::config_field_sensitivity::ConfigFieldSensitivity::Public)
        };
        let Some(example) = attributes.0.as_ref() else {
            return quote::quote! {
                compile_error!(constants_str::CONFIG_ENV_EXAMPLE_REQUIRES_FIELD_EXAMPLE);
            };
        };
        quote::quote! {
            config_lib::config_field_descriptor::ConfigFieldDescriptor::new(
                config_lib::env_var_name_ref::EnvVarNameRef::from(#env_name),
                config_lib::config_field_example_ref::ConfigFieldExampleRef::from(#example),
                |value| {
                    if <#field_type as config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(value).is_ok() {
                        config_lib::config_example_validity::ConfigExampleValidity::Valid
                    } else {
                        config_lib::config_example_validity::ConfigExampleValidity::Invalid
                    }
                },
                config_lib::config_field_requirement::ConfigFieldRequirement::Required,
                config_lib::config_rust_type_name::ConfigRustTypeName::from(stringify!(#field_type)),
                #sensitivity,
            )
        }
    });
    let env_example = if generate_env_example {
        let lines = fields_named
            .iter()
            .zip(field_attributes.iter())
            .map(|(field, attributes)| {
                let example_field_identifier =
                    field_identifier(field, constants_str::VALUE_8B79A379);
                let env_name = naming_common::domain_types::ToTokensToUpperSnakeCaseStr::case(
                    &example_field_identifier,
                );
                let env_name_literal = syn::LitStr::new(&env_name, identifier.span());
                attributes.0.as_ref().map_or_else(
                    || {
                        Err(syn::Error::new_spanned(
                            field,
                            constants_str::CONFIG_ENV_EXAMPLE_REQUIRES_FIELD_EXAMPLE,
                        ))
                    },
                    |example| {
                        let value = example.value();
                        let literal = if value.chars().any(|character| {
                            character.is_whitespace()
                                || matches!(character, '\'' | '"' | '\\' | '$' | '#')
                        }) {
                            let escaped = std::iter::once('"')
                                .chain(value.chars().flat_map(|character| {
                                    match character {
                                        '\\' | '"' | '$' => [Some('\\'), Some(character)],
                                        '\n' => [Some('\\'), Some('n')],
                                        _ => [Some(character), None],
                                    }
                                    .into_iter()
                                    .flatten()
                                }))
                                .chain(std::iter::once('"'))
                                .collect::<String>();
                            syn::LitStr::new(&escaped, example.span())
                        } else {
                            example.clone()
                        };
                        Ok((
                            env_name,
                            quote::quote!(#env_name_literal, "=", #literal, "\n"),
                        ))
                    },
                )
            })
            .collect::<syn::Result<Vec<_>>>();
        match lines {
            Ok(mut generated_lines) => {
                generated_lines.sort_by(|left, right| left.0.cmp(&right.0));
                let line_tokens = generated_lines.iter().map(|line| &line.1);
                quote::quote! {
                    #[must_use]
                    pub const fn env_example() -> &'static str {
                        concat!(#(#line_tokens),*)
                    }
                }
            }
            Err(error) => return error.to_compile_error().into(),
        }
    } else {
        proc_macro2::TokenStream::new()
    };
    let error_token_stream = {
        let vrts_token_stream = fields_named.iter().map(|element| {
            let element_identifier = field_identifier(element, constants_str::VALUE_2ECB63C1);
            let element_identifier_upper_camel_case_token_stream =
                naming_common::domain_types::ToTokensToUpperCamelCaseTokenStream::case_or_panic(
                    &element_identifier,
                );
            let element_ty = &element.ty;
            quote::quote! {
                #element_identifier_upper_camel_case_token_stream {
                    #element_identifier: <#element_ty as config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk>::Error,
                }
            }
        });
        quote::quote! {
            #[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
            pub enum #identifier_try_from_env_error_upper_camel_case {
                #dotenv_upper_camel_case {
                    #dotenv_snake_case: dotenv::Error,
                },
                #std_env_var_error_upper_camel_case {
                    #std_env_var_error_snake_case: std::env::VarError,
                    env_var_name: config_lib::env_var_name::EnvVarName,
                },
                #(#vrts_token_stream),*
            }
        }
    };
    let display_error_token_stream = {
        let vrts_token_stream = fields_named.iter().map(|element| {
            let element_identifier = field_identifier(element, constants_str::VALUE_8B79A379);
            let element_identifier_upper_camel_case_token_stream = naming_common::domain_types::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_identifier);
            quote::quote! {
                Self::#element_identifier_upper_camel_case_token_stream { #element_identifier } => write!(f, "{}", #element_identifier)
            }
        });
        macro_helpers::generate_impl_display_token_stream::generate_impl_display_token_stream(
            &proc_macro2::TokenStream::new(),
            &identifier_try_from_env_error_upper_camel_case,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {
                match self {
                    Self::#dotenv_upper_camel_case {
                        #dotenv_snake_case
                    } => write!(f, "{}", #dotenv_snake_case),
                    Self::#std_env_var_error_upper_camel_case {
                        #std_env_var_error_snake_case,
                        env_var_name
                    } => write!(f, "{} {}", #std_env_var_error_snake_case, env_var_name),
                    #(#vrts_token_stream),*
                }
            },
        )
    };
    let try_from_env_token_stream = {
        let fields_initialization_token_stream = fields_named.iter().map(|element| {
            let element_identifier = field_identifier(element, constants_str::EBF4E1B2);
            let element_ty = &element.ty;
            let element_identifier_quotes_upper_snake_case_string =
                syn::LitStr::new(&naming_common::domain_types::ToTokensToUpperSnakeCaseStr::case(&element_identifier), identifier.span());
            let element_identifier_upper_camel_case_token_stream = naming_common::domain_types::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&element_identifier);
            quote::quote! {
                let #element_identifier = config_lib::parse_required_env_var::parse_required_env_var(
                    config_lib::env_var_name_ref::EnvVarNameRef::from(#element_identifier_quotes_upper_snake_case_string),
                    |#std_env_var_error_snake_case, #env_var_name_snake_case| #identifier_try_from_env_error_upper_camel_case::#std_env_var_error_upper_camel_case {
                        #std_env_var_error_snake_case,
                        #env_var_name_snake_case,
                    },
                    |v| <
                        #element_ty as
                        config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk
                    >::try_from_std_env_var_ok(v),
                    |#element_identifier| #identifier_try_from_env_error_upper_camel_case::#element_identifier_upper_camel_case_token_stream {
                        #element_identifier,
                    },
                )?;
            }
        });
        let fields_token_stream = fields_named.iter().map(|element| &element.ident);
        let accessors_token_stream = fields_named
            .iter()
            .zip(field_attributes.iter())
            .filter(|(_field, attributes)| attributes.1)
            .map(|(field, _attributes)| {
                let accessor_identifier = field_identifier(field, constants_str::VALUE_8B79A379);
                let field_type = &field.ty;
                quote::quote! {
                    #[must_use]
                    pub const fn #accessor_identifier(&self) -> &#field_type {
                        &self.#accessor_identifier
                    }
                }
            });
        quote::quote! {
            impl #identifier {
                #env_example
                #(#accessors_token_stream)*
                pub fn field_descriptors() -> Vec<config_lib::config_field_descriptor::ConfigFieldDescriptor> {
                    vec![#(#config_descriptors),*]
                }
                pub fn try_from_env() -> Result<Self, #identifier_try_from_env_error_upper_camel_case> {
                    if let Err(error) = dotenv::dotenv() {
                        return Err(#identifier_try_from_env_error_upper_camel_case::#dotenv_upper_camel_case {
                            #dotenv_snake_case: error,
                        });
                    }
                    #(#fields_initialization_token_stream)*
                    Ok(Self {
                        #(#fields_token_stream),*
                    })
                }
            }
        }
    };
    let generated = quote::quote! {
        #error_token_stream
        #display_error_token_stream
        #try_from_env_token_stream
    };
    generated.into()
}
