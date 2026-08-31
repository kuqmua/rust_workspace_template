pub fn only_one(
    variant_ref: crate::syn_variant_ref::SynVariantRef<'_>,
) -> Result<crate::status_code::StatusCode, crate::only_one_status_code_error::OnlyOneStatusCodeError>
{
    let variant = variant_ref.variant();
    let mut supported_attrs = variant.attrs.iter().filter_map(|attr| {
        if attr.path().segments.len() != 1 {
            return None;
        }
        let segment = attr.path().segments.first()?;
        crate::status_code::StatusCode::try_from(&segment.ident.to_string()).ok()
    });
    let optional_self = supported_attrs.next();
    if supported_attrs.next().is_some() {
        return Err(crate::only_one_status_code_error::OnlyOneStatusCodeError::MoreThanOne);
    }
    optional_self.ok_or(crate::only_one_status_code_error::OnlyOneStatusCodeError::NotFound)
}
