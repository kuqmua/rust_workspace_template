#[must_use]
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationFile(&'static str);
#[derive(Debug, Clone, Copy, newtype::TryFrom)]
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
        if *value == 0u32 {
            Err(FieldLocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, newtype::TryFrom)]
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
        if *value == 0u32 {
            Err(FieldLocationCoordinateTryFromU32Error)
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct FieldLocationCoordinateTryFromU32Error;
#[must_use]
pub fn generate_field_location_new_token_stream(
    file: FieldLocationFile,
    line: FieldLocationLine,
    column: FieldLocationColumn,
) -> crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    let location_snake_case = naming::LocationSnakeCase;
    let location_new_token_stream = {
        let file_token_stream = generate_quotes::dq_token_stream(&file.0);
        let line_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line.0);
            quote::quote! {#literal}
        };
        let column_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(column.0);
            quote::quote! {#literal}
        };
        quote::quote! {
            location_lib::location::Location::new(
                file!(),
                location_lib::location::LocationLine::from(
                    std::num::NonZeroU32::new(line!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                location_lib::location::LocationColumn::from(
                    std::num::NonZeroU32::new(column!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                Some(location_lib::location::Occr {
                    file: location_lib::location::LocationFile::try_from(String::from(#file_token_stream)).unwrap_or_else(location_lib::location::LocationFile::from),
                    line: location_lib::location::LocationLine::try_from(#line_token_stream)
                        .unwrap_or_else(|_error| location_lib::location::LocationLine::first()),
                    column: location_lib::location::LocationColumn::try_from(#column_token_stream)
                        .unwrap_or_else(|_error| location_lib::location::LocationColumn::first()),
                })
            )
        }
    };
    crate::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {#location_snake_case: #location_new_token_stream},
    )
}
