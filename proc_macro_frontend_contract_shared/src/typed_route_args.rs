#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    authentication: crate::contract_syn_expr::ContractSynExpr,
    error_response: Option<crate::contract_syn_type::ContractSynType>,
    errors: crate::syn_typed_route_errors::SynTypedRouteErrors,
    method: crate::contract_syn_expr::ContractSynExpr,
    mutation: Option<crate::contract_syn_expr::ContractSynExpr>,
    obligations: Option<crate::contract_syn_expr::ContractSynExpr>,
    openapi_operation_id: crate::contract_syn_expr::ContractSynExpr,
    path: crate::contract_syn_expr::ContractSynExpr,
    path_parameter: Option<crate::contract_syn_type::ContractSynType>,
    request: crate::contract_syn_type::ContractSynType,
    request_body: Option<crate::contract_syn_expr::ContractSynExpr>,
    response: crate::contract_syn_type::ContractSynType,
    success_status: crate::contract_syn_expr::ContractSynExpr,
    transport: crate::contract_syn_type::ContractSynType,
}
impl syn::parse::Parse for TypedRouteArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
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
        while !parse_stream.is_empty() {
            let name: syn::Ident = parse_stream.parse()?;
            let _equals: syn::Token![=] = parse_stream.parse()?;
            match &name {
                value if value == constants_str::TYPED_ROUTE_FIELD_AUTHENTICATION => {
                    authentication = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_ERROR_STATUSES => {
                    error_statuses = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_ERROR_RESPONSE => {
                    error_response = Some(crate::contract_syn_type::ContractSynType::from(
                        parse_stream.parse::<syn::Type>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_ERROR_POLICY => {
                    error_policy = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::METHOD => {
                    method = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::OPENAPI_OPERATION_ID => {
                    openapi_operation_id = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::MUTATION => {
                    mutation = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::OBLIGATIONS => {
                    obligations = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_PATH => {
                    path = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_PATH_PARAMETER => {
                    path_parameter = Some(crate::contract_syn_type::ContractSynType::from(
                        parse_stream.parse::<syn::Type>()?,
                    ));
                }
                value if value == constants_str::REQUEST => {
                    request = Some(crate::contract_syn_type::ContractSynType::from(
                        parse_stream.parse::<syn::Type>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_REQUEST_BODY => {
                    request_body = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::RESPONSE => {
                    response = Some(crate::contract_syn_type::ContractSynType::from(
                        parse_stream.parse::<syn::Type>()?,
                    ));
                }
                value if value == constants_str::TYPED_ROUTE_FIELD_SUCCESS_STATUS => {
                    success_status = Some(crate::contract_syn_expr::ContractSynExpr::from(
                        parse_stream.parse::<syn::Expr>()?,
                    ));
                }
                value if value == constants_str::TRANSPORT => {
                    transport = Some(crate::contract_syn_type::ContractSynType::from(
                        parse_stream.parse::<syn::Type>()?,
                    ));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        name,
                        constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                    ));
                }
            }
            if !parse_stream.is_empty() {
                let _comma: syn::Token![,] = parse_stream.parse()?;
            }
        }
        let errors = match (error_policy, error_statuses) {
            (Some(policy), None) => {
                crate::syn_typed_route_errors::SynTypedRouteErrors::Policy(policy)
            }
            (None, Some(statuses)) => {
                crate::syn_typed_route_errors::SynTypedRouteErrors::Statuses(statuses)
            }
            (None, None) | (Some(_), Some(_)) => {
                return Err(parse_stream
                    .error(constants_str::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES));
            }
        };
        Ok(Self::new(
            authentication.ok_or_else(|| {
                parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_AUTHENTICATION)
            })?,
            error_response,
            errors,
            method.ok_or_else(|| parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_METHOD))?,
            mutation,
            obligations,
            openapi_operation_id.ok_or_else(|| {
                parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_OPERATION_ID)
            })?,
            path.ok_or_else(|| parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_PATH))?,
            path_parameter,
            request
                .ok_or_else(|| parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_REQUEST))?,
            request_body,
            response
                .ok_or_else(|| parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_RESPONSE))?,
            success_status.ok_or_else(|| {
                parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_SUCCESS_STATUS)
            })?,
            transport
                .ok_or_else(|| parse_stream.error(constants_str::TYPED_ROUTE_REQUIRES_TRANSPORT))?,
        ))
    }
}
