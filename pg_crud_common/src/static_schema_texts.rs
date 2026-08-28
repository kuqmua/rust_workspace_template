pub(crate) fn static_schema_texts(
    values: super::DbStaticSchemaTexts,
) -> Result<super::DbSchemaTexts, super::DbSchemaConformanceError> {
    Vec::from(values)
        .into_iter()
        .map(super::static_schema_text)
        .collect::<Result<Vec<_>, _>>()
        .map(Into::into)
}
