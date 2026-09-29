pub fn only_one(
    syn_variant_ref: crate::syn_variant_ref::SynVariantRef<'_>,
) -> Result<crate::status_code::StatusCode, crate::only_one_status_code_error::OnlyOneStatusCodeError>
{
    let variant = syn_variant_ref.variant();
    variant
        .attrs
        .iter()
        .try_fold(None, |supported_status_code, attr| {
            if attr.path().segments.len() != 1 {
                return Ok(supported_status_code);
            }
            let Some(segment) = attr.path().segments.first() else {
                return Ok(supported_status_code);
            };
            let Ok(status_code) =
                crate::status_code::StatusCode::try_from(&segment.ident.to_string())
            else {
                return Ok(supported_status_code);
            };
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return Err(
                    crate::only_one_status_code_error::OnlyOneStatusCodeError::MalformedAttribute,
                );
            }
            if supported_status_code.is_some() {
                return Err(crate::only_one_status_code_error::OnlyOneStatusCodeError::MoreThanOne);
            }
            Ok(Some(status_code))
        })?
        .ok_or(crate::only_one_status_code_error::OnlyOneStatusCodeError::NotFound)
}
