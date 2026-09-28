#[cfg(test)]
mod tests {
    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(bare, get_mut)]
    struct GetterBoundaryFixture {
        #[getters(copy)]
        get_value: workspace_macro_helpers::part_index::PartIndex,
        #[getters(copy)]
        r#type: workspace_macro_helpers::part_index::PartIndex,
        #[getters(copy)]
        value: workspace_macro_helpers::part_index::PartIndex,
    }

    impl From<workspace_macro_helpers::part_index::PartIndex> for GetterBoundaryFixture {
        fn from(value: workspace_macro_helpers::part_index::PartIndex) -> Self {
            Self {
                r#type: value,
                value,
                get_value: value,
            }
        }
    }

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
    struct TupleField(#[getters(copy)] u64);

    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(bare, legacy_refs)]
    struct BareFields {
        #[getters(copy)]
        count: u64,
        #[getters(skip)]
        text: String,
    }

    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(bare, legacy_refs)]
    struct LegacyReferenceField {
        text: String,
    }

    #[derive(
        proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    #[getters(bare, legacy_refs)]
    struct LegacyOptionalField {
        optional: Option<u16>,
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

        fn try_from(value: u64) -> Result<Self, Self::Error> {
            if value == 0 {
                Err(TupleFieldError::Zero)
            } else {
                Ok(Self(value))
            }
        }
    }

    #[test]
    fn test_bare_and_legacy_accessors_borrow_the_same_fields() {
        let legacy_reference_field = LegacyReferenceField {
            text: String::from(constants_str::A_ALT),
        };
        assert!(std::ptr::eq(
            legacy_reference_field.text(),
            legacy_reference_field.get_text()
        ));
        assert!(std::ptr::eq(
            legacy_reference_field.text(),
            legacy_reference_field.get_ref_text()
        ));
        let legacy_optional_field = LegacyOptionalField { optional: Some(3) };
        assert_eq!(legacy_optional_field.optional(), Some(&3));
        assert_eq!(legacy_optional_field.get_optional(), Some(&3));
        let absent = LegacyOptionalField { optional: None };
        assert_eq!(absent.optional(), None);
        assert_eq!(absent.get_optional(), None);
    }

    #[test]
    fn test_generates_named_optional_mutable_and_snake_case_getters() {
        let _proc_macro2_marker: Option<proc_macro2::TokenStream> = None;
        let _shared_macro_tokens =
            workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from(
                quote::quote!(),
            );
        let _syn_marker: Option<syn::DeriveInput> = None;
        let mut named = NamedFields {
            optional: Some(3),
            value: 5,
        };
        assert_eq!(named.get_ref_optional(), Some(&3));
        *named.get_value_mut() = 8;
        assert_eq!(*named.get_ref_value(), 8);
        assert_eq!(
            *PascalCaseField {
                RouteTypeUpperCamelCase: 13,
            }
            .get_ref_route_type_upper_camel_case(),
            13
        );
        let tuple = match TupleField::try_from(21) {
            Ok(value) => value,
            Err(TupleFieldError::Zero) => return,
        };
        assert_eq!(*tuple.get_ref(), 21);
        assert_eq!(tuple.get_value(), 21);
        let bare = BareFields {
            count: 34,
            text: String::from(constants_str::A_ALT),
        };
        assert_eq!(*bare.get_ref_count(), 34);
        assert_eq!(bare.count(), 34);
        assert_eq!(*bare.get_count(), 34);
        assert_eq!(bare.get_value_count(), 34);
        assert_eq!(bare.text_len(), constants_usize::ONE);
    }

    #[test]
    fn test_raw_and_prefixed_getters_read_and_mutate_distinct_fields() {
        let mut fixture = GetterBoundaryFixture::from(
            workspace_macro_helpers::part_index::PartIndex::from(constants_usize::ZERO),
        );
        let first = workspace_macro_helpers::part_index::PartIndex::from(constants_usize::ONE);
        let second = workspace_macro_helpers::part_index::PartIndex::from(constants_usize::TWO);
        let third = workspace_macro_helpers::part_index::PartIndex::from(constants_usize::THREE);
        *fixture.type_mut() = first;
        *fixture.value_mut() = second;
        *fixture.get_value_mut() = third;
        assert_eq!(fixture.r#type(), first);
        assert_eq!(fixture.get_ref_type(), &first);
        assert_eq!(fixture.get_value_type(), first);
        assert_eq!(fixture.value(), second);
        assert_eq!(fixture.get_ref_value(), &second);
        assert_eq!(fixture.get_value_value(), second);
        assert_eq!(fixture.get_value(), third);
        assert_eq!(fixture.get_ref_get_value(), &third);
        assert_eq!(fixture.get_value_get_value(), third);
    }

    const _: usize = constants_str::DOT.len();
    const _: usize = constants_usize::ZERO;
}
