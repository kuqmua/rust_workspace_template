#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(generate_constructor::New, optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct GeneratePgTableModel {
    #[getters(copy)]
    field_count: super::generate_pg_table_field_count::GeneratePgTableFieldCount,
    input: super::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput,
}
impl GeneratePgTableModel {
    pub(crate) fn into_input(
        self,
    ) -> super::syn_generate_pg_table_model_input::SynGeneratePgTableModelInput {
        self.input
    }
    pub(crate) fn validate(
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
    fn test_field_count_preserves_validated_model_count() {
        let input: syn::DeriveInput = syn::parse_quote!(
            struct Example {
                value: u8,
            }
        );
        let model = crate::generate_pg_table_model::GeneratePgTableModel {
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
