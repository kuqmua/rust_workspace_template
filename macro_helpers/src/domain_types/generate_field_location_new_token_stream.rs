#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationFile(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::TryFrom)]
#[try_from(
    error = FieldLocationCoordinateTryFromU32Error,
    validator = FieldLocationLine::validate
)]
pub struct FieldLocationLine(u32);
impl From<std::num::NonZeroU32> for FieldLocationLine {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(value.get())
    }
}
impl FieldLocationLine {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), FieldLocationCoordinateTryFromU32Error> {
        if *value == constants_u32::ZERO {
            Err(FieldLocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::TryFrom)]
#[try_from(
    error = FieldLocationCoordinateTryFromU32Error,
    validator = FieldLocationColumn::validate
)]
pub struct FieldLocationColumn(u32);
impl From<std::num::NonZeroU32> for FieldLocationColumn {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(value.get())
    }
}
impl FieldLocationColumn {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u32) -> Result<(), FieldLocationCoordinateTryFromU32Error> {
        if *value == constants_u32::ZERO {
            Err(FieldLocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct FieldLocationCoordinateTryFromU32Error;
#[must_use]
pub fn generate_field_location_new_token_stream(
    file: FieldLocationFile,
    line: FieldLocationLine,
    column: FieldLocationColumn,
) -> crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
{
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let location_new_token_stream = {
        let file_token_stream = generate_quotes::domain_types::dq_token_stream(&file.0);
        let line_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line.0);
            quote::quote! {#literal}
        };
        let column_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(column.0);
            quote::quote! {#literal}
        };
        quote::quote! {
            location_lib::domain_types::Location::new(
                file!(),
                location_lib::domain_types::LocationLine::from(
                    std::num::NonZeroU32::new(line!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                location_lib::domain_types::LocationColumn::from(
                    std::num::NonZeroU32::new(column!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                Some(location_lib::domain_types::Occr {
                    file: location_lib::domain_types::LocationFile::try_from(String::from(#file_token_stream)).unwrap_or_else(location_lib::domain_types::LocationFile::from),
                    line: location_lib::domain_types::LocationLine::try_from(#line_token_stream)
                        .unwrap_or_else(|_error| location_lib::domain_types::LocationLine::first()),
                    column: location_lib::domain_types::LocationColumn::try_from(#column_token_stream)
                        .unwrap_or_else(|_error| location_lib::domain_types::LocationColumn::first()),
                })
            )
        }
    };
    crate::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {#location_snake_case: #location_new_token_stream},
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn coordinates_reject_zero_and_accept_positive_values() {
        let _line_error = super::FieldLocationLine::try_from(constants_u32::ZERO)
            .expect_err(constants_str::VALUE_A7ABF9D9);
        let _column_error = super::FieldLocationColumn::try_from(constants_u32::ZERO)
            .expect_err(constants_str::VALUE_B430FE14);
        let _line = super::FieldLocationLine::try_from(7u32).expect(
            "070dbee8 coordinates_reject_zero_and_accept_positive_values invariant must hold",
        );
        let _column = super::FieldLocationColumn::try_from(11u32).expect(
            "e067c790 coordinates_reject_zero_and_accept_positive_values invariant must hold",
        );
    }
    #[test]
    fn first_coordinates_generate_complete_location_field() {
        let generated = super::generate_field_location_new_token_stream(
            super::FieldLocationFile::from(constants_str::VALUE_E1CEB1AF),
            super::FieldLocationLine::first(),
            super::FieldLocationColumn::first(),
        )
        .as_ref()
        .to_string();
        assert!(
            generated.starts_with("location : location_lib :: domain_types :: Location :: new")
        );
        assert!(generated.contains("\"src/example.rs\""));
        assert!(generated.contains("try_from (1)"));
        assert!(generated.contains("file ! ()"));
        assert!(generated.contains("line ! ()"));
        assert!(generated.contains("column ! ()"));
    }
}
