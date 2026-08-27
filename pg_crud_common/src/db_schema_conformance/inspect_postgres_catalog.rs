use super::*;

pub async fn inspect_postgres_catalog(
    pool: SqlxPgPoolRef<'_>,
    schema: DbSchemaNameRef<'_>,
) -> Result<DbCatalogSnapshot, DbSchemaConformanceError> {
    let rows = sqlx::query(constants_str::DB_SCHEMA_CATALOG_QUERY)
        .bind(schema.0)
        .fetch_all(pool.0)
        .await
        .map_err(|error| {
            DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
        })?;
    rows.into_iter()
        .map(|row| {
            let kind_text: String =
                sqlx::Row::try_get(&row, constants_str::OBJECT_KIND).map_err(|error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                })?;
            let kind = match kind_text.as_str() {
                constants_str::EXTENSION => DbObjectKind::Extension,
                constants_str::FUNCTION => DbObjectKind::Function,
                constants_str::TRIGGER => DbObjectKind::Trigger,
                constants_str::VIEW => DbObjectKind::View,
                _ => return Err(DbSchemaConformanceError::UnknownObjectKind),
            };
            let name = sqlx::Row::try_get::<String, _>(&row, constants_str::OBJECT_NAME).map_err(
                |error| {
                    DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
                },
            )?;
            let definition = sqlx::Row::try_get::<String, _>(
                &row,
                constants_str::OBJECT_DEFINITION,
            )
            .map_err(|error| {
                DbSchemaConformanceError::Inspection(SqlxDbSchemaInspectionError::from(error))
            })?;
            Ok(DbObjectSnapshot::new(
                DbSchemaText::try_from(name).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
                kind,
                DbSchemaText::try_from(definition).map_err(|error| {
                    DbSchemaConformanceError::SchemaTextTooLong(DbSchemaTextError::from(error))
                })?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|objects| DbCatalogSnapshot::new(objects.into()))
}
