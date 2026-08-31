#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
pub(crate) struct WireEnumAttrs {
    error_message: crate::syn_expr::SynExpr,
    ref_type: crate::syn_type::SynType,
}
impl syn::parse::Parse for WireEnumAttrs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut error_message = None;
        let mut ref_type = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::WIRE_ENUM_REF_TYPE {
                ref_type = Some(crate::syn_type::SynType::from(input.parse::<syn::Type>()?));
            } else if name == constants_str::WIRE_ENUM_ERROR_MESSAGE {
                error_message = Some(crate::syn_expr::SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            error_message: error_message
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
            ref_type: ref_type
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
        })
    }
}
