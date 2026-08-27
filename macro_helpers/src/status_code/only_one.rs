pub fn only_one(
    variant_ref: super::SynStatusCodeVariantRef<'_>,
) -> Result<super::StatusCode, super::OnlyOneStatusCodeError> {
    let variant = variant_ref.0;
    let mut supported_attrs = variant.attrs.iter().filter_map(|attr| {
        if attr.path().segments.len() != 1 {
            return None;
        }
        let segment = attr.path().segments.first()?;
        super::StatusCode::try_from(&segment.ident.to_string()).ok()
    });
    let optional_self = supported_attrs.next();
    if supported_attrs.next().is_some() {
        return Err(super::OnlyOneStatusCodeError::MoreThanOne);
    }
    optional_self.ok_or(super::OnlyOneStatusCodeError::NotFound)
}
