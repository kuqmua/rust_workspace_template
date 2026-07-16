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
    method: SynExpr,
    openapi_operation_id: SynExpr,
    path: SynExpr,
    request: SynType,
    response: SynType,
    transport: SynType,
}

impl syn::parse::Parse for TypedRouteArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut method = None;
        let mut openapi_operation_id = None;
        let mut path = None;
        let mut request = None;
        let mut response = None;
        let mut transport = None;
        while !input.is_empty() {
            let name: syn::Ident = input.parse()?;
            let _equals: syn::Token![=] = input.parse()?;
            match name.to_string().as_str() {
                str_constants::METHOD => {
                    method = Some(SynExpr::from(input.parse::<syn::Expr>()?));
                }
                str_constants::OPENAPI_OPERATION_ID => {
                    openapi_operation_id = Some(SynExpr::from(input.parse::<syn::Expr>()?));
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
            method: method
                .ok_or_else(|| input.error(str_constants::TYPED_ROUTE_REQUIRES_METHOD))?,
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
    let method = args.method.0;
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
                    frontend_contract::ContractStr::from(#method),
                    frontend_contract::ContractStr::from(#openapi_operation_id),
                    frontend_contract::ContractStr::from(#path),
                )
            }
        }
    }
    .into()
}
