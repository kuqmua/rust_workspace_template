#[cfg(test)]
mod tests {
    #[derive(generate_accessor::Getters, optimal_memory_layout::OptimalMemoryLayout)]
    #[getters(get_mut)]
    struct NamedFields {
        optional: Option<u16>,
        value: u8,
    }

    #[allow(
        non_snake_case,
        reason = "fixture verifies generated snake_case names for macro-oriented identifiers"
    )]
    #[derive(generate_accessor::Getters, optimal_memory_layout::OptimalMemoryLayout)]
    struct PascalCaseField {
        RouteTypeUpperCamelCase: u32,
    }

    #[derive(generate_accessor::Getters, optimal_memory_layout::OptimalMemoryLayout)]
    struct TupleField(u64);

    #[test]
    fn generates_named_optional_mutable_and_snake_case_getters() {
        let _proc_macro2_marker: Option<proc_macro2::TokenStream> = None;
        let _quote_marker = quote::quote!();
        let _syn_marker: Option<syn::DeriveInput> = None;
        let mut named = NamedFields {
            optional: Some(3),
            value: 5,
        };
        assert_eq!(named.get_optional(), Some(&3));
        *named.get_value_mut() = 8;
        assert_eq!(*named.get_value(), 8);
        assert_eq!(
            *PascalCaseField {
                RouteTypeUpperCamelCase: 13,
            }
            .get_route_type_upper_camel_case(),
            13
        );
        assert_eq!(*TupleField(21).get_inner(), 21);
    }

    const _: usize = constants_str::catalog::DOT.len();
    const _: usize = constants_usize::ZERO;
}
