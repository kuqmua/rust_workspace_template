#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "sibling generator pipeline modules construct the private table descriptor"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct GeneratePgTableModel {
    pub(in super::super) field_count:
        super::generate_pg_table_field_count::GeneratePgTableFieldCount,
    pub(in super::super) input:
        super::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput,
}
impl GeneratePgTableModel {
    #[must_use]
    pub const fn field_count(
        &self,
    ) -> super::generate_pg_table_field_count::GeneratePgTableFieldCount {
        self.field_count
    }
    pub(in super::super) fn into_input(
        self,
    ) -> super::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput {
        self.input
    }
    pub(in super::super) fn validate(
        self,
    ) -> Result<Self, super::syn_generate_pg_table_model_error::SynGeneratePgTableModelError> {
        if *self.field_count == constants_usize::ZERO {
            Err(syn::Error::new_spanned(
                &self.input.ident,
                constants_str::GENERATE_PG_TABLE_REQUIRES_FIELD,
            )
            .into())
        } else {
            Ok(self)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn field_count_preserves_validated_model_count() {
        let input: syn::DeriveInput = syn::parse_quote!(
            struct Example {
                value: u8,
            }
        );
        let model = super::GeneratePgTableModel {
            field_count:
                super::super::generate_pg_table_field_count::GeneratePgTableFieldCount::from(
                    constants_usize::ONE,
                ),
            input:
                super::super::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput::from(
                    input,
                ),
        };
        assert_eq!(usize::from(model.field_count()), constants_usize::ONE);
    }
}
