#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters)]
pub(crate) struct WireEnumAttrs {
    error_message: crate::syn_expr::SynExpr,
    ref_type: crate::syn_type::SynType,
}
impl syn::parse::Parse for WireEnumAttrs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut error_message = None;
        let mut ref_type = None;
        while !parse_stream.is_empty() {
            let name = parse_stream.parse::<syn::Ident>()?;
            let _equals = parse_stream.parse::<syn::Token![=]>()?;
            if name == constants_str::WIRE_ENUM_REF_TYPE {
                ref_type = Some(crate::syn_type::SynType::from(
                    parse_stream.parse::<syn::Type>()?,
                ));
            } else if name == constants_str::WIRE_ENUM_ERROR_MESSAGE {
                error_message = Some(crate::syn_expr::SynExpr::from(
                    parse_stream.parse::<syn::Expr>()?,
                ));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
                ));
            }
            if !parse_stream.is_empty() {
                let _comma = parse_stream.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            error_message: error_message
                .ok_or_else(|| parse_stream.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
            ref_type: ref_type
                .ok_or_else(|| parse_stream.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
        })
    }
}
