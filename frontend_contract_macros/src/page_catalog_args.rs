#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
pub(crate) struct PageCatalogArgs {
    pub inventory: crate::contract_syn_ident::ContractSynIdent,
    pub path_ref: crate::contract_syn_type::ContractSynType,
    pub spec: crate::contract_syn_type::ContractSynType,
}

impl syn::parse::Parse for PageCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::test_fixtures::PAGE_CATALOG_INVENTORY {
                inventory = Some(crate::contract_syn_ident::ContractSynIdent::from(
                    input.parse::<syn::Ident>()?,
                ));
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_PATH_REF {
                path_ref = Some(crate::contract_syn_type::ContractSynType::from(
                    input.parse::<syn::Type>()?,
                ));
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_SPEC {
                spec = Some(crate::contract_syn_type::ContractSynType::from(
                    input.parse::<syn::Type>()?,
                ));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            path_ref: path_ref.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            spec: spec.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
        })
    }
}
