#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters)]
pub(crate) struct PageCatalogArgs {
    inventory: crate::contract_syn_ident::ContractSynIdent,
    path_ref: crate::contract_syn_type::ContractSynType,
    spec: crate::contract_syn_type::ContractSynType,
}

impl syn::parse::Parse for PageCatalogArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !parse_stream.is_empty() {
            let name = parse_stream.parse::<syn::Ident>()?;
            let _equals = parse_stream.parse::<syn::Token![=]>()?;
            if name == constants_str::PAGE_CATALOG_INVENTORY {
                inventory = Some(crate::contract_syn_ident::ContractSynIdent::from(
                    parse_stream.parse::<syn::Ident>()?,
                ));
            } else if name == constants_str::PAGE_CATALOG_PATH_REF {
                path_ref = Some(crate::contract_syn_type::ContractSynType::from(
                    parse_stream.parse::<syn::Type>()?,
                ));
            } else if name == constants_str::PAGE_CATALOG_SPEC {
                spec = Some(crate::contract_syn_type::ContractSynType::from(
                    parse_stream.parse::<syn::Type>()?,
                ));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !parse_stream.is_empty() {
                let _comma = parse_stream.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory.ok_or_else(|| {
                parse_stream.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            path_ref: path_ref.ok_or_else(|| {
                parse_stream.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            spec: spec.ok_or_else(|| {
                parse_stream.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
        })
    }
}
