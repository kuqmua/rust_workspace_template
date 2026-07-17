#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "proc-macro parser models precede their entrypoints while related derive parsers remain adjacent"
)]
struct SynExpr(syn::Expr);
impl From<syn::Expr> for SynExpr {
    fn from(value: syn::Expr) -> Self {
        Self(value)
    }
}
struct SynType(syn::Type);
impl From<syn::Type> for SynType {
    fn from(value: syn::Type) -> Self {
        Self(value)
    }
}
struct TypedRouteArgs {
    access: Option<SynExpr>,
    method: SynExpr,
    mutation: Option<SynExpr>,
    obligations: Option<SynExpr>,
    openapi_operation_id: SynExpr,
    path: SynExpr,
    request: SynType,
    response: SynType,
    transport: SynType,
}

struct RouteRegistryBinding {
    handler: SynRouteRegistryHandler,
    route: SynRouteRegistryRoute,
}
struct SynRouteRegistryHandler(syn::Path);
struct SynRouteRegistryRoute(syn::Type);
struct SynRouteRegistryBindings(syn::punctuated::Punctuated<RouteRegistryBinding, syn::Token![,]>);
struct SynRouteRegistryState(syn::Type);
impl syn::parse::Parse for RouteRegistryBinding {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in input);
        let route = SynRouteRegistryRoute(content.parse::<syn::Type>()?);
        let _comma = content.parse::<syn::Token![,]>()?;
        let handler = SynRouteRegistryHandler(content.parse::<syn::Path>()?);
        Ok(Self { handler, route })
    }
}
struct RouteRegistryArgs {
    bindings: SynRouteRegistryBindings,
    state: SynRouteRegistryState,
}

#[proc_macro_attribute]
pub fn route_openapi(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function = match syn::parse::<syn::ItemFn>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let metadata = proc_macro2::TokenStream::from(attribute_args);
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
impl syn::parse::Parse for RouteRegistryArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = input.parse::<syn::Ident>()?;
        if state_name != str_constants::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                str_constants::ROUTE_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = input.parse::<syn::Token![=]>()?;
        let state = SynRouteRegistryState(input.parse::<syn::Type>()?);
        let _semicolon = input.parse::<syn::Token![;]>()?;
        let bindings =
            syn::punctuated::Punctuated::<RouteRegistryBinding, syn::Token![,]>::parse_terminated(
                input,
            )?;
        if bindings.is_empty() {
            return Err(input.error(str_constants::ROUTE_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self {
            bindings: SynRouteRegistryBindings(bindings),
            state,
        })
    }
}

#[proc_macro_attribute]
pub fn route_registry(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_args = match syn::parse::<RouteRegistryArgs>(attribute_args) {
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
        .position(|attribute| attribute.path().is_ident(str_constants::OPENAPI))
    else {
        return syn::Error::new_spanned(
            item.ident,
            str_constants::ROUTE_REGISTRY_REQUIRES_OPENAPI_ATTRIBUTE,
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
                str_constants::ROUTE_REGISTRY_REQUIRES_OPENAPI_ATTRIBUTE,
            )
            .to_compile_error()
            .into();
        }
    };
    let identifier = &item.ident;
    let openapi_identifier = quote::format_ident!("{}OpenApi", identifier);
    let state = parsed_args.state.0;
    let routes = parsed_args
        .bindings
        .0
        .iter()
        .map(|binding| &binding.route.0)
        .collect::<Vec<_>>();
    let handlers = parsed_args
        .bindings
        .0
        .iter()
        .map(|binding| &binding.handler.0)
        .collect::<Vec<_>>();
    let openapi_paths = parsed_args
        .bindings
        .0
        .iter()
        .map(|binding| {
            let mut path = binding.handler.0.clone();
            if let Some(last_segment) = path.segments.last_mut() {
                last_segment.ident = quote::format_ident!("__path_{}", last_segment.ident);
            }
            path
        })
        .collect::<Vec<_>>();
    quote::quote! {
        #item
        #[allow(clippy::needless_for_each)]
        #[derive(utoipa::OpenApi)]
        #[openapi(paths(#(#handlers),*), #openapi_metadata)]
        struct #openapi_identifier;
        impl #identifier {
            fn open_api() -> utoipa::openapi::OpenApi {
                let mut document = <#openapi_identifier as utoipa::OpenApi>::openapi();
                document.paths = utoipa::openapi::path::Paths::new();
                #({
                    let metadata = <#routes as frontend_contract::TypedRoute>::metadata();
                    let mut source_path_item = <#openapi_paths as utoipa::Path>::path_item(None);
                    if let Some(mut operation) = source_path_item
                        .operations
                        .remove(&utoipa::openapi::path::PathItemType::Get)
                    {
                        operation.operation_id = Some(metadata.openapi_operation_id().as_ref().to_owned());
                        let path_item_type = match metadata.route_method() {
                            frontend_contract::RouteMethod::Connect => utoipa::openapi::path::PathItemType::Connect,
                            frontend_contract::RouteMethod::Delete => utoipa::openapi::path::PathItemType::Delete,
                            frontend_contract::RouteMethod::Get => utoipa::openapi::path::PathItemType::Get,
                            frontend_contract::RouteMethod::Head => utoipa::openapi::path::PathItemType::Head,
                            frontend_contract::RouteMethod::Options => utoipa::openapi::path::PathItemType::Options,
                            frontend_contract::RouteMethod::Patch => utoipa::openapi::path::PathItemType::Patch,
                            frontend_contract::RouteMethod::Post => utoipa::openapi::path::PathItemType::Post,
                            frontend_contract::RouteMethod::Put => utoipa::openapi::path::PathItemType::Put,
                            frontend_contract::RouteMethod::Trace => utoipa::openapi::path::PathItemType::Trace,
                        };
                        let mut path_item = utoipa::openapi::path::PathItem::new(path_item_type, operation);
                        document
                            .paths
                            .paths
                            .entry(metadata.path().as_ref().to_owned())
                            .and_modify(|existing| existing.operations.extend(path_item.operations.clone()))
                            .or_insert(path_item);
                    }
                })*
                document
            }
            fn router() -> axum::Router<#state> {
                axum::Router::new()
                    #(.route(
                        frontend_contract::typed_route_path::<#routes>().as_ref(),
                        axum::routing::on(
                            match <#routes as frontend_contract::TypedRoute>::metadata().route_method() {
                                frontend_contract::RouteMethod::Connect => axum::routing::MethodFilter::CONNECT,
                                frontend_contract::RouteMethod::Delete => axum::routing::MethodFilter::DELETE,
                                frontend_contract::RouteMethod::Get => axum::routing::MethodFilter::GET,
                                frontend_contract::RouteMethod::Head => axum::routing::MethodFilter::HEAD,
                                frontend_contract::RouteMethod::Options => axum::routing::MethodFilter::OPTIONS,
                                frontend_contract::RouteMethod::Patch => axum::routing::MethodFilter::PATCH,
                                frontend_contract::RouteMethod::Post => axum::routing::MethodFilter::POST,
                                frontend_contract::RouteMethod::Put => axum::routing::MethodFilter::PUT,
                                frontend_contract::RouteMethod::Trace => axum::routing::MethodFilter::TRACE,
                            },
                            #handlers,
                        ),
                    ))*
            }
        }
    }
    .into()
}

impl syn::parse::Parse for TypedRouteArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut method = None;
        let mut access = None;
        let mut mutation = None;
        let mut obligations = None;
        let mut openapi_operation_id = None;
        let mut path = None;
        let mut request = None;
        let mut response = None;
        let mut transport = None;
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            let _equals: syn::Token![=] = input.parse()?;
            match name.to_string().as_str() {
                str_constants::ACCESS => {
                    access = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::METHOD => {
                    method = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::OPENAPI_OPERATION_ID => {
                    openapi_operation_id = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::MUTATION => {
                    mutation = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::OBLIGATIONS => {
                    obligations = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_PATH => {
                    path = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::REQUEST => {
                    request = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::RESPONSE => {
                    response = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::TRANSPORT => {
                    transport = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        str_constants::UNSUPPORTED_TYPED_ROUTE_FIELD,
                    ));
                }
            }
            if !input.is_empty() {
                let _comma: syn::Token![,] = input.parse()?;
            }
        }
        Ok(Self {
            access,
            method: method
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_METHOD))?,
            mutation,
            obligations,
            openapi_operation_id: openapi_operation_id
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_OPERATION_ID))?,
            path: path.ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_PATH))?,
            request: request
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_REQUEST))?,
            response: response
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_RESPONSE))?,
            transport: transport
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_TRANSPORT))?,
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
        .find(|attribute| attribute.path().is_ident(str_constants::TYPED_ROUTE))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            str_constants::TYPED_ROUTE_DERIVE_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match attribute.parse_args::<TypedRouteArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = derive_input.ident;
    let method = match args.method.0 {
        syn::Expr::Path(path_expression) => {
            match path_expression
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
            {
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::CONNECT) => {
                    quote::quote!(frontend_contract::RouteMethod::Connect)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::DELETE) => {
                    quote::quote!(frontend_contract::RouteMethod::Delete)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::GET) => {
                    quote::quote!(frontend_contract::RouteMethod::Get)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::HEAD) => {
                    quote::quote!(frontend_contract::RouteMethod::Head)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::OPTIONS) => {
                    quote::quote!(frontend_contract::RouteMethod::Options)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::PATCH) => {
                    quote::quote!(frontend_contract::RouteMethod::Patch)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::POST) => {
                    quote::quote!(frontend_contract::RouteMethod::Post)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::PUT) => {
                    quote::quote!(frontend_contract::RouteMethod::Put)
                }
                Some(method_name) if method_name.eq_ignore_ascii_case(str_constants::TRACE) => {
                    quote::quote!(frontend_contract::RouteMethod::Trace)
                }
                _ => {
                    return syn::Error::new_spanned(
                        path_expression,
                        str_constants::TYPED_ROUTE_METHOD_MUST_BE_STANDARD_HTTP_METHOD,
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        value => {
            return syn::Error::new_spanned(
                value,
                str_constants::TYPED_ROUTE_METHOD_MUST_BE_STANDARD_HTTP_METHOD,
            )
            .to_compile_error()
            .into();
        }
    };
    let access = args.access.map_or_else(
        || quote::quote!(frontend_contract::RouteAccess::Public),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let mutation = args.mutation.map_or_else(
        || quote::quote!(frontend_contract::RouteMutation::ReadOnly),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let obligations = args.obligations.map_or_else(
        || quote::quote!(&[]),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let openapi_operation_id = args.openapi_operation_id.0;
    let path = args.path.0;
    let request = args.request.0;
    let response = args.response.0;
    let transport = args.transport.0;
    quote::quote! {
        impl frontend_contract::TypedRoute for #identifier {
            type Request = #request;
            type Response = #response;
            type Transport = #transport;
            fn metadata() -> frontend_contract::RouteMetadata {
                frontend_contract::RouteMetadata::new(
                    #method,
                    frontend_contract::ContractStr::from(#openapi_operation_id),
                    frontend_contract::ContractStr::from(#path),
                )
            }
        }
        impl frontend_contract::CoveredRoute for #identifier {
            fn coverage_descriptor() -> frontend_contract::RouteCoverageDescriptor {
                frontend_contract::RouteCoverageDescriptor::new(
                    <Self as frontend_contract::TypedRoute>::metadata(),
                    #access,
                    #mutation,
                    frontend_contract::RouteCoverageEvidence::new(#obligations),
                )
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
        .find(|attribute| attribute.path().is_ident(str_constants::ROUTE_FAMILY))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            str_constants::ROUTE_FAMILY_DERIVE_REQUIRES_ATTRIBUTE,
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
                str_constants::ROUTE_FAMILY_REQUIRES_ROUTE,
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
                .is_ident(str_constants::ROUTE_FAMILY_BODY_LIMIT)
        })
        .map(syn::Attribute::parse_args::<syn::Expr>)
        .transpose()
    {
        Ok(Some(value)) => quote::quote! {
            fn body_limit() -> Option<frontend_contract::RouteBodyLimit> {
                Some(frontend_contract::RouteBodyLimit::from(#value))
            }
        },
        Ok(None) => quote::quote! {},
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = derive_input.ident;
    let route_types = routes.iter().collect::<Vec<_>>();
    quote::quote! {
        impl frontend_contract::RouteFamily for #identifier {
            #body_limit
            fn coverage_descriptors() -> Vec<frontend_contract::RouteCoverageDescriptor> {
                vec![
                    #(
                        <#route_types as frontend_contract::CoveredRoute>::coverage_descriptor()
                    ),*
                ]
            }
        }
    }
    .into()
}
