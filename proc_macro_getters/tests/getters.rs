#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(get_mut)]
    struct NamedFields {
        optional: Option<u16>,
        value: u8,
    }

    #[allow(
        non_snake_case,
        reason = "fixture verifies generated snake_case names for macro-oriented identifiers"
    )]
    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    struct PascalCaseField {
        RouteTypeUpperCamelCase: u32,
    }

    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    struct TupleField(u64);

    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(bare)]
    struct BareFields {
        #[getters(copy)]
        count: u64,
        #[getters(skip)]
        text: String,
    }

    impl BareFields {
        fn text_len(&self) -> usize {
            self.text.len()
        }
    }

    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    enum TupleFieldError {
        Zero,
    }

    impl TryFrom<u64> for TupleField {
        type Error = TupleFieldError;

        fn try_from(u64: u64) -> Result<Self, Self::Error> {
            if u64 == 0 {
                Err(TupleFieldError::Zero)
            } else {
                Ok(Self(u64))
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
        let bare = BareFields {
            count: 34,
            text: String::from(constants_str::A_ALT),
        };
        assert_eq!(bare.count(), 34);
        assert_eq!(bare.text_len(), constants_usize::ONE);
    }

    const _: usize = constants_str::DOT.len();
    const _: usize = constants_usize::ZERO;
}
