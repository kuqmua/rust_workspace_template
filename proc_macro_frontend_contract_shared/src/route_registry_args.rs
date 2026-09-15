#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteRegistryArgs {
    authenticated_security: crate::contract_syn_expr::ContractSynExpr,
    bindings: crate::syn_route_registry_bindings::SynRouteRegistryBindings,
    csrf_security: crate::contract_syn_expr::ContractSynExpr,
    family: crate::syn_route_registry_family::SynRouteRegistryFamily,
    schemas: crate::syn_route_registry_schemas::SynRouteRegistrySchemas,
    state: crate::syn_route_registry_state::SynRouteRegistryState,
}
impl syn::parse::Parse for RouteRegistryArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let state_name = parse_stream.parse::<syn::Ident>()?;
        if state_name != constants_str::STATE {
            return Err(syn::Error::new_spanned(
                state_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_STATE,
            ));
        }
        let _equals = parse_stream.parse::<syn::Token![=]>()?;
        let state = crate::syn_route_registry_state::SynRouteRegistryState::from(
            parse_stream.parse::<syn::Type>()?,
        );
        let _state_comma = parse_stream.parse::<syn::Token![,]>()?;
        let family_name = parse_stream.parse::<syn::Ident>()?;
        if family_name != constants_str::FAMILY {
            return Err(syn::Error::new_spanned(
                family_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_FAMILY,
            ));
        }
        let _family_equals = parse_stream.parse::<syn::Token![=]>()?;
        let family = crate::syn_route_registry_family::SynRouteRegistryFamily::from(
            parse_stream.parse::<syn::Type>()?,
        );
        let _family_semicolon = parse_stream.parse::<syn::Token![;]>()?;
        let security_content;
        let _security_parenthesis = syn::parenthesized!(security_content in parse_stream);
        let authenticated_security =
            crate::contract_syn_expr::ContractSynExpr::from(security_content.parse::<syn::Expr>()?);
        let _comma = security_content.parse::<syn::Token![,]>()?;
        let csrf_security =
            crate::contract_syn_expr::ContractSynExpr::from(security_content.parse::<syn::Expr>()?);
        let _security_semicolon = parse_stream.parse::<syn::Token![;]>()?;
        let schemas_name = parse_stream.parse::<syn::Ident>()?;
        if schemas_name != constants_str::SCHEMAS {
            return Err(syn::Error::new_spanned(
                schemas_name,
                constants_str::ROUTE_REGISTRY_REQUIRES_SCHEMAS,
            ));
        }
        let schemas_content;
        let _schemas_parenthesis = syn::parenthesized!(schemas_content in parse_stream);
        let schemas = syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated(
            &schemas_content,
        )?
        .into_iter()
        .collect::<Vec<_>>();
        let _schemas_semicolon = parse_stream.parse::<syn::Token![;]>()?;
        let bindings = syn::punctuated::Punctuated::<
            crate::route_registry_binding::RouteRegistryBinding,
            syn::Token![,],
        >::parse_terminated(parse_stream)?;
        if bindings.is_empty() {
            return Err(parse_stream.error(constants_str::ROUTE_REGISTRY_REQUIRES_BINDING));
        }
        Ok(Self::new(
            authenticated_security,
            crate::syn_route_registry_bindings::SynRouteRegistryBindings::from(bindings),
            csrf_security,
            family,
            crate::syn_route_registry_schemas::SynRouteRegistrySchemas::from(schemas),
            state,
        ))
    }
}
