#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "proc-macro parser models precede their entrypoints while related derive parsers remain adjacent"
)]
mod domain_types;
#[allow(
    clippy::single_call_fn,
    reason = "a named parser keeps derive expansion focused and is exercised directly by parser tests"
)]
fn parse_contract_struct_api_args(
    attributes: domain_types::SynAttributesRef<'_>,
) -> syn::Result<domain_types::ContractStructApiArgs> {
    let mut args = domain_types::ContractStructApiArgs::default();
    attributes
        .get()
        .iter()
        .filter(|attribute| {
            attribute
                .path()
                .is_ident(constants_str::CONTRACT_STRUCT_API)
        })
        .try_for_each(|attribute| {
            attribute.parse_nested_meta(|metadata| {
                if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_NEW)
                {
                    args.new = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_INTO_PARTS)
                {
                    args.into_parts = domain_types::StdBool::from(true);
                    Ok(())
                } else {
                    Err(metadata.error(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE))
                }
            })
        })?;
    Ok(args)
}
#[allow(
    clippy::single_call_fn,
    reason = "a named parser keeps derive expansion focused and is exercised directly by parser tests"
)]
fn parse_contract_struct_api_field_args(
    attributes: domain_types::SynAttributesRef<'_>,
) -> syn::Result<domain_types::ContractStructApiFieldArgs> {
    let mut args = domain_types::ContractStructApiFieldArgs::default();
    attributes
        .get()
        .iter()
        .filter(|attribute| {
            attribute
                .path()
                .is_ident(constants_str::CONTRACT_STRUCT_API)
        })
        .try_for_each(|attribute| {
            attribute.parse_nested_meta(|metadata| {
                if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_BORROW)
                {
                    args.borrow = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_COPY)
                {
                    args.copy = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_COPY_REF)
                {
                    args.copy_ref = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_INTO)
                {
                    args.into = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_OPTION_BORROW)
                {
                    args.option_borrow = domain_types::StdBool::from(true);
                    Ok(())
                } else if metadata
                    .path
                    .is_ident(constants_str::CONTRACT_STRUCT_API_SLICE)
                {
                    args.slice = Some(domain_types::SynType::from(
                        metadata.value()?.parse::<syn::Type>()?,
                    ));
                    Ok(())
                } else {
                    Err(metadata.error(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE))
                }
            })
        })?;
    Ok(args)
}

impl syn::parse::Parse for domain_types::PageCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::PAGE_CATALOG_INVENTORY {
                inventory = Some(domain_types::SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::PAGE_CATALOG_PATH_REF {
                path_ref = Some(domain_types::SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::PAGE_CATALOG_SPEC {
                spec = Some(domain_types::SynIdent::from(input.parse::<syn::Ident>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            path_ref: path_ref
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            spec: spec
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
        })
    }
}
impl syn::parse::Parse for domain_types::PageCatalogPageArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut capability = None;
        let mut metadata = None;
        let mut path = None;
        let mut route = None;
        let mut title = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::PAGE_CATALOG_CAPABILITY {
                capability = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == constants_str::PAGE_CATALOG_METADATA {
                metadata = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == constants_str::ROUTE_CATALOG_PATH {
                path = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == constants_str::PAGE_CATALOG_ROUTE {
                route = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == constants_str::PAGE_CATALOG_TITLE {
                title = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            capability: capability
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            metadata: metadata
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            path: path
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            route: route
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            title: title
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
        })
    }
}
impl syn::parse::Parse for domain_types::RouteCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut body_limit = None;
        let mut family = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::ROUTE_CATALOG_FAMILY {
                family = Some(domain_types::SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::ROUTE_CATALOG_BODY_LIMIT {
                body_limit = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            body_limit: body_limit
                .ok_or_else(|| input.error(constants_str::ROUTE_CATALOG_REQUIRES_BODY_LIMIT))?,
            family: family
                .ok_or_else(|| input.error(constants_str::ROUTE_CATALOG_REQUIRES_FAMILY))?,
        })
    }
}
impl syn::parse::Parse for domain_types::RouteCatalogRouteArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
            let mut contract = None;
            let mut exclude_from_family = domain_types::StdBool::from(false);
            let mut path = None;
            while !input.is_empty() {
                let name = input.parse::<syn::Ident>()?;
                if name == constants_str::ROUTE_CATALOG_EXCLUDE_FROM_FAMILY {
                    exclude_from_family = domain_types::StdBool::from(true);
                } else {
                    let _equals = input.parse::<syn::Token![=]>()?;
                    if name == constants_str::ROUTE_CATALOG_CONTRACT {
                        contract = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                    } else if name == constants_str::ROUTE_CATALOG_PATH {
                        path = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                    } else {
                        return Err(syn::Error::new_spanned(
                            name,
                            constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                        ));
                    }
                }
                if !input.is_empty() {
                    let _comma = input.parse::<syn::Token![,]>()?;
                }
            }
            if contract.is_none() || path.is_none() {
                return Err(
                    input.error(constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES)
                );
            }
            Ok(Self {
                contract,
                exclude_from_family,
                path,
                route: None,
            })
        } else {
            Ok(Self {
                contract: None,
                exclude_from_family: domain_types::StdBool::from(false),
                path: None,
                route: Some(domain_types::SynType::from(input.parse::<syn::Type>()?)),
            })
        }
    }
}
impl syn::parse::Parse for domain_types::EndpointRegistryBinding {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in input);
        let contract =
            domain_types::SynEndpointRegistryContract::from(content.parse::<syn::Expr>()?);
        let _contract_comma = content.parse::<syn::Token![,]>()?;
        let endpoint =
            domain_types::SynEndpointRegistryEndpoint::from(content.parse::<syn::Path>()?);
        Ok(Self { contract, endpoint })
    }
}

impl syn::parse::Parse for domain_types::EndpointRegistryArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = input.parse::<syn::Ident>()?;
        if state_name != constants_str::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                constants_str::ENDPOINT_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = input.parse::<syn::Token![=]>()?;
        let state = domain_types::SynEndpointRegistryState::from(input.parse::<syn::Type>()?);
        let _semicolon = input.parse::<syn::Token![;]>()?;
        let bindings = syn::punctuated::Punctuated::<
            domain_types::EndpointRegistryBinding,
            syn::Token![,],
        >::parse_terminated(input)?;
        if bindings.is_empty() {
            return Err(input.error(constants_str::ENDPOINT_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self {
            bindings: domain_types::SynEndpointRegistryBindings::from(bindings),
            state,
        })
    }
}

impl syn::parse::Parse for domain_types::RouteRegistryBinding {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in input);
        let route = domain_types::SynRouteRegistryRoute::from(content.parse::<syn::Type>()?);
        let _comma = content.parse::<syn::Token![,]>()?;
        let endpoint = domain_types::SynRouteRegistryEndpoint::from(content.parse::<syn::Path>()?);
        Ok(Self { endpoint, route })
    }
}
#[proc_macro_attribute]
pub fn endpoint_registry(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_args = match syn::parse::<domain_types::EndpointRegistryArgs>(attribute_args) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let item = match syn::parse::<syn::ItemStruct>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = &item.ident;
    let visibility = &item.vis;
    let state = parsed_args.state.into_inner();
    let contracts = parsed_args
        .bindings
        .as_ref()
        .iter()
        .map(|binding| binding.contract.as_ref())
        .collect::<Vec<_>>();
    let endpoints = parsed_args
        .bindings
        .as_ref()
        .iter()
        .map(|binding| binding.endpoint.as_ref())
        .collect::<Vec<_>>();
    quote::quote! {
        #item
        impl #identifier {
            #visibility fn router() -> axum::Router<#state> {
                axum::Router::new()
                    #(.route(
                        frontend_contract::domain_types::RouteRegistrationContract::path(#contracts).get(),
                        frontend_contract::domain_types::route_method_router(
                            frontend_contract::domain_types::RouteRegistrationContract::method(#contracts),
                            #endpoints,
                        ).into(),
                    ))*
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn route_openapi(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function = match syn::parse::<syn::ItemFn>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let parsed_metadata_items = match syn::parse::Parser::parse(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        attribute_args,
    ) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let mut delegate = None;
    let metadata_items = parsed_metadata_items
        .into_iter()
        .filter_map(|metadata| {
            if metadata
                .path()
                .is_ident(constants_str::ROUTE_OPENAPI_DELEGATE)
            {
                let syn::Meta::NameValue(name_value) = metadata else {
                    delegate = Some(Err(syn::Error::new_spanned(
                        metadata,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_PATH,
                    )));
                    return None;
                };
                let syn::Expr::Path(path) = name_value.value else {
                    delegate = Some(Err(syn::Error::new_spanned(
                        name_value.value,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_PATH,
                    )));
                    return None;
                };
                delegate = Some(Ok(path.path));
                None
            } else {
                Some(metadata)
            }
        })
        .collect::<Vec<_>>();
    let metadata = quote::quote! { #(#metadata_items),* };
    if let Some(delegate_result) = delegate {
        let delegate_path = match delegate_result {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        };
        if !function.block.stmts.is_empty() {
            return syn::Error::new_spanned(
                function.block,
                constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_EMPTY_BODY,
            )
            .to_compile_error()
            .into();
        }
        let parameters = match function
            .sig
            .inputs
            .iter()
            .map(|argument| {
                let syn::FnArg::Typed(typed_argument) = argument else {
                    return Err(syn::Error::new_spanned(
                        argument,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_IDENT_PARAMETERS,
                    ));
                };
                let syn::Pat::Ident(identifier) = typed_argument.pat.as_ref() else {
                    return Err(syn::Error::new_spanned(
                        typed_argument,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_IDENT_PARAMETERS,
                    ));
                };
                if identifier.subpat.is_some() {
                    return Err(syn::Error::new_spanned(
                        identifier,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_IDENT_PARAMETERS,
                    ));
                }
                Ok(&identifier.ident)
            })
            .collect::<syn::Result<Vec<_>>>()
        {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        };
        let error_type = match &function.sig.output {
            syn::ReturnType::Type(_arrow, return_type) => {
                let syn::Type::Path(return_path) = return_type.as_ref() else {
                    return syn::Error::new_spanned(
                        return_type,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                    )
                    .to_compile_error()
                    .into();
                };
                let Some(result_segment) = return_path.path.segments.last() else {
                    return syn::Error::new_spanned(
                        return_type,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                    )
                    .to_compile_error()
                    .into();
                };
                if result_segment.ident != constants_str::RESULT_UPPER_CAMEL_CASE {
                    return syn::Error::new_spanned(
                        return_type,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                    )
                    .to_compile_error()
                    .into();
                }
                let syn::PathArguments::AngleBracketed(arguments) = &result_segment.arguments
                else {
                    return syn::Error::new_spanned(
                        return_type,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                    )
                    .to_compile_error()
                    .into();
                };
                let Some(syn::GenericArgument::Type(error_type)) = arguments.args.iter().nth(1)
                else {
                    return syn::Error::new_spanned(
                        return_type,
                        constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                    )
                    .to_compile_error()
                    .into();
                };
                error_type
            }
            syn::ReturnType::Default => {
                return syn::Error::new_spanned(
                    &function.sig,
                    constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT,
                )
                .to_compile_error()
                .into();
            }
        };
        function.block = Box::new(syn::parse_quote!({
            #delegate_path(#(#parameters),*)
                .await
                .map_err(#error_type::from)
        }));
        let reason = syn::LitStr::new(
            constants_str::ROUTE_OPENAPI_SINGLE_CALL_REASON,
            proc_macro2::Span::call_site(),
        );
        function.attrs.push(syn::parse_quote!(
            #[allow(clippy::single_call_fn, reason = #reason)]
        ));
    }
    let dummy_path = syn::LitStr::new(
        format!("/__typed_route_{}", function.sig.ident).as_str(),
        proc_macro2::Span::call_site(),
    );
    quote::quote! {
        #[utoipa::path(get, path = #dummy_path, #metadata)]
        #function
    }
    .into()
}

#[proc_macro_derive(ContractStructApi, attributes(contract_struct_api))]
pub fn derive_contract_struct_api(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let args = match parse_contract_struct_api_args(domain_types::SynAttributesRef::from(
        derive_input.attrs.as_slice(),
    )) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Struct(data) = &derive_input.data else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::CONTRACT_STRUCT_API_REQUIRES_NAMED_STRUCT,
        )
        .to_compile_error()
        .into();
    };
    let syn::Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::CONTRACT_STRUCT_API_REQUIRES_NAMED_STRUCT,
        )
        .to_compile_error()
        .into();
    };
    let parsed_fields = match fields
        .named
        .iter()
        .map(|field| {
            let Some(identifier) = field.ident.as_ref() else {
                return Err(syn::Error::new_spanned(
                    field,
                    constants_str::CONTRACT_STRUCT_API_REQUIRES_NAMED_STRUCT,
                ));
            };
            parse_contract_struct_api_field_args(domain_types::SynAttributesRef::from(
                field.attrs.as_slice(),
            ))
            .map(|field_args| (identifier, &field.ty, field_args))
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifiers = parsed_fields
        .iter()
        .map(|(identifier, _field_type, _args)| *identifier)
        .collect::<Vec<_>>();
    let types = parsed_fields
        .iter()
        .map(|(_identifier, field_type, _args)| *field_type)
        .collect::<Vec<_>>();
    let constructor = bool::from(args.new).then(|| {
        quote::quote! {
            #[must_use]
            pub const fn new(#(#identifiers: #types),*) -> Self {
                Self { #(#identifiers),* }
            }
        }
    });
    let into_parts = bool::from(args.into_parts).then(|| {
        quote::quote! {
            #[must_use]
            pub fn into_parts(self) -> (#(#types,)*) {
                (#(self.#identifiers,)*)
            }
        }
    });
    let accessors = parsed_fields
        .iter()
        .flat_map(|(identifier, field_type, field_args)| {
            let borrowed = bool::from(field_args.borrow).then(|| {
                quote::quote! {
                    #[must_use]
                    pub const fn #identifier(&self) -> &#field_type {
                        &self.#identifier
                    }
                }
            });
            let copied = bool::from(field_args.copy).then(|| {
                quote::quote! {
                    #[must_use]
                    pub const fn #identifier(self) -> #field_type {
                        self.#identifier
                    }
                }
            });
            let consumed = bool::from(field_args.into).then(|| {
                let method = quote::format_ident!(
                    "{}_{}",
                    constants_str::CONTRACT_STRUCT_API_INTO,
                    identifier
                );
                quote::quote! {
                    #[must_use]
                    pub fn #method(self) -> #field_type {
                        self.#identifier
                    }
                }
            });
            let copied_ref = bool::from(field_args.copy_ref).then(|| {
                quote::quote! {
                    #[must_use]
                    pub const fn #identifier(&self) -> #field_type {
                        self.#identifier
                    }
                }
            });
            let option_borrowed = bool::from(field_args.option_borrow).then(|| {
                let syn::Type::Path(option_path) = field_type else {
                    return syn::Error::new_spanned(
                        field_type,
                        constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE,
                    )
                    .to_compile_error();
                };
                let Some(option_segment) = option_path.path.segments.last() else {
                    return syn::Error::new_spanned(
                        field_type,
                        constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE,
                    )
                    .to_compile_error();
                };
                let syn::PathArguments::AngleBracketed(arguments) = &option_segment.arguments
                else {
                    return syn::Error::new_spanned(
                        field_type,
                        constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE,
                    )
                    .to_compile_error();
                };
                let Some(syn::GenericArgument::Type(inner_type)) = arguments.args.first() else {
                    return syn::Error::new_spanned(
                        field_type,
                        constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE,
                    )
                    .to_compile_error();
                };
                quote::quote! {
                    #[must_use]
                    pub const fn #identifier(&self) -> Option<&#inner_type> {
                        self.#identifier.as_ref()
                    }
                }
            });
            let slice = field_args.slice.as_ref().map(|wrapped_element_type| {
                let element_type = &wrapped_element_type.as_ref();
                quote::quote! {
                    #[must_use]
                    pub const fn #identifier(&self) -> &[#element_type] {
                        self.#identifier.as_slice()
                    }
                }
            });
            [
                borrowed,
                copied,
                copied_ref,
                consumed,
                option_borrowed,
                slice,
            ]
            .into_iter()
            .flatten()
        });
    let identifier = &derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics #identifier #type_generics #where_clause {
            #constructor
            #into_parts
            #(#accessors)*
        }
    }
    .into()
}

impl syn::parse::Parse for domain_types::RouteRegistryArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = input.parse::<syn::Ident>()?;
        if state_name != constants_str::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = input.parse::<syn::Token![=]>()?;
        let state = domain_types::SynRouteRegistryState::from(input.parse::<syn::Type>()?);
        let _state_comma = input.parse::<syn::Token![,]>()?;
        let family_name = input.parse::<syn::Ident>()?;
        if family_name != constants_str::FAMILY {
            return Err(syn::Error::new_spanned(
                family_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_FAMILY,
            ));
        }
        let _family_equals = input.parse::<syn::Token![=]>()?;
        let family = domain_types::SynRouteRegistryFamily::from(input.parse::<syn::Type>()?);
        let _family_semicolon = input.parse::<syn::Token![;]>()?;
        let security_content;
        let _security_parenthesis = syn::parenthesized!(security_content in input);
        let authenticated_security =
            domain_types::SynExpr::from(security_content.parse::<syn::Expr>()?);
        let _comma = security_content.parse::<syn::Token![,]>()?;
        let csrf_security = domain_types::SynExpr::from(security_content.parse::<syn::Expr>()?);
        let _security_semicolon = input.parse::<syn::Token![;]>()?;
        let schemas_name = input.parse::<syn::Ident>()?;
        if schemas_name != constants_str::SCHEMAS {
            return Err(syn::Error::new_spanned(
                schemas_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_SCHEMAS,
            ));
        }
        let schemas_content;
        let _schemas_parenthesis = syn::parenthesized!(schemas_content in input);
        let schemas = syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated(
            &schemas_content,
        )?
        .into_iter()
        .collect::<Vec<_>>();
        let _schemas_semicolon = input.parse::<syn::Token![;]>()?;
        let bindings = syn::punctuated::Punctuated::<
            domain_types::RouteRegistryBinding,
            syn::Token![,],
        >::parse_terminated(input)?;
        if bindings.is_empty() {
            return Err(input.error(constants_str::ROUTE_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self {
            authenticated_security,
            bindings: domain_types::SynRouteRegistryBindings::from(bindings),
            csrf_security,
            family,
            schemas: domain_types::SynRouteRegistrySchemas::from(schemas),
            state,
        })
    }
}

#[proc_macro_attribute]
pub fn route_registry(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_args = match syn::parse::<domain_types::RouteRegistryArgs>(attribute_args) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let mut item = match syn::parse::<syn::ItemStruct>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(openapi_attribute_index) = item
        .attrs
        .iter()
        .position(|attribute| attribute.path().is_ident(constants_str::OPENAPI))
    else {
        return syn::Error::new_spanned(
            item.ident,
            constants_str::ROUTE_REGISTRY_REQUIRES_OPENAPI_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let openapi_attribute = item.attrs.remove(openapi_attribute_index);
    let openapi_metadata = match openapi_attribute.meta {
        syn::Meta::List(value) => value.tokens,
        value @ (syn::Meta::Path(_) | syn::Meta::NameValue(_)) => {
            return syn::Error::new_spanned(
                value,
                constants_str::ROUTE_REGISTRY_REQUIRES_OPENAPI_ATTRIBUTE,
            )
            .to_compile_error()
            .into();
        }
    };
    let identifier = &item.ident;
    let visibility = &item.vis;
    let unique_route_trait_identifier = quote::format_ident!("{}UniqueRoute", identifier);
    let openapi_identifier = quote::format_ident!("{}OpenApi", identifier);
    let state = parsed_args.state.into_inner();
    let family = parsed_args.family.into_inner();
    let authenticated_security = parsed_args.authenticated_security.into_inner();
    let csrf_security = parsed_args.csrf_security.into_inner();
    let schemas = parsed_args.schemas.into_inner();
    let routes = parsed_args
        .bindings
        .as_ref()
        .iter()
        .map(|binding| binding.route.as_ref())
        .collect::<Vec<_>>();
    let endpoints = parsed_args
        .bindings
        .as_ref()
        .iter()
        .map(|binding| binding.endpoint.as_ref())
        .collect::<Vec<_>>();
    let route_count = routes.len();
    let openapi_paths = parsed_args
        .bindings
        .as_ref()
        .iter()
        .map(|binding| {
            let mut path = binding.endpoint.as_ref().clone();
            if let Some(last_segment) = path.segments.last_mut() {
                last_segment.ident = quote::format_ident!("__path_{}", last_segment.ident);
            }
            path
        })
        .collect::<Vec<_>>();
    quote::quote! {
        #item
        trait #unique_route_trait_identifier {}
        #(impl #unique_route_trait_identifier for #routes {})*
        const _: [(); <#family as frontend_contract::domain_types::RouteFamily>::ROUTE_COUNT] =
            [(); #route_count];
        #[allow(clippy::needless_for_each)]
        #[derive(utoipa::OpenApi)]
        #[openapi(paths(#(#endpoints),*), #openapi_metadata)]
        struct #openapi_identifier;
        impl #identifier {
            #visibility fn assert_route_family_membership<Route>()
            where
                Route: frontend_contract::domain_types::RouteInFamily<#family> + #unique_route_trait_identifier,
            {
            }
            #visibility fn open_api() -> utoipa::openapi::OpenApi {
                let mut document = <#openapi_identifier as utoipa::OpenApi>::openapi();
                #({
                    let components = document
                        .components
                        .get_or_insert_with(utoipa::openapi::schema::Components::new);
                    let mut schema_components =
                        frontend_contract::domain_types::UtoipaOpenApiComponentsRefMut::from(components);
                    frontend_contract::domain_types::register_openapi_schema::<#schemas>(
                        &mut schema_components
                    );
                })*
                document.paths = utoipa::openapi::path::Paths::new();
                #({
                    let mut open_api = frontend_contract::domain_types::UtoipaOpenApiRefMut::from(&mut document);
                    frontend_contract::domain_types::register_openapi_route_schemas::<#routes>(
                        &mut open_api
                    );
                    let metadata = <#routes as frontend_contract::domain_types::TypedRoute>::metadata();
                    let mut operation = <#openapi_paths as utoipa::Path>::operation();
                    {
                        operation.operation_id = Some(metadata.openapi_operation_id().as_ref().to_owned());
                        frontend_contract::domain_types::apply_openapi_request_contract::<#routes>(&mut operation);
                        frontend_contract::domain_types::apply_openapi_success_contract::<#routes>(&mut operation);
                        frontend_contract::domain_types::apply_openapi_error_contract::<#routes>(&mut operation);
                        frontend_contract::domain_types::apply_openapi_path_parameter_contract::<#routes>(&mut operation);
                        frontend_contract::domain_types::apply_openapi_security_contract::<#routes>(
                            &mut operation,
                            frontend_contract::domain_types::OpenApiSecuritySchemeRef::from(#authenticated_security),
                            frontend_contract::domain_types::OpenApiSecuritySchemeRef::from(#csrf_security),
                        );
                        let path_item_type = match metadata.route_method() {
                            frontend_contract::domain_types::RouteMethod::Connect => None,
                            frontend_contract::domain_types::RouteMethod::Delete => Some(utoipa::openapi::path::HttpMethod::Delete),
                            frontend_contract::domain_types::RouteMethod::Get => Some(utoipa::openapi::path::HttpMethod::Get),
                            frontend_contract::domain_types::RouteMethod::Head => Some(utoipa::openapi::path::HttpMethod::Head),
                            frontend_contract::domain_types::RouteMethod::Options => Some(utoipa::openapi::path::HttpMethod::Options),
                            frontend_contract::domain_types::RouteMethod::Patch => Some(utoipa::openapi::path::HttpMethod::Patch),
                            frontend_contract::domain_types::RouteMethod::Post => Some(utoipa::openapi::path::HttpMethod::Post),
                            frontend_contract::domain_types::RouteMethod::Put => Some(utoipa::openapi::path::HttpMethod::Put),
                            frontend_contract::domain_types::RouteMethod::Trace => Some(utoipa::openapi::path::HttpMethod::Trace),
                        };
                        if let Some(path_item_type) = path_item_type {
                            let path_item = utoipa::openapi::path::PathItem::new(path_item_type, operation);
                            document
                                .paths
                                .paths
                                .entry(metadata.path().as_ref().to_owned())
                                .and_modify(|existing| existing.merge_operations(path_item.clone()))
                                .or_insert(path_item);
                        }
                    }
                })*
                document
            }
            #visibility fn router() -> axum::Router<#state> {
                #(Self::assert_route_family_membership::<#routes>();)*
                axum::Router::new()
                    #(.route(
                        frontend_contract::domain_types::typed_route_path::<#routes>().as_ref(),
                        axum::routing::on(
                            axum::routing::MethodFilter::from(
                                frontend_contract::domain_types::axum_method_filter(
                                    <#routes as frontend_contract::domain_types::TypedRoute>::metadata()
                                        .contract()
                                        .method()
                                )
                            ),
                            #endpoints,
                        ),
                    ))*
            }
        }
    }
    .into()
}

impl syn::parse::Parse for domain_types::TypedRouteArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut method = None;
        let mut authentication = None;
        let mut error_response = None;
        let mut error_policy = None;
        let mut error_statuses = None;
        let mut mutation = None;
        let mut obligations = None;
        let mut openapi_operation_id = None;
        let mut path = None;
        let mut path_parameter = None;
        let mut request = None;
        let mut request_body = None;
        let mut response = None;
        let mut success_status = None;
        let mut transport = None;
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            let _equals: syn::Token![=] = input.parse()?;
            match name.to_string().as_str() {
                constants_str::TYPED_ROUTE_FIELD_AUTHENTICATION => {
                    authentication = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_ERROR_STATUSES => {
                    error_statuses = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_ERROR_RESPONSE => {
                    error_response = Some(domain_types::SynType::from(input.parse::<syn::Type>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_ERROR_POLICY => {
                    error_policy = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::METHOD => {
                    method = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::OPENAPI_OPERATION_ID => {
                    openapi_operation_id =
                        Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::MUTATION => {
                    mutation = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::OBLIGATIONS => {
                    obligations = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_PATH => {
                    path = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_PATH_PARAMETER => {
                    path_parameter = Some(domain_types::SynType::from(input.parse::<syn::Type>()?));
                }
                constants_str::REQUEST => {
                    request = Some(domain_types::SynType::from(input.parse::<syn::Type>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_REQUEST_BODY => {
                    request_body = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::RESPONSE => {
                    response = Some(domain_types::SynType::from(input.parse::<syn::Type>()?));
                }
                constants_str::TYPED_ROUTE_FIELD_SUCCESS_STATUS => {
                    success_status = Some(domain_types::SynExpr::from(input.parse::<syn::Expr>()?));
                }
                constants_str::TRANSPORT => {
                    transport = Some(domain_types::SynType::from(input.parse::<syn::Type>()?));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                    ));
                }
            }
            if !input.is_empty() {
                let _comma: syn::Token![,] = input.parse()?;
            }
        }
        let errors = match (error_policy, error_statuses) {
            (Some(policy), None) => domain_types::SynTypedRouteErrors::Policy(policy),
            (None, Some(statuses)) => domain_types::SynTypedRouteErrors::Statuses(statuses),
            (None, None) | (Some(_), Some(_)) => {
                return Err(
                    input.error(constants_str::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES)
                );
            }
        };
        Ok(Self {
            authentication: authentication
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_AUTHENTICATION))?,
            error_response,
            errors,
            method: method
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_METHOD))?,
            mutation,
            obligations,
            openapi_operation_id: openapi_operation_id
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_OPERATION_ID))?,
            path: path.ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_PATH))?,
            path_parameter,
            request: request
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_REQUEST))?,
            request_body,
            response: response
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_RESPONSE))?,
            success_status: success_status
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_SUCCESS_STATUS))?,
            transport: transport
                .ok_or_else(|| input.error(constants_str::TYPED_ROUTE_REQUIRES_TRANSPORT))?,
        })
    }
}

#[proc_macro_derive(TypedRoute, attributes(typed_route))]
#[allow(
    clippy::wildcard_enum_match_arm,
    reason = "typed route methods intentionally reject every current and future non-path expression"
)]
pub fn derive_typed_route(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(attribute) = derive_input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(constants_str::TYPED_ROUTE))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::TYPED_ROUTE_DERIVE_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match attribute.parse_args::<domain_types::TypedRouteArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let visibility = derive_input.vis;
    let identifier = derive_input.ident;
    let method = match args.method.into_inner() {
        syn::Expr::Path(path_expression) => {
            match path_expression
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
            {
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::CONNECT) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Connect)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::DELETE) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Delete)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::GET) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Get)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::HEAD) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Head)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::OPTIONS) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Options)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::PATCH) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Patch)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::POST) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Post)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::PUT) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Put)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(constants_str::TRACE) => {
                    quote::quote!(frontend_contract::domain_types::RouteMethod::Trace)
                }
                _ => {
                    return syn::Error::new_spanned(
                        path_expression,
                        constants_str::TYPED_ROUTE_METHOD_MUST_BE_STANDARD_HTTP_METHOD,
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        value => {
            return syn::Error::new_spanned(
                value,
                constants_str::TYPED_ROUTE_METHOD_MUST_BE_STANDARD_HTTP_METHOD,
            )
            .to_compile_error()
            .into();
        }
    };
    let authentication = args.authentication.into_inner();
    let mutation = args.mutation.map_or_else(
        || quote::quote!(frontend_contract::domain_types::RouteMutation::ReadOnly),
        |value| quote::ToTokens::into_token_stream(value.as_ref()),
    );
    let error_statuses = match args.errors {
        domain_types::SynTypedRouteErrors::Policy(value) => {
            let policy = value.into_inner();
            quote::quote!((#policy).statuses(#authentication, #mutation))
        }
        domain_types::SynTypedRouteErrors::Statuses(value) => {
            quote::ToTokens::into_token_stream(value.as_ref())
        }
    };
    let (error_response_schema, error_response_schema_registration) = match args
        .error_response
        .map(domain_types::SynType::into_inner)
    {
        Some(syn::Type::Tuple(tuple)) if tuple.elems.is_empty() => (
            quote::quote! {
                fn openapi_error_response_schema(
                    _status: frontend_contract::domain_types::RouteErrorStatus,
                ) -> Option<frontend_contract::domain_types::UtoipaOpenApiRouteSchema> {
                    None
                }
            },
            proc_macro2::TokenStream::new(),
        ),
        Some(response_type) => (
            quote::quote! {
                fn openapi_error_response_schema(
                    _status: frontend_contract::domain_types::RouteErrorStatus,
                ) -> Option<frontend_contract::domain_types::UtoipaOpenApiRouteSchema> {
                    Some(frontend_contract::domain_types::UtoipaOpenApiRouteSchema::from(
                        <#response_type as utoipa::PartialSchema>::schema()
                    ))
                }
            },
            quote::quote! {
                frontend_contract::domain_types::register_openapi_schema::<#response_type>(components);
            },
        ),
        None => (
            proc_macro2::TokenStream::new(),
            proc_macro2::TokenStream::new(),
        ),
    };
    let obligations = args.obligations.map_or_else(
        || quote::quote!(&[]),
        |value| quote::ToTokens::into_token_stream(value.as_ref()),
    );
    let literal_operation_name = match args.openapi_operation_id.as_ref() {
        syn::Expr::Lit(expression) => match &expression.lit {
            syn::Lit::Str(value) => Some(value.value()),
            _ => None,
        },
        _ => None,
    };
    let operation_name = literal_operation_name.unwrap_or_else(|| {
        let identifier_value = identifier.to_string();
        identifier_value
            .strip_suffix(constants_str::VALUE_ADC74704)
            .unwrap_or(identifier_value.as_str())
            .chars()
            .enumerate()
            .fold(
                String::with_capacity(identifier_value.len().saturating_mul(2usize)),
                |mut value, (index, character)| {
                    if character.is_ascii_uppercase() {
                        if index != constants_usize::ZERO {
                            value.push('_');
                        }
                        value.push(character.to_ascii_lowercase());
                    } else {
                        value.push(character);
                    }
                    value
                },
            )
    });
    let route_function_identifier =
        quote::format_ident!("{}_route", operation_name, span = identifier.span());
    let client_function_identifier =
        quote::format_ident!("{}_client", operation_name, span = identifier.span());
    let openapi_operation_id = args.openapi_operation_id.into_inner();
    let mut openapi_path_parameter = quote::quote!(None);
    let mut named_route_and_client = quote::quote! {
        #[must_use]
        #visibility fn #route_function_identifier() -> frontend_contract::domain_types::ContractStr {
            frontend_contract::domain_types::typed_route_path::<#identifier>()
        }
        #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
        #visibility async fn #client_function_identifier<Transport>(
            client: &frontend_contract::domain_types::TypedClient<Transport>,
            request: <#identifier as frontend_contract::domain_types::TypedRoute>::Request,
        ) -> Result<
            <#identifier as frontend_contract::domain_types::TypedRoute>::Response,
            frontend_contract::domain_types::ClientError,
        >
        where
            Transport: frontend_contract::domain_types::Transport,
        {
            client.send::<#identifier>(request).await
        }
    };
    let parameterized_route = match args.path_parameter.as_ref() {
        Some(parameter_type) => {
            let syn::Expr::Lit(path_expression) = args.path.as_ref() else {
                return syn::Error::new_spanned(
                    args.path.as_ref(),
                    constants_str::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
                )
                .to_compile_error()
                .into();
            };
            let syn::Lit::Str(path_literal) = &path_expression.lit else {
                return syn::Error::new_spanned(
                    &path_expression.lit,
                    constants_str::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
                )
                .to_compile_error()
                .into();
            };
            let path_value = path_literal.value();
            let Some((prefix_value, placeholder_and_suffix)) = path_value.split_once('{') else {
                return syn::Error::new_spanned(
                    path_literal,
                    constants_str::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_PLACEHOLDER,
                )
                .to_compile_error()
                .into();
            };
            let Some((placeholder, suffix_value)) = placeholder_and_suffix.split_once('}') else {
                return syn::Error::new_spanned(
                    path_literal,
                    constants_str::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_CLOSED_PLACEHOLDER,
                )
                .to_compile_error()
                .into();
            };
            if placeholder.is_empty()
                || suffix_value.contains('{')
                || prefix_value.contains('}')
                || suffix_value.contains('}')
            {
                return syn::Error::new_spanned(
                    path_literal,
                    constants_str::TYPED_ROUTE_PARAMETER_PATH_SUPPORTS_ONE_PLACEHOLDER,
                )
                .to_compile_error()
                .into();
            }
            let prefix = syn::LitStr::new(prefix_value, path_literal.span());
            let suffix = syn::LitStr::new(suffix_value, path_literal.span());
            let parameter_name = syn::LitStr::new(placeholder, path_literal.span());
            let parameter_path = parameter_type.as_ref();
            named_route_and_client = quote::quote! {
                #[must_use]
                #visibility fn #route_function_identifier(
                    parameter: &#parameter_path,
                ) -> frontend_contract::domain_types::ParameterizedRoutePath {
                    frontend_contract::domain_types::typed_parameterized_route_path::<#identifier>(parameter)
                }
                #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
                #visibility async fn #client_function_identifier<Transport>(
                    client: &frontend_contract::domain_types::TypedClient<Transport>,
                    parameter: &#parameter_path,
                    request: <#identifier as frontend_contract::domain_types::TypedRoute>::Request,
                ) -> Result<
                    <#identifier as frontend_contract::domain_types::TypedRoute>::Response,
                    frontend_contract::domain_types::ClientError,
                >
                where
                    Transport: frontend_contract::domain_types::Transport,
                {
                    client.send_parameterized::<#identifier>(parameter, request).await
                }
            };
            openapi_path_parameter = quote::quote! {
                Some(frontend_contract::domain_types::UtoipaOpenApiPathParameter::from(
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name(#parameter_name)
                        .parameter_in(utoipa::openapi::path::ParameterIn::Path)
                        .required(utoipa::openapi::Required::True)
                        .schema(Some(<#parameter_path as utoipa::PartialSchema>::schema()))
                        .build()
                ))
            };
            quote::quote! {
                impl frontend_contract::domain_types::ParameterizedRoute for #identifier {
                    type Parameter = #parameter_path;
                    fn path(parameter: &Self::Parameter) -> frontend_contract::domain_types::ParameterizedRoutePath {
                        frontend_contract::domain_types::ParameterizedRoutePath::try_from(format!("{}{}{}", #prefix, parameter, #suffix)).unwrap_or_default()
                    }
                }
            }
        }
        None => proc_macro2::TokenStream::new(),
    };
    let path = args.path.into_inner();
    let request = args.request.into_inner();
    let request_body = args.request_body.map_or_else(
        || quote::quote!(frontend_contract::domain_types::RouteRequestBody::Absent),
        |value| quote::ToTokens::into_token_stream(value.as_ref()),
    );
    let response = args.response.into_inner();
    let response_schema = match &response {
        syn::Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::VEC) =>
        {
            quote::quote! {
                Some(frontend_contract::domain_types::UtoipaOpenApiRouteSchema::from(<#response as utoipa::PartialSchema>::schema()))
            }
        }
        _ => quote::quote! {
            Some(frontend_contract::domain_types::UtoipaOpenApiRouteSchema::from(<#response as utoipa::PartialSchema>::schema()))
        },
    };
    let response_schema_registration = match &response {
        syn::Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::VEC) =>
        {
            proc_macro2::TokenStream::new()
        }
        _ => quote::quote! {
            frontend_contract::domain_types::register_openapi_schema::<#response>(components);
        },
    };
    let success_status = args.success_status.into_inner();
    let transport = args.transport.into_inner();
    quote::quote! {
        impl frontend_contract::domain_types::TypedRoute for #identifier {
            type Request = #request;
            type Response = #response;
            type Transport = #transport;
            fn metadata() -> frontend_contract::domain_types::RouteMetadata {
                frontend_contract::domain_types::RouteMetadata::new_with_policy(
                    #authentication,
                    #error_statuses,
                    #method,
                    #mutation,
                    frontend_contract::domain_types::ContractStr::from(#openapi_operation_id),
                    frontend_contract::domain_types::ContractStr::from(#path),
                    #success_status,
                )
            }
            fn openapi_request_schema() -> Option<frontend_contract::domain_types::UtoipaOpenApiRouteSchema> {
                Some(frontend_contract::domain_types::UtoipaOpenApiRouteSchema::from(<#request as utoipa::PartialSchema>::schema()))
            }
            fn openapi_request_body_schema() -> Option<frontend_contract::domain_types::UtoipaOpenApiRouteSchema> {
                let name = <#request as utoipa::ToSchema>::name();
                Some(frontend_contract::domain_types::UtoipaOpenApiRouteSchema::from(
                    utoipa::openapi::RefOr::Ref(utoipa::openapi::Ref::from_schema_name(name.as_ref()))
                ))
            }
            fn request_body() -> frontend_contract::domain_types::RouteRequestBody {
                #request_body
            }
            fn openapi_response_schema() -> Option<frontend_contract::domain_types::UtoipaOpenApiRouteSchema> {
                #response_schema
            }
            #error_response_schema
            fn openapi_path_parameter() -> Option<frontend_contract::domain_types::UtoipaOpenApiPathParameter> {
                #openapi_path_parameter
            }
            fn register_openapi_schemas(
                components: &mut frontend_contract::domain_types::UtoipaOpenApiComponentsRefMut<'_>,
            ) {
                if <Self as frontend_contract::domain_types::TypedRoute>::request_body()
                    == frontend_contract::domain_types::RouteRequestBody::Json
                {
                    frontend_contract::domain_types::register_openapi_schema::<#request>(components);
                }
                #response_schema_registration
                #error_response_schema_registration
            }
        }
        impl frontend_contract::domain_types::CoveredRoute for #identifier {
            fn coverage_descriptor() -> frontend_contract::domain_types::RouteCoverageDescriptor {
                let metadata = <Self as frontend_contract::domain_types::TypedRoute>::metadata();
                frontend_contract::domain_types::RouteCoverageDescriptor::new(
                    metadata,
                    metadata.access(),
                    metadata.mutation(),
                    frontend_contract::domain_types::RouteCoverageEvidence::new(#obligations),
                )
            }
        }
        #parameterized_route
        #named_route_and_client
    }
    .into()
}

#[proc_macro]
pub fn api_operation_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_ers = match syn::parse::Parser::parse(
        syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
        input,
    ) {
        Ok(value) => value,
        Err(parse_error) => return parse_error.to_compile_error().into(),
    };
    let mut ers = parsed_ers.into_iter();
    let Some(error) = ers.next() else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            constants_str::API_OPERATION_ERROR_REQUIRES_ERROR_TYPE,
        )
        .to_compile_error()
        .into();
    };
    if ers.next().is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            constants_str::API_OPERATION_ERROR_ACCEPTS_ONE_ERROR_TYPE,
        )
        .to_compile_error()
        .into();
    }
    quote::quote! {
        #[derive(Debug, thiserror::Error)]
        enum #error {
            #[error("administrator authentication failed")]
            Authentication,
            #[error("administrator authentication secret text is invalid")]
            AuthenticationSecretText(
                #[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>,
            ),
            #[error("administrator authorization failed")]
            Authorization,
            #[error("administrator operation conflicts with current state")]
            Conflict,
            #[error("administrator request failed CSRF validation")]
            Csrf,
            #[error("administrator CSRF secret text is invalid")]
            CsrfSecretText(
                #[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>,
            ),
            #[error("administrator authentication is temporarily rate limited")]
            RateLimited,
            #[error("administrator request validation failed")]
            Validation,
            #[error("administrator API database operation failed: {0:?}")]
            Pg(#[source] server_runtime_http::domain_types::ObservedError<super::SqlxAdminError>),
            #[error("administrator password hashing failed: {0}")]
            PasswordHash(
                #[source] server_runtime_http::domain_types::ObservedError<super::AdminPasswordHashError>,
            ),
            #[error("administrator password text is invalid")]
            PasswordText(
                #[source]
                server_runtime_http::domain_types::ObservedError<super::AdminPasswordTryFromStringError>,
            ),
            #[error("administrator request body is too large")]
            PayloadTooLarge,
            #[error("administrator secret text is invalid")]
            SecretText(#[source] server_runtime_http::domain_types::ObservedError<super::AdminSecretTextError>),
            #[error("administrator route does not support this HTTP method")]
            MethodNotAllowed,
            #[error("administrator session operation failed: {0}")]
            Session(#[source] server_runtime_http::domain_types::ObservedError<AdminSessionError>),
            #[error("administrator response header is invalid: {0:?}")]
            Header(#[source] server_runtime_http::domain_types::ObservedError<HttpAdminHeaderValueError>),
        }
        impl From<AdminError> for #error {
            fn from(value: AdminError) -> Self {
                match value {
                    AdminError::Authentication => Self::Authentication,
                    AdminError::AuthenticationSecretText(source) => {
                        Self::AuthenticationSecretText(source)
                    }
                    AdminError::Authorization => Self::Authorization,
                    AdminError::Conflict => Self::Conflict,
                    AdminError::Csrf => Self::Csrf,
                    AdminError::CsrfSecretText(source) => Self::CsrfSecretText(source),
                    AdminError::RateLimited => Self::RateLimited,
                    AdminError::Validation => Self::Validation,
                    AdminError::Pg(source) => Self::Pg(source),
                    AdminError::PasswordHash(source) => Self::PasswordHash(source),
                    AdminError::PasswordText(source) => Self::PasswordText(source),
                    AdminError::PayloadTooLarge => Self::PayloadTooLarge,
                    AdminError::SecretText(source) => Self::SecretText(source),
                    AdminError::MethodNotAllowed => Self::MethodNotAllowed,
                    AdminError::Session(source) => Self::Session(source),
                    AdminError::Header(source) => Self::Header(source),
                }
            }
        }
        impl axum::response::IntoResponse for #error {
            fn into_response(self) -> axum::response::Response {
                let route_error_status = match &self {
                    Self::Authentication | Self::AuthenticationSecretText(_) => {
                        frontend_contract::domain_types::RouteErrorStatus::Authentication
                    }
                    Self::Authorization | Self::Csrf | Self::CsrfSecretText(_) => {
                        frontend_contract::domain_types::RouteErrorStatus::Authorization
                    }
                    Self::Conflict => frontend_contract::domain_types::RouteErrorStatus::Conflict,
                    Self::MethodNotAllowed => {
                        frontend_contract::domain_types::RouteErrorStatus::MethodNotAllowed
                    }
                    Self::PayloadTooLarge => {
                        frontend_contract::domain_types::RouteErrorStatus::PayloadTooLarge
                    }
                    Self::RateLimited => frontend_contract::domain_types::RouteErrorStatus::RateLimited,
                    Self::Validation | Self::PasswordText(_) | Self::SecretText(_) => {
                        frontend_contract::domain_types::RouteErrorStatus::Validation
                    }
                    Self::Pg(_)
                    | Self::PasswordHash(_)
                    | Self::Session(_)
                    | Self::Header(_) => frontend_contract::domain_types::RouteErrorStatus::Internal,
                };
                let error_type =
                    server_runtime_http::domain_types::HttpErrorType::from(constants_str::ADMIN_API_ERROR_TYPE);
                let optional_diagnostic = match &self {
                    Self::Pg(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::PasswordHash(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::Session(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::Header(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::AuthenticationSecretText(source)
                    | Self::CsrfSecretText(source)
                    | Self::SecretText(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::PasswordText(source) => Some(
                        server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(error_type, source),
                    ),
                    Self::Authentication
                    | Self::Authorization
                    | Self::Conflict
                    | Self::Csrf
                    | Self::MethodNotAllowed
                    | Self::PayloadTooLarge
                    | Self::RateLimited
                    | Self::Validation => None,
                };
                admin_error_response_parts(route_error_status, optional_diagnostic)
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn route_error(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let error = match syn::parse::<syn::Path>(attribute_args) {
        Ok(value) => value,
        Err(parse_error) => return parse_error.to_compile_error().into(),
    };
    if error.segments.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            constants_str::ROUTE_ERROR_REQUIRES_ERROR_TYPE,
        )
        .to_compile_error()
        .into();
    }
    let mut function = match syn::parse::<syn::ItemFn>(input) {
        Ok(value) => value,
        Err(parse_error) => return parse_error.to_compile_error().into(),
    };
    if function.sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            &function.sig,
            constants_str::ROUTE_ERROR_REQUIRES_ASYNC_FUNCTION,
        )
        .to_compile_error()
        .into();
    }
    let return_type = match &function.sig.output {
        syn::ReturnType::Default => {
            return syn::Error::new_spanned(
                &function.sig,
                constants_str::ROUTE_ERROR_REQUIRES_EXPLICIT_RETURN_TYPE,
            )
            .to_compile_error()
            .into();
        }
        syn::ReturnType::Type(_arrow, value) => value.clone(),
    };
    let arguments = match function
        .sig
        .inputs
        .iter()
        .map(|argument| match argument {
            syn::FnArg::Typed(value) => match value.pat.as_ref() {
                syn::Pat::Ident(pattern)
                    if pattern.attrs.is_empty()
                        && pattern.by_ref.is_none()
                        && pattern.mutability.is_none()
                        && pattern.subpat.is_none() =>
                {
                    let identifier = &pattern.ident;
                    Ok(quote::quote! { #identifier })
                }
                syn::Pat::TupleStruct(pattern) if pattern.attrs.is_empty() => {
                    let path = &pattern.path;
                    let elements = pattern
                        .elems
                        .iter()
                        .map(|element| match element {
                            syn::Pat::Ident(inner_pattern)
                                if inner_pattern.attrs.is_empty()
                                    && inner_pattern.by_ref.is_none()
                                    && inner_pattern.mutability.is_none()
                                    && inner_pattern.subpat.is_none() =>
                            {
                                let identifier = &inner_pattern.ident;
                                Ok(quote::quote! { #identifier })
                            }
                            syn::Pat::Const(_)
                            | syn::Pat::Ident(_)
                            | syn::Pat::Lit(_)
                            | syn::Pat::Macro(_)
                            | syn::Pat::Or(_)
                            | syn::Pat::Paren(_)
                            | syn::Pat::Path(_)
                            | syn::Pat::Range(_)
                            | syn::Pat::Reference(_)
                            | syn::Pat::Rest(_)
                            | syn::Pat::Slice(_)
                            | syn::Pat::Struct(_)
                            | syn::Pat::Tuple(_)
                            | syn::Pat::TupleStruct(_)
                            | syn::Pat::Type(_)
                            | syn::Pat::Verbatim(_)
                            | syn::Pat::Wild(_)
                            | _ => Err(syn::Error::new_spanned(
                                element,
                                constants_str::ROUTE_ERROR_UNSUPPORTED_PARAMETER_PATTERN,
                            )),
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    Ok(quote::quote! { #path(#(#elements),*) })
                }
                pattern @ (syn::Pat::Const(_)
                | syn::Pat::Ident(_)
                | syn::Pat::Lit(_)
                | syn::Pat::Macro(_)
                | syn::Pat::Or(_)
                | syn::Pat::Paren(_)
                | syn::Pat::Path(_)
                | syn::Pat::Range(_)
                | syn::Pat::Reference(_)
                | syn::Pat::Rest(_)
                | syn::Pat::Slice(_)
                | syn::Pat::Struct(_)
                | syn::Pat::Tuple(_)
                | syn::Pat::TupleStruct(_)
                | syn::Pat::Type(_)
                | syn::Pat::Verbatim(_)
                | syn::Pat::Wild(_)
                | _) => Err(syn::Error::new_spanned(
                    pattern,
                    constants_str::ROUTE_ERROR_UNSUPPORTED_PARAMETER_PATTERN,
                )),
            },
            syn::FnArg::Receiver(value) => Err(syn::Error::new_spanned(
                value,
                constants_str::ROUTE_ERROR_REQUIRES_TYPED_PARAMETERS,
            )),
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(value) => value,
        Err(parse_error) => return parse_error.to_compile_error().into(),
    };
    let mut inner_signature = function.sig.clone();
    inner_signature.ident = quote::format_ident!("{}_route_impl", function.sig.ident);
    let inner_identifier = &inner_signature.ident;
    let original_block = function.block;
    let unused_async_reason = syn::LitStr::new(
        constants_str::ROUTE_ERROR_UNUSED_ASYNC_REASON,
        proc_macro2::Span::call_site(),
    );
    function.sig.output = syn::parse_quote! {
        -> Result<#return_type, #error>
    };
    function.block = syn::parse_quote! {{
        #[allow(clippy::unused_async, reason = #unused_async_reason)]
        #inner_signature #original_block
        Ok(#inner_identifier(#(#arguments),*).await)
    }};
    let visibility = &function.vis;
    quote::quote! {
        #[derive(Debug, thiserror::Error)]
        #visibility enum #error {}
        impl axum::response::IntoResponse for #error {
            fn into_response(self) -> axum::response::Response {
                match self {}
            }
        }
        #function
    }
    .into()
}

#[proc_macro_attribute]
pub fn route_operation(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if !attribute_args.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            constants_str::ROUTE_OPERATION_ACCEPTS_NO_ARGUMENTS,
        )
        .to_compile_error()
        .into();
    }
    match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(_function) => input,
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro_derive(RouteCatalog, attributes(route_catalog, route_catalog_route))]
pub fn derive_route_catalog(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(catalog_attribute) = derive_input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(constants_str::ROUTE_CATALOG))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::ROUTE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match catalog_attribute.parse_args::<domain_types::RouteCatalogArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let visibility = derive_input.vis.clone();
    let syn::Data::Enum(data_enum) = derive_input.data else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::ROUTE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let variants_capacity = data_enum.variants.len();
    let mut contract_arms = Vec::with_capacity(variants_capacity);
    let mut family_routes = Vec::with_capacity(variants_capacity);
    let mut path_arms = Vec::with_capacity(variants_capacity);
    let mut custom_route_functions = Vec::with_capacity(variants_capacity);
    let snake_case_identifier = |identifier: domain_types::SynIdent| {
        let identifier_value = identifier.as_ref().to_string();
        let value = identifier_value.chars().enumerate().fold(
            String::with_capacity(identifier_value.len().saturating_mul(2usize)),
            |mut value, (index, character)| {
                if character.is_ascii_uppercase() {
                    if index != constants_usize::ZERO {
                        value.push('_');
                    }
                    value.push(character.to_ascii_lowercase());
                } else {
                    value.push(character);
                }
                value
            },
        );
        (identifier, value)
    };
    let all_variant_identifiers = data_enum
        .variants
        .iter()
        .filter(|variant| matches!(variant.fields, syn::Fields::Unit))
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();
    let all_variants_are_unit = all_variant_identifiers.len() == data_enum.variants.len();
    let mut variants = data_enum.variants.into_iter();
    loop {
        let Some(variant) = variants.next() else {
            break;
        };
        let Some(route_attribute) = variant.attrs.iter().find(|attribute| {
            attribute
                .path()
                .is_ident(constants_str::ROUTE_CATALOG_ROUTE)
        }) else {
            return syn::Error::new_spanned(
                variant.ident,
                constants_str::ROUTE_CATALOG_VARIANT_REQUIRES_ROUTE,
            )
            .to_compile_error()
            .into();
        };
        let route_args = match route_attribute.parse_args::<domain_types::RouteCatalogRouteArgs>() {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        };
        let variant_identifier = variant.ident;
        match (route_args.route, variant.fields) {
            (Some(route), syn::Fields::Unit) => {
                let route_type = route.into_inner();
                contract_arms.push(quote::quote! {
                    Self::#variant_identifier => <#route_type as frontend_contract::domain_types::TypedRoute>::metadata().contract()
                });
                path_arms.push(quote::quote! {
                    Self::#variant_identifier => frontend_contract::domain_types::ParameterizedRoutePath::try_from(
                        String::from(frontend_contract::domain_types::typed_route_path::<#route_type>())
                    ).unwrap_or_default()
                });
                family_routes.push(route_type);
            }
            (Some(route), syn::Fields::Unnamed(fields))
                if fields.unnamed.len() == constants_usize::ONE =>
            {
                let route_type = route.into_inner();
                contract_arms.push(quote::quote! {
                    Self::#variant_identifier(_value) => <#route_type as frontend_contract::domain_types::TypedRoute>::metadata().contract()
                });
                path_arms.push(quote::quote! {
                    Self::#variant_identifier(value) => frontend_contract::domain_types::typed_parameterized_route_path::<#route_type>(&value)
                });
                family_routes.push(route_type);
            }
            (Some(_route), syn::Fields::Named(_) | syn::Fields::Unnamed(_)) => {
                return syn::Error::new_spanned(
                    variant_identifier,
                    constants_str::ROUTE_CATALOG_ROUTE_SUPPORTS_UNIT_OR_SINGLE_FIELD_VARIANTS,
                )
                .to_compile_error()
                .into();
            }
            (None, syn::Fields::Unit) => {
                let Some(contract) = route_args.contract else {
                    return syn::Error::new_spanned(
                        variant_identifier,
                        constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                };
                let Some(path) = route_args.path else {
                    return syn::Error::new_spanned(
                        variant_identifier,
                        constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                };
                let contract_expression = contract.into_inner();
                let path_expression = path.into_inner();
                let (wrapped_identifier, variant_name) =
                    snake_case_identifier(domain_types::SynIdent::from(variant_identifier));
                let custom_identifier = wrapped_identifier.into_inner();
                let route_function_identifier =
                    quote::format_ident!("{}_route", variant_name, span = custom_identifier.span());
                let client_function_identifier = quote::format_ident!(
                    "{}_client",
                    variant_name,
                    span = custom_identifier.span()
                );
                custom_route_functions.push(quote::quote! {
                    #[must_use]
                    #visibility fn #route_function_identifier() -> frontend_contract::domain_types::ContractStr {
                        frontend_contract::domain_types::ContractStr::from(#path_expression)
                    }
                    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
                    #visibility async fn #client_function_identifier<Transport>(
                        client: &frontend_contract::domain_types::TypedClient<Transport>,
                    ) -> Result<frontend_contract::domain_types::TransportBody, frontend_contract::domain_types::ClientError>
                    where
                        Transport: frontend_contract::domain_types::Transport,
                    {
                        client.send_contract(#contract_expression, #route_function_identifier()).await
                    }
                });
                contract_arms.push(quote::quote! {
                    Self::#custom_identifier => #contract_expression
                });
                path_arms.push(quote::quote! {
                    Self::#custom_identifier => frontend_contract::domain_types::ParameterizedRoutePath::try_from(
                        String::from(#path_expression)
                    ).unwrap_or_default()
                });
                if !route_args.exclude_from_family.get() {
                    return syn::Error::new_spanned(
                        custom_identifier,
                        constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                }
            }
            (None, syn::Fields::Named(_) | syn::Fields::Unnamed(_)) => {
                return syn::Error::new_spanned(
                    variant_identifier,
                    constants_str::ROUTE_CATALOG_CUSTOM_ROUTE_MUST_BE_UNIT,
                )
                .to_compile_error()
                .into();
            }
        }
    }
    let identifier = derive_input.ident;
    let family = args.family.into_inner();
    let body_limit = args.body_limit.into_inner();
    let route_count = family_routes.len();
    let all = if all_variants_are_unit {
        let variant_count = all_variant_identifiers.len();
        quote::quote! {
            pub const ALL: [Self; #variant_count] = [
                #(Self::#all_variant_identifiers),*
            ];
        }
    } else {
        quote::quote! {}
    };
    quote::quote! {
        #(#custom_route_functions)*
        impl #identifier {
            #all

            #[must_use]
            pub fn contract(self) -> frontend_contract::domain_types::RouteContract {
                match self {
                    #(#contract_arms),*
                }
            }
            fn catalog_path(self) -> frontend_contract::domain_types::ParameterizedRoutePath {
                match self {
                    #(#path_arms),*
                }
            }
        }
        #[derive(Clone, Copy, Debug)]
        pub struct #family;
        impl frontend_contract::domain_types::RouteFamily for #family {
            const ROUTE_COUNT: usize = #route_count;
            fn body_limit() -> Option<frontend_contract::domain_types::RouteBodyLimit> {
                Some(frontend_contract::domain_types::RouteBodyLimit::from(#body_limit))
            }
            fn coverage_descriptors() -> frontend_contract::domain_types::RouteCoverageDescriptors {
                frontend_contract::domain_types::RouteCoverageDescriptors::from_max_iter(vec![
                    #(
                        <#family_routes as frontend_contract::domain_types::CoveredRoute>::coverage_descriptor()
                    ),*
                ])
            }
            fn schema_contracts() -> frontend_contract::domain_types::RouteSchemaContracts {
                frontend_contract::domain_types::RouteSchemaContracts::from_max_iter(vec![
                    #(
                        frontend_contract::domain_types::RouteSchemaContract::from_typed_route::<#family_routes>()
                    ),*
                ])
            }
        }
        #(impl frontend_contract::domain_types::RouteInFamily<#family> for #family_routes {})*
    }
    .into()
}

#[proc_macro_derive(UnitEnumCatalog)]
pub fn derive_unit_enum_catalog(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = derive_input.ident.clone();
    let syn::Data::Enum(data_enum) = derive_input.data else {
        return syn::Error::new_spanned(identifier, constants_str::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS)
            .to_compile_error()
            .into();
    };
    let identifiers_result = data_enum
        .variants
        .iter()
        .map(|variant| {
            if matches!(variant.fields, syn::Fields::Unit) {
                Ok(&variant.ident)
            } else {
                Err(syn::Error::new_spanned(
                    &variant.ident,
                    constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS,
                ))
            }
        })
        .collect::<Result<Vec<_>, _>>();
    let identifiers = match identifiers_result {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let count = identifiers.len();
    quote::quote! {
        impl #identifier {
            pub const ALL: [Self; #count] = [
                #(Self::#identifiers),*
            ];
        }
    }
    .into()
}

#[proc_macro_derive(UnitEnumIndex)]
pub fn derive_unit_enum_index(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = derive_input.ident.clone();
    let syn::Data::Enum(data_enum) = derive_input.data else {
        return syn::Error::new_spanned(identifier, constants_str::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS)
            .to_compile_error()
            .into();
    };
    let identifiers = match data_enum
        .variants
        .iter()
        .map(|variant| {
            if matches!(variant.fields, syn::Fields::Unit) {
                Ok(&variant.ident)
            } else {
                Err(syn::Error::new_spanned(
                    &variant.ident,
                    constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS,
                ))
            }
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let indices = constants_usize::ZERO..identifiers.len();
    let count = identifiers.len();
    quote::quote! {
        impl #identifier {
            pub const COUNT: usize = #count;
            #[must_use]
            pub const fn index(self) -> usize {
                match self {
                    #(Self::#identifiers => #indices),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(PageCatalog, attributes(page_catalog, page_catalog_page))]
pub fn derive_page_catalog(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(attribute) = input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(constants_str::PAGE_CATALOG))
    else {
        return syn::Error::new_spanned(
            input.ident,
            constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match attribute.parse_args::<domain_types::PageCatalogArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            constants_str::PAGE_CATALOG_SUPPORTS_UNIT_VARIANTS,
        )
        .to_compile_error()
        .into();
    };
    let pages = match data_enum
        .variants
        .iter()
        .map(|variant| {
            if !matches!(variant.fields, syn::Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    constants_str::PAGE_CATALOG_SUPPORTS_UNIT_VARIANTS,
                ));
            }
            let page_attribute = variant
                .attrs
                .iter()
                .find(|candidate| candidate.path().is_ident(constants_str::PAGE_CATALOG_PAGE))
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        &variant.ident,
                        constants_str::PAGE_CATALOG_VARIANT_REQUIRES_PAGE,
                    )
                })?;
            page_attribute
                .parse_args::<domain_types::PageCatalogPageArgs>()
                .map(|page| (&variant.ident, page))
        })
        .collect::<syn::Result<Vec<_>>>()
    {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifiers = pages
        .iter()
        .map(|(identifier, _page)| *identifier)
        .collect::<Vec<_>>();
    let capabilities = pages
        .iter()
        .map(|(_identifier, page)| page.capability.as_ref())
        .collect::<Vec<_>>();
    let paths = pages
        .iter()
        .map(|(_identifier, page)| page.path.as_ref())
        .collect::<Vec<_>>();
    let metadata = pages
        .iter()
        .map(|(_identifier, page)| page.metadata.as_ref())
        .collect::<Vec<_>>();
    let routes = pages
        .iter()
        .map(|(_identifier, page)| page.route.as_ref())
        .collect::<Vec<_>>();
    let titles = pages
        .iter()
        .map(|(_identifier, page)| page.title.as_ref())
        .collect::<Vec<_>>();
    let identifier = &input.ident;
    let inventory = args.inventory.into_inner();
    let path_ref = args.path_ref.into_inner();
    let spec = args.spec.into_inner();
    let indexes = (0..pages.len()).map(syn::Index::from);
    let count = pages.len();
    quote::quote! {
        const #inventory: [#spec; #count] = [
            #(
                #spec::new(
                    #capabilities,
                    #metadata,
                    #identifier::#identifiers,
                    #paths,
                    #routes,
                    #titles,
                )
            ),*
        ];
        impl #identifier {
            pub fn all() -> impl Iterator<Item = Self> {
                #inventory.iter().map(|spec| spec.page())
            }
            #[must_use]
            pub const fn specs() -> &'static [#spec] {
                &#inventory
            }
            #[must_use]
            pub fn from_path(path: #path_ref<'_>) -> Option<Self> {
                #inventory
                    .iter()
                    .find(|spec| spec.path().as_ref() == path.0)
                    .map(|spec| spec.page())
            }
            #[must_use]
            pub const fn spec(self) -> #spec {
                match self {
                    #(Self::#identifiers => #inventory[#indexes]),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(RouteFamily, attributes(route_family, route_family_body_limit))]
pub fn derive_route_family(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = match syn::parse::<syn::DeriveInput>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(route_family_attribute) = derive_input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(constants_str::ROUTE_FAMILY))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            constants_str::ROUTE_FAMILY_DERIVE_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let routes = match route_family_attribute
        .parse_args_with(syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated)
    {
        Ok(value) if !value.is_empty() => value,
        Ok(_) => {
            return syn::Error::new_spanned(
                route_family_attribute,
                constants_str::ROUTE_FAMILY_REQUIRES_ROUTE,
            )
            .to_compile_error()
            .into();
        }
        Err(error) => return error.to_compile_error().into(),
    };
    let body_limit = match derive_input
        .attrs
        .iter()
        .find(|attribute| {
            attribute
                .path()
                .is_ident(constants_str::ROUTE_FAMILY_BODY_LIMIT)
        })
        .map(syn::Attribute::parse_args::<syn::Expr>)
        .transpose()
    {
        Ok(Some(value)) => quote::quote! {
            fn body_limit() -> Option<frontend_contract::domain_types::RouteBodyLimit> {
                Some(frontend_contract::domain_types::RouteBodyLimit::from(#value))
            }
        },
        Ok(None) => quote::quote! {},
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = derive_input.ident;
    let route_types = routes.iter().collect::<Vec<_>>();
    let route_count = route_types.len();
    quote::quote! {
        impl frontend_contract::domain_types::RouteFamily for #identifier {
            const ROUTE_COUNT: usize = #route_count;
            #body_limit
            fn coverage_descriptors() -> frontend_contract::domain_types::RouteCoverageDescriptors {
                frontend_contract::domain_types::RouteCoverageDescriptors::from_max_iter(vec![
                    #(
                        <#route_types as frontend_contract::domain_types::CoveredRoute>::coverage_descriptor()
                    ),*
                ])
            }
            fn schema_contracts() -> frontend_contract::domain_types::RouteSchemaContracts {
                frontend_contract::domain_types::RouteSchemaContracts::from_max_iter(vec![
                    #(
                        frontend_contract::domain_types::RouteSchemaContract::from_typed_route::<#route_types>()
                    ),*
                ])
            }
        }
        #(impl frontend_contract::domain_types::RouteInFamily<#identifier> for #route_types {})*
    }
    .into()
}

#[cfg(test)]
mod tests;
