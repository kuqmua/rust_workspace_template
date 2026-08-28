pub(crate) fn assignment_ids_impl<Id, IdError, Ids, IdsError>(
    value: &crate::AdminHtmlFormText,
) -> Result<Ids, crate::AdminError>
where
    Id: TryFrom<i64, Error = IdError>,
    Ids: TryFrom<Vec<Id>, Error = IdsError>,
{
    if value.is_empty() {
        return Ids::try_from(Vec::new()).map_err(|_error| crate::AdminError::Validation);
    }
    let values = value
        .split(',')
        .map(|item| {
            let parsed = item
                .parse::<i64>()
                .map_err(|_error| crate::AdminError::Validation)?;
            Id::try_from(parsed).map_err(|_error| crate::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ids::try_from(values).map_err(|_error| crate::AdminError::Validation)
}
