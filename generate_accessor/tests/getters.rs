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

    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    enum TupleFieldError {
        Zero,
    }

    impl TryFrom<u64> for TupleField {
        type Error = TupleFieldError;

        fn try_from(value: u64) -> Result<Self, Self::Error> {
            if value == 0 {
                Err(TupleFieldError::Zero)
            } else {
                Ok(Self(value))
            }
        }
    }

    #[test]
    fn test_generates_named_optional_mutable_and_snake_case_getters() {
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
        let tuple = match TupleField::try_from(21) {
            Ok(value) => value,
            Err(TupleFieldError::Zero) => return,
        };
        assert_eq!(*tuple.get_inner(), 21);
    }

    const _: usize = constants_str::DOT.len();
    const _: usize = constants_usize::ZERO;
}
