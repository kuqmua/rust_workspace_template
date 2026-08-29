use super::{SynExpr, SynIdent};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogArgs {
    body_limit: SynExpr,
    family: SynIdent,
}

impl syn::parse::Parse for RouteCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut body_limit = None;
        let mut family = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::ROUTE_CATALOG_FAMILY {
                family = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::ROUTE_CATALOG_BODY_LIMIT {
                body_limit = Some(SynExpr::from(input.parse::<syn::Expr>()?));
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
