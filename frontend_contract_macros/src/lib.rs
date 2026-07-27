#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "proc-macro parser models precede their entrypoints while related derive parsers remain adjacent"
)]
#[derive(newtype::FromInner)]
struct SynExpr(syn::Expr);

#[derive(newtype::FromInner)]
struct SynType(syn::Type);

#[derive(newtype::FromInner)]
struct SynIdent(syn::Ident);

#[derive(newtype::FromInner)]
struct StdBool(bool);

struct RouteCatalogArgs {
    body_limit: SynExpr,
    family: SynIdent,
}
struct RouteCatalogRouteArgs {
    contract: Option<SynExpr>,
    exclude_from_family: StdBool,
    path: Option<SynExpr>,
    route: Option<SynType>,
}
struct PageCatalogArgs {
    inventory: SynIdent,
    path_ref: SynIdent,
    spec: SynIdent,
}
struct PageCatalogPageArgs {
    capability: SynExpr,
    metadata: SynExpr,
    path: SynExpr,
    route: SynExpr,
    title: SynExpr,
}
impl syn::parse::Parse for PageCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == str_constants::PAGE_CATALOG_INVENTORY {
                inventory = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == str_constants::PAGE_CATALOG_PATH_REF {
                path_ref = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == str_constants::PAGE_CATALOG_SPEC {
                spec = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    str_constants::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            path_ref: path_ref
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            spec: spec
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
        })
    }
}
impl syn::parse::Parse for PageCatalogPageArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut capability = None;
        let mut metadata = None;
        let mut path = None;
        let mut route = None;
        let mut title = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == str_constants::PAGE_CATALOG_CAPABILITY {
                capability = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == str_constants::PAGE_CATALOG_METADATA {
                metadata = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == str_constants::ROUTE_CATALOG_PATH {
                path = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == str_constants::PAGE_CATALOG_ROUTE {
                route = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else if name == str_constants::PAGE_CATALOG_TITLE {
                title = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            capability: capability
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            metadata: metadata
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            path: path
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            route: route
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
            title: title
                .ok_or_else(|| input.error(str_constants::PAGE_CATALOG_PAGE_REQUIRES_FIELDS))?,
        })
    }
}
impl syn::parse::Parse for RouteCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut body_limit = None;
        let mut family = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == str_constants::ROUTE_CATALOG_FAMILY {
                family = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == str_constants::ROUTE_CATALOG_BODY_LIMIT {
                body_limit = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    str_constants::UNSUPPORTED_TYPED_ROUTE_FIELD,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            body_limit: body_limit
                .ok_or_else(|| input.error(str_constants::ROUTE_CATALOG_REQUIRES_BODY_LIMIT))?,
            family: family
                .ok_or_else(|| input.error(str_constants::ROUTE_CATALOG_REQUIRES_FAMILY))?,
        })
    }
}
impl syn::parse::Parse for RouteCatalogRouteArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(syn::Ident) && input.peek2(syn::Token![=]) {
            let mut contract = None;
            let mut exclude_from_family = StdBool::from(false);
            let mut path = None;
            while !input.is_empty() {
                let name = input.parse::<syn::Ident>()?;
                if name == str_constants::ROUTE_CATALOG_EXCLUDE_FROM_FAMILY {
                    exclude_from_family = StdBool::from(true);
                } else {
                    let _equals = input.parse::<syn::Token![=]>()?;
                    if name == str_constants::ROUTE_CATALOG_CONTRACT {
                        contract = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                    } else if name == str_constants::ROUTE_CATALOG_PATH {
                        path = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                    } else {
                        return Err(syn::Error::new_spanned(
                            name,
                            str_constants::UNSUPPORTED_TYPED_ROUTE_FIELD,
                        ));
                    }
                }
                if !input.is_empty() {
                    let _comma = input.parse::<syn::Token![,]>()?;
                }
            }
            if contract.is_none() || path.is_none() {
                return Err(
                    input.error(str_constants::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES)
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
                exclude_from_family: StdBool::from(false),
                path: None,
                route: Some(SynType::from(input.parse::<syn::Type>()?)),
            })
        }
    }
}
struct TypedRouteArgs {
    authentication: SynExpr,
    error_response: Option<SynType>,
    errors: SynTypedRouteErrors,
    method: SynExpr,
    mutation: Option<SynExpr>,
    obligations: Option<SynExpr>,
    openapi_operation_id: SynExpr,
    path: SynExpr,
    path_parameter: Option<SynType>,
    request: SynType,
    request_body: Option<SynExpr>,
    response: SynType,
    success_status: SynExpr,
    transport: SynType,
}
enum SynTypedRouteErrors {
    Policy(SynExpr),
    Statuses(SynExpr),
}

struct RouteRegistryBinding {
    handler: SynRouteRegistryHandler,
    route: SynRouteRegistryRoute,
}
#[derive(newtype::FromInner)]
struct SynRouteRegistryHandler(syn::Path);

#[derive(newtype::FromInner)]
struct SynRouteRegistryRoute(syn::Type);

#[derive(newtype::FromInner)]
struct SynRouteRegistryBindings(syn::punctuated::Punctuated<RouteRegistryBinding, syn::Token![,]>);

#[derive(newtype::FromInner)]
struct SynRouteRegistrySchemas(Vec<syn::Type>);

#[derive(newtype::FromInner)]
struct SynRouteRegistryState(syn::Type);

#[derive(newtype::FromInner)]
struct SynRouteRegistryFamily(syn::Type);

struct HandlerRegistryBinding {
    contract: SynHandlerRegistryContract,
    handler: SynHandlerRegistryHandler,
}
#[derive(newtype::FromInner)]
struct SynHandlerRegistryContract(syn::Expr);
#[derive(newtype::FromInner)]
struct SynHandlerRegistryHandler(syn::Path);

#[derive(newtype::FromInner)]
struct SynHandlerRegistryBindings(
    syn::punctuated::Punctuated<HandlerRegistryBinding, syn::Token![,]>,
);

#[derive(newtype::FromInner)]
struct SynHandlerRegistryState(syn::Type);

impl syn::parse::Parse for HandlerRegistryBinding {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in input);
        let contract = SynHandlerRegistryContract::from(content.parse::<syn::Expr>()?);
        let _contract_comma = content.parse::<syn::Token![,]>()?;
        let handler = SynHandlerRegistryHandler::from(content.parse::<syn::Path>()?);
        Ok(Self { contract, handler })
    }
}

struct HandlerRegistryArgs {
    bindings: SynHandlerRegistryBindings,
    state: SynHandlerRegistryState,
}

impl syn::parse::Parse for HandlerRegistryArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = input.parse::<syn::Ident>()?;
        if state_name != str_constants::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                str_constants::HANDLER_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = input.parse::<syn::Token![=]>()?;
        let state = SynHandlerRegistryState::from(input.parse::<syn::Type>()?);
        let _semicolon = input.parse::<syn::Token![;]>()?;
        let bindings =
            syn::punctuated::Punctuated::<HandlerRegistryBinding, syn::Token![,]>::parse_terminated(
                input,
            )?;
        if bindings.is_empty() {
            return Err(input.error(str_constants::HANDLER_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self {
            bindings: SynHandlerRegistryBindings::from(bindings),
            state,
        })
    }
}

impl syn::parse::Parse for RouteRegistryBinding {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let content;
        let _parenthesis = syn::parenthesized!(content in input);
        let route = SynRouteRegistryRoute::from(content.parse::<syn::Type>()?);
        let _comma = content.parse::<syn::Token![,]>()?;
        let handler = SynRouteRegistryHandler::from(content.parse::<syn::Path>()?);
        Ok(Self { handler, route })
    }
}
struct RouteRegistryArgs {
    authenticated_security: SynExpr,
    bindings: SynRouteRegistryBindings,
    csrf_security: SynExpr,
    family: SynRouteRegistryFamily,
    schemas: SynRouteRegistrySchemas,
    state: SynRouteRegistryState,
}

#[proc_macro_attribute]
pub fn handler_registry(
    attribute_args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_args = match syn::parse::<HandlerRegistryArgs>(attribute_args) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let item = match syn::parse::<syn::ItemStruct>(input) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifier = &item.ident;
    let state = parsed_args.state.0;
    let contracts = parsed_args
        .bindings
        .0
        .iter()
        .map(|binding| &binding.contract.0)
        .collect::<Vec<_>>();
    let handlers = parsed_args
        .bindings
        .0
        .iter()
        .map(|binding| &binding.handler.0)
        .collect::<Vec<_>>();
    quote::quote! {
        #item
        impl #identifier {
            fn router() -> axum::Router<#state> {
                axum::Router::new()
                    #(.route(
                        frontend_contract::HandlerContract::path(#contracts).get(),
                        frontend_contract::handler_method_router(
                            frontend_contract::HandlerContract::method(#contracts),
                            #handlers,
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
        let state = SynRouteRegistryState::from(input.parse::<syn::Type>()?);
        let _state_comma = input.parse::<syn::Token![,]>()?;
        let family_name = input.parse::<syn::Ident>()?;
        if family_name != str_constants::FAMILY {
            return Err(syn::Error::new_spanned(
                family_name,
                str_constants::ROUTE_REGISTRY_REQUIRES_FAMILY,
            ));
        }
        let _family_equals = input.parse::<syn::Token![=]>()?;
        let family = SynRouteRegistryFamily::from(input.parse::<syn::Type>()?);
        let _family_semicolon = input.parse::<syn::Token![;]>()?;
        let security_content;
        let _security_parenthesis = syn::parenthesized!(security_content in input);
        let authenticated_security = SynExpr::from(security_content.parse::<syn::Expr>()?);
        let _comma = security_content.parse::<syn::Token![,]>()?;
        let csrf_security = SynExpr::from(security_content.parse::<syn::Expr>()?);
        let _security_semicolon = input.parse::<syn::Token![;]>()?;
        let schemas_name = input.parse::<syn::Ident>()?;
        if schemas_name != str_constants::SCHEMAS {
            return Err(syn::Error::new_spanned(
                schemas_name,
                str_constants::ROUTE_REGISTRY_REQUIRES_SCHEMAS,
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
        let bindings =
            syn::punctuated::Punctuated::<RouteRegistryBinding, syn::Token![,]>::parse_terminated(
                input,
            )?;
        if bindings.is_empty() {
            return Err(input.error(str_constants::ROUTE_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self {
            authenticated_security,
            bindings: SynRouteRegistryBindings::from(bindings),
            csrf_security,
            family,
            schemas: SynRouteRegistrySchemas::from(schemas),
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
    let unique_route_trait_identifier = quote::format_ident!("{}UniqueRoute", identifier);
    let openapi_identifier = quote::format_ident!("{}OpenApi", identifier);
    let state = parsed_args.state.0;
    let family = parsed_args.family.0;
    let authenticated_security = parsed_args.authenticated_security.0;
    let csrf_security = parsed_args.csrf_security.0;
    let schemas = parsed_args.schemas.0;
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
    let route_count = routes.len();
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
        trait #unique_route_trait_identifier {}
        #(impl #unique_route_trait_identifier for #routes {})*
        const _: [(); <#family as frontend_contract::RouteFamily>::ROUTE_COUNT] =
            [(); #route_count];
        #[allow(clippy::needless_for_each)]
        #[derive(utoipa::OpenApi)]
        #[openapi(paths(#(#handlers),*), #openapi_metadata)]
        struct #openapi_identifier;
        impl #identifier {
            fn assert_route_family_membership<Route>()
            where
                Route: frontend_contract::RouteInFamily<#family> + #unique_route_trait_identifier,
            {
            }
            fn open_api() -> utoipa::openapi::OpenApi {
                let mut document = <#openapi_identifier as utoipa::OpenApi>::openapi();
                #({
                    let components = document
                        .components
                        .get_or_insert_with(utoipa::openapi::schema::Components::new);
                    let mut schema_components =
                        frontend_contract::UtoipaOpenApiComponentsRefMut::from(components);
                    frontend_contract::register_openapi_schema::<#schemas>(
                        &mut schema_components
                    );
                })*
                document.paths = utoipa::openapi::path::Paths::new();
                #({
                    let mut open_api = frontend_contract::UtoipaOpenApiRefMut::from(&mut document);
                    frontend_contract::register_openapi_route_schemas::<#routes>(
                        &mut open_api
                    );
                    let metadata = <#routes as frontend_contract::TypedRoute>::metadata();
                    let mut source_path_item = <#openapi_paths as utoipa::Path>::path_item(None);
                    if let Some(mut operation) = source_path_item
                        .operations
                        .remove(&utoipa::openapi::path::PathItemType::Get)
                    {
                        operation.operation_id = Some(metadata.openapi_operation_id().as_ref().to_owned());
                        frontend_contract::apply_openapi_request_contract::<#routes>(&mut operation);
                        frontend_contract::apply_openapi_success_contract::<#routes>(&mut operation);
                        frontend_contract::apply_openapi_error_contract::<#routes>(&mut operation);
                        frontend_contract::apply_openapi_path_parameter_contract::<#routes>(&mut operation);
                        frontend_contract::apply_openapi_security_contract::<#routes>(
                            &mut operation,
                            frontend_contract::OpenApiSecuritySchemeRef::from(#authenticated_security),
                            frontend_contract::OpenApiSecuritySchemeRef::from(#csrf_security),
                        );
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
                #(Self::assert_route_family_membership::<#routes>();)*
                axum::Router::new()
                    #(.route(
                        frontend_contract::typed_route_path::<#routes>().as_ref(),
                        axum::routing::on(
                            axum::routing::MethodFilter::from(
                                frontend_contract::axum_method_filter(
                                    <#routes as frontend_contract::TypedRoute>::metadata()
                                        .contract()
                                        .method()
                                )
                            ),
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
                str_constants::TYPED_ROUTE_FIELD_AUTHENTICATION => {
                    authentication = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_ERROR_STATUSES => {
                    error_statuses = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_ERROR_RESPONSE => {
                    error_response = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_ERROR_POLICY => {
                    error_policy = Some(SynExpr::from(input.parse::<syn::Expr>()?));
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
                str_constants::TYPED_ROUTE_FIELD_PATH_PARAMETER => {
                    path_parameter = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::REQUEST => {
                    request = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_REQUEST_BODY => {
                    request_body = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::RESPONSE => {
                    response = Some(SynType::from(input.parse::<syn::Type>()?));
                }
                str_constants::TYPED_ROUTE_FIELD_SUCCESS_STATUS => {
                    success_status = Some(SynExpr::from(input.parse::<syn::Expr>()?));
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
        let errors = match (error_policy, error_statuses) {
            (Some(policy), None) => SynTypedRouteErrors::Policy(policy),
            (None, Some(statuses)) => SynTypedRouteErrors::Statuses(statuses),
            (None, None) | (Some(_), Some(_)) => {
                return Err(
                    input.error(str_constants::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES)
                );
            }
        };
        Ok(Self {
            authentication: authentication
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_AUTHENTICATION))?,
            error_response,
            errors,
            method: method
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_METHOD))?,
            mutation,
            obligations,
            openapi_operation_id: openapi_operation_id
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_OPERATION_ID))?,
            path: path.ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_PATH))?,
            path_parameter,
            request: request
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_REQUEST))?,
            request_body,
            response: response
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_RESPONSE))?,
            success_status: success_status
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_SUCCESS_STATUS))?,
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
    let authentication = args.authentication.0;
    let mutation = args.mutation.map_or_else(
        || quote::quote!(frontend_contract::RouteMutation::ReadOnly),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let error_statuses = match args.errors {
        SynTypedRouteErrors::Policy(value) => {
            let policy = value.0;
            quote::quote!((#policy).statuses(#authentication, #mutation))
        }
        SynTypedRouteErrors::Statuses(value) => quote::ToTokens::into_token_stream(&value.0),
    };
    let (error_response_schema, error_response_schema_registration) =
        match args.error_response.map(|value| value.0) {
            Some(syn::Type::Tuple(tuple)) if tuple.elems.is_empty() => (
                quote::quote! {
                    fn openapi_error_response_schema(
                        _status: frontend_contract::RouteErrorStatus,
                    ) -> Option<frontend_contract::UtoipaOpenApiRouteSchema> {
                        None
                    }
                },
                proc_macro2::TokenStream::new(),
            ),
            Some(response_type) => (
                quote::quote! {
                    fn openapi_error_response_schema(
                        _status: frontend_contract::RouteErrorStatus,
                    ) -> Option<frontend_contract::UtoipaOpenApiRouteSchema> {
                        Some(frontend_contract::UtoipaOpenApiRouteSchema::from(
                            <#response_type as utoipa::ToSchema>::schema().1
                        ))
                    }
                },
                quote::quote! {
                    frontend_contract::register_openapi_schema::<#response_type>(components);
                },
            ),
            None => (
                proc_macro2::TokenStream::new(),
                proc_macro2::TokenStream::new(),
            ),
        };
    let obligations = args.obligations.map_or_else(
        || quote::quote!(&[]),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let openapi_operation_id = args.openapi_operation_id.0;
    let mut openapi_path_parameter = quote::quote!(None);
    let parameterized_route = match args.path_parameter {
        Some(parameter_type) => {
            let syn::Expr::Lit(path_expression) = &args.path.0 else {
                return syn::Error::new_spanned(
                    &args.path.0,
                    str_constants::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
                )
                .to_compile_error()
                .into();
            };
            let syn::Lit::Str(path_literal) = &path_expression.lit else {
                return syn::Error::new_spanned(
                    &path_expression.lit,
                    str_constants::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
                )
                .to_compile_error()
                .into();
            };
            let path_value = path_literal.value();
            let Some((prefix_value, placeholder_and_suffix)) = path_value.split_once('{') else {
                return syn::Error::new_spanned(
                    path_literal,
                    str_constants::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_PLACEHOLDER,
                )
                .to_compile_error()
                .into();
            };
            let Some((placeholder, suffix_value)) = placeholder_and_suffix.split_once('}') else {
                return syn::Error::new_spanned(
                    path_literal,
                    str_constants::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_CLOSED_PLACEHOLDER,
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
                    str_constants::TYPED_ROUTE_PARAMETER_PATH_SUPPORTS_ONE_PLACEHOLDER,
                )
                .to_compile_error()
                .into();
            }
            let prefix = syn::LitStr::new(prefix_value, path_literal.span());
            let suffix = syn::LitStr::new(suffix_value, path_literal.span());
            let parameter_name = syn::LitStr::new(placeholder, path_literal.span());
            let parameter_path = parameter_type.0;
            openapi_path_parameter = quote::quote! {
                Some(frontend_contract::UtoipaOpenApiPathParameter::from(
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name(#parameter_name)
                        .parameter_in(utoipa::openapi::path::ParameterIn::Path)
                        .required(utoipa::openapi::Required::True)
                        .schema(Some(<#parameter_path as utoipa::ToSchema>::schema().1))
                        .build()
                ))
            };
            quote::quote! {
                impl frontend_contract::ParameterizedRoute for #identifier {
                    type Parameter = #parameter_path;
                    fn path(parameter: &Self::Parameter) -> frontend_contract::ParameterizedRoutePath {
                        frontend_contract::ParameterizedRoutePath::try_from(format!("{}{}{}", #prefix, parameter, #suffix)).unwrap_or_default()
                    }
                }
            }
        }
        None => proc_macro2::TokenStream::new(),
    };
    let path = args.path.0;
    let request = args.request.0;
    let request_body = args.request_body.map_or_else(
        || quote::quote!(frontend_contract::RouteRequestBody::Absent),
        |value| quote::ToTokens::into_token_stream(&value.0),
    );
    let response = args.response.0;
    let response_schema = match &response {
        syn::Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == str_constants::VEC) =>
        {
            quote::quote! {
                Some(frontend_contract::UtoipaOpenApiRouteSchema::from(<#response as utoipa::PartialSchema>::schema()))
            }
        }
        _ => quote::quote! {
            Some(frontend_contract::UtoipaOpenApiRouteSchema::from(<#response as utoipa::ToSchema>::schema().1))
        },
    };
    let response_schema_registration = match &response {
        syn::Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == str_constants::VEC) =>
        {
            proc_macro2::TokenStream::new()
        }
        _ => quote::quote! {
            frontend_contract::register_openapi_schema::<#response>(components);
        },
    };
    let success_status = args.success_status.0;
    let transport = args.transport.0;
    quote::quote! {
        impl frontend_contract::TypedRoute for #identifier {
            type Request = #request;
            type Response = #response;
            type Transport = #transport;
            fn metadata() -> frontend_contract::RouteMetadata {
                frontend_contract::RouteMetadata::new_with_policy(
                    #authentication,
                    #error_statuses,
                    #method,
                    #mutation,
                    frontend_contract::ContractStr::from(#openapi_operation_id),
                    frontend_contract::ContractStr::from(#path),
                    #success_status,
                )
            }
            fn openapi_request_schema() -> Option<frontend_contract::UtoipaOpenApiRouteSchema> {
                Some(frontend_contract::UtoipaOpenApiRouteSchema::from(<#request as utoipa::ToSchema>::schema().1))
            }
            fn openapi_request_body_schema() -> Option<frontend_contract::UtoipaOpenApiRouteSchema> {
                let (name, _schema) = <#request as utoipa::ToSchema>::schema();
                Some(frontend_contract::UtoipaOpenApiRouteSchema::from(
                    utoipa::openapi::RefOr::Ref(utoipa::openapi::Ref::from_schema_name(name))
                ))
            }
            fn request_body() -> frontend_contract::RouteRequestBody {
                #request_body
            }
            fn openapi_response_schema() -> Option<frontend_contract::UtoipaOpenApiRouteSchema> {
                #response_schema
            }
            #error_response_schema
            fn openapi_path_parameter() -> Option<frontend_contract::UtoipaOpenApiPathParameter> {
                #openapi_path_parameter
            }
            fn register_openapi_schemas(
                components: &mut frontend_contract::UtoipaOpenApiComponentsRefMut<'_>,
            ) {
                if <Self as frontend_contract::TypedRoute>::request_body()
                    == frontend_contract::RouteRequestBody::Json
                {
                    frontend_contract::register_openapi_schema::<#request>(components);
                }
                #response_schema_registration
                #error_response_schema_registration
            }
        }
        impl frontend_contract::CoveredRoute for #identifier {
            fn coverage_descriptor() -> frontend_contract::RouteCoverageDescriptor {
                let metadata = <Self as frontend_contract::TypedRoute>::metadata();
                frontend_contract::RouteCoverageDescriptor::new(
                    metadata,
                    metadata.access(),
                    metadata.mutation(),
                    frontend_contract::RouteCoverageEvidence::new(#obligations),
                )
            }
        }
        #parameterized_route
    }
    .into()
}

#[proc_macro]
pub fn api_operation_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let paths = match syn::parse::Parser::parse(
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        input,
    ) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let mut path_iter = paths.into_iter();
    let Some(error_path) = path_iter.next() else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::API_OPERATION_ERROR_REQUIRES_ERROR_TYPE,
        )
        .to_compile_error()
        .into();
    };
    let Some(source_path) = path_iter.next() else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::API_OPERATION_ERROR_REQUIRES_SOURCE_TYPE,
        )
        .to_compile_error()
        .into();
    };
    let Some(render_path) = path_iter.next() else {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::API_OPERATION_ERROR_REQUIRES_RENDER_FUNCTION,
        )
        .to_compile_error()
        .into();
    };
    if path_iter.next().is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::API_OPERATION_ERROR_ACCEPTS_EXACTLY_THREE_PATHS,
        )
        .to_compile_error()
        .into();
    }
    quote::quote! {
        #[derive(Debug, thiserror::Error)]
        enum #error_path {
            #[error(transparent)]
            Operation(#source_path),
        }
        impl From<#source_path> for #error_path {
            fn from(value: #source_path) -> Self {
                Self::Operation(value)
            }
        }
        impl axum::response::IntoResponse for #error_path {
            fn into_response(self) -> axum::response::Response {
                match self {
                    Self::Operation(error) => #render_path(&error),
                }
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
            str_constants::ROUTE_ERROR_REQUIRES_ERROR_TYPE,
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
            str_constants::ROUTE_ERROR_REQUIRES_ASYNC_FUNCTION,
        )
        .to_compile_error()
        .into();
    }
    let return_type = match &function.sig.output {
        syn::ReturnType::Default => {
            return syn::Error::new_spanned(
                &function.sig,
                str_constants::ROUTE_ERROR_REQUIRES_EXPLICIT_RETURN_TYPE,
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
                                str_constants::ROUTE_ERROR_UNSUPPORTED_PARAMETER_PATTERN,
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
                    str_constants::ROUTE_ERROR_UNSUPPORTED_PARAMETER_PATTERN,
                )),
            },
            syn::FnArg::Receiver(value) => Err(syn::Error::new_spanned(
                value,
                str_constants::ROUTE_ERROR_REQUIRES_TYPED_PARAMETERS,
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
        str_constants::ROUTE_ERROR_UNUSED_ASYNC_REASON,
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
    quote::quote! {
        #[derive(Debug, thiserror::Error)]
        enum #error {}
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
            str_constants::ROUTE_OPERATION_ACCEPTS_NO_ARGUMENTS,
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
        .find(|attribute| attribute.path().is_ident(str_constants::ROUTE_CATALOG))
    else {
        return syn::Error::new_spanned(
            derive_input.ident,
            str_constants::ROUTE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match catalog_attribute.parse_args::<RouteCatalogArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Enum(data_enum) = derive_input.data else {
        return syn::Error::new_spanned(
            derive_input.ident,
            str_constants::ROUTE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let mut contract_arms = Vec::new();
    let mut family_routes = Vec::new();
    let mut path_arms = Vec::new();
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
                .is_ident(str_constants::ROUTE_CATALOG_ROUTE)
        }) else {
            return syn::Error::new_spanned(
                variant.ident,
                str_constants::ROUTE_CATALOG_VARIANT_REQUIRES_ROUTE,
            )
            .to_compile_error()
            .into();
        };
        let route_args = match route_attribute.parse_args::<RouteCatalogRouteArgs>() {
            Ok(value) => value,
            Err(error) => return error.to_compile_error().into(),
        };
        let variant_identifier = variant.ident;
        match (route_args.route, variant.fields) {
            (Some(route), syn::Fields::Unit) => {
                let route_type = route.0;
                contract_arms.push(quote::quote! {
                    Self::#variant_identifier => <#route_type as frontend_contract::TypedRoute>::metadata().contract()
                });
                path_arms.push(quote::quote! {
                    Self::#variant_identifier => frontend_contract::ParameterizedRoutePath::try_from(
                        String::from(frontend_contract::typed_route_path::<#route_type>())
                    ).unwrap_or_default()
                });
                family_routes.push(route_type);
            }
            (Some(route), syn::Fields::Unnamed(fields)) if fields.unnamed.len() == 1usize => {
                let route_type = route.0;
                contract_arms.push(quote::quote! {
                    Self::#variant_identifier(_value) => <#route_type as frontend_contract::TypedRoute>::metadata().contract()
                });
                path_arms.push(quote::quote! {
                    Self::#variant_identifier(value) => frontend_contract::typed_parameterized_route_path::<#route_type>(&value)
                });
                family_routes.push(route_type);
            }
            (Some(_route), syn::Fields::Named(_) | syn::Fields::Unnamed(_)) => {
                return syn::Error::new_spanned(
                    variant_identifier,
                    str_constants::ROUTE_CATALOG_ROUTE_SUPPORTS_UNIT_OR_SINGLE_FIELD_VARIANTS,
                )
                .to_compile_error()
                .into();
            }
            (None, syn::Fields::Unit) => {
                let Some(contract) = route_args.contract else {
                    return syn::Error::new_spanned(
                        variant_identifier,
                        str_constants::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                };
                let Some(path) = route_args.path else {
                    return syn::Error::new_spanned(
                        variant_identifier,
                        str_constants::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                };
                let contract_expression = contract.0;
                let path_expression = path.0;
                contract_arms.push(quote::quote! {
                    Self::#variant_identifier => #contract_expression
                });
                path_arms.push(quote::quote! {
                    Self::#variant_identifier => frontend_contract::ParameterizedRoutePath::try_from(
                        String::from(#path_expression)
                    ).unwrap_or_default()
                });
                if !route_args.exclude_from_family.0 {
                    return syn::Error::new_spanned(
                        variant_identifier,
                        str_constants::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES,
                    )
                    .to_compile_error()
                    .into();
                }
            }
            (None, syn::Fields::Named(_) | syn::Fields::Unnamed(_)) => {
                return syn::Error::new_spanned(
                    variant_identifier,
                    str_constants::ROUTE_CATALOG_CUSTOM_ROUTE_MUST_BE_UNIT,
                )
                .to_compile_error()
                .into();
            }
        }
    }
    let identifier = derive_input.ident;
    let family = args.family.0;
    let body_limit = args.body_limit.0;
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
        impl #identifier {
            #all

            #[must_use]
            pub fn contract(self) -> frontend_contract::RouteContract {
                match self {
                    #(#contract_arms),*
                }
            }
            fn catalog_path(self) -> frontend_contract::ParameterizedRoutePath {
                match self {
                    #(#path_arms),*
                }
            }
        }
        #[derive(Clone, Copy, Debug)]
        pub struct #family;
        impl frontend_contract::RouteFamily for #family {
            const ROUTE_COUNT: usize = #route_count;
            fn body_limit() -> Option<frontend_contract::RouteBodyLimit> {
                Some(frontend_contract::RouteBodyLimit::from(#body_limit))
            }
            fn coverage_descriptors() -> frontend_contract::RouteCoverageDescriptors {
                vec![
                    #(
                        <#family_routes as frontend_contract::CoveredRoute>::coverage_descriptor()
                    ),*
                ].into()
            }
            fn schema_contracts() -> frontend_contract::RouteSchemaContracts {
                vec![
                    #(
                        frontend_contract::RouteSchemaContract::from_typed_route::<#family_routes>()
                    ),*
                ].into()
            }
        }
        #(impl frontend_contract::RouteInFamily<#family> for #family_routes {})*
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
        return syn::Error::new_spanned(identifier, str_constants::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS)
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
                    str_constants::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS,
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

#[proc_macro_derive(PageCatalog, attributes(page_catalog, page_catalog_page))]
pub fn derive_page_catalog(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(attribute) = input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(str_constants::PAGE_CATALOG))
    else {
        return syn::Error::new_spanned(
            input.ident,
            str_constants::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
        )
        .to_compile_error()
        .into();
    };
    let args = match attribute.parse_args::<PageCatalogArgs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            str_constants::PAGE_CATALOG_SUPPORTS_UNIT_VARIANTS,
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
                    str_constants::PAGE_CATALOG_SUPPORTS_UNIT_VARIANTS,
                ));
            }
            let page_attribute = variant
                .attrs
                .iter()
                .find(|candidate| candidate.path().is_ident(str_constants::PAGE_CATALOG_PAGE))
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        &variant.ident,
                        str_constants::PAGE_CATALOG_VARIANT_REQUIRES_PAGE,
                    )
                })?;
            page_attribute
                .parse_args::<PageCatalogPageArgs>()
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
        .map(|(_identifier, page)| &page.capability.0)
        .collect::<Vec<_>>();
    let paths = pages
        .iter()
        .map(|(_identifier, page)| &page.path.0)
        .collect::<Vec<_>>();
    let metadata = pages
        .iter()
        .map(|(_identifier, page)| &page.metadata.0)
        .collect::<Vec<_>>();
    let routes = pages
        .iter()
        .map(|(_identifier, page)| &page.route.0)
        .collect::<Vec<_>>();
    let titles = pages
        .iter()
        .map(|(_identifier, page)| &page.title.0)
        .collect::<Vec<_>>();
    let identifier = &input.ident;
    let inventory = args.inventory.0;
    let path_ref = args.path_ref.0;
    let spec = args.spec.0;
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
    let route_count = route_types.len();
    quote::quote! {
        impl frontend_contract::RouteFamily for #identifier {
            const ROUTE_COUNT: usize = #route_count;
            #body_limit
            fn coverage_descriptors() -> frontend_contract::RouteCoverageDescriptors {
                vec![
                    #(
                        <#route_types as frontend_contract::CoveredRoute>::coverage_descriptor()
                    ),*
                ].into()
            }
            fn schema_contracts() -> frontend_contract::RouteSchemaContracts {
                vec![
                    #(
                        frontend_contract::RouteSchemaContract::from_typed_route::<#route_types>()
                    ),*
                ].into()
            }
        }
        #(impl frontend_contract::RouteInFamily<#identifier> for #route_types {})*
    }
    .into()
}

#[cfg(test)]
mod tests {
    fn typed_route_args(errors: &str) -> String {
        format!(
            "authentication = Authentication, {errors} method = Method, openapi_operation_id = \"operation\", path = \"/path\", request = Request, response = Response, success_status = Status, transport = Transport"
        )
    }

    #[test]
    #[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy
    fn typed_route_args_require_exactly_one_error_source() {
        ["", "error_policy = Policy, error_statuses = Statuses,"]
            .into_iter()
            .for_each(|errors| {
                let result =
                    syn::parse_str::<super::TypedRouteArgs>(typed_route_args(errors).as_str());
                let Err(error) = result else {
                    panic!("f58d0a31");
                };
                assert!(
                    error
                        .to_string()
                        .contains(str_constants::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES)
                );
            });
        ["error_policy = Policy,", "error_statuses = Statuses,"]
            .into_iter()
            .for_each(|errors| {
                let Ok(_args) =
                    syn::parse_str::<super::TypedRouteArgs>(typed_route_args(errors).as_str())
                else {
                    panic!("470bf91c");
                };
            });
    }

    #[test]
    fn route_registry_args_require_family_after_state() {
        let result = syn::parse_str::<super::RouteRegistryArgs>(
            "state = (), wrong = Family; (\"authenticated\", \"csrf\"); schemas(); (Route, handler),",
        );
        let Err(error) = result else {
            panic!("da287c44");
        };
        assert!(
            error
                .to_string()
                .contains(str_constants::ROUTE_REGISTRY_REQUIRES_FAMILY)
        );
    }

    #[test]
    fn route_registry_args_parse_family_and_bindings() {
        let result = syn::parse_str::<super::RouteRegistryArgs>(
            "state = (), family = Family; (\"authenticated\", \"csrf\"); schemas(Schema); (Route, handler),",
        );
        let Ok(args) = result else {
            panic!("6282e207");
        };
        assert_eq!(args.bindings.0.len(), 1usize);
        assert_eq!(args.schemas.0.len(), 1usize);
        assert_eq!(
            quote::ToTokens::to_token_stream(&args.family.0).to_string(),
            str_constants::FAMILY_UPPER_CAMEL_CASE
        );
    }
}
