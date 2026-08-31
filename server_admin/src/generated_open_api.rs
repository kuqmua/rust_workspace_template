#[must_use]
pub fn generated_open_api() -> crate::utoipa_admin_open_api::UtoipaAdminOpenApi {
    fn collect_schema_refs(
        value: &serde_json::Value,
        refs: &mut std::collections::BTreeSet<String>,
    ) {
        match value {
            serde_json::Value::Array(values) => values
                .iter()
                .for_each(|child| collect_schema_refs(child, refs)),
            serde_json::Value::Object(values) => values.iter().for_each(|(key, child)| {
                if key == constants_str::DOLLAR_REF
                    && let Some(name) = child.as_str().and_then(|reference| {
                        reference.strip_prefix(constants_str::COMPONENTS_SCHEMAS)
                    })
                {
                    let _inserted = refs.insert(name.to_owned());
                }
                collect_schema_refs(child, refs);
            }),
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    let mut document = utoipa::openapi::OpenApi::from(
        crate::admin_generated_table::AdminGeneratedTable::ALL[0].open_api(),
    );
    document.merge(utoipa::openapi::OpenApi::from(
        crate::admin_api_open_api::admin_api_open_api(),
    ));
    crate::admin_generated_table::AdminGeneratedTable::ALL[1..]
        .iter()
        .copied()
        .for_each(|table| {
            document.merge(utoipa::openapi::OpenApi::from(table.open_api()));
        });
    let mut refs = std::collections::BTreeSet::new();
    if let Ok(value) = serde_json::to_value(&document) {
        collect_schema_refs(&value, &mut refs);
    }
    if let Some(components) = document.components.as_mut() {
        refs.into_iter().for_each(|name| {
            if !components.schemas.contains_key(name.as_str())
                && let Some(suffix) = name.rsplit('.').next()
                && let Some(schema) = components.schemas.get(suffix).cloned()
            {
                let _previous = components.schemas.insert(name, schema);
            }
        });
    }
    crate::utoipa_admin_open_api::UtoipaAdminOpenApi::from(document)
}
