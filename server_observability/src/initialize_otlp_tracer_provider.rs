#[allow(
    clippy::single_call_fn,
    reason = "the exporter initialization boundary requires direct unit coverage proving disabled export creates no network client"
)]
pub(super) fn initialize_otlp_tracer_provider(
    otlp_export_mode: crate::otlp_export_mode::OtlpExportMode,
    service_name: crate::service_name::ServiceName,
) -> Result<
    Option<crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider>,
    crate::observability_init_error::ObservabilityInitError,
> {
    if otlp_export_mode == crate::otlp_export_mode::OtlpExportMode::Disabled {
        return Ok(None);
    }
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()
        .map_err(|error| {
            crate::observability_init_error::ObservabilityInitError::Exporter(
                crate::opentelemetry_otlp_exporter_build_error::OpentelemetryOtlpExporterBuildError::from(error),
            )
        })?;
    Ok(Some(
        crate::opentelemetry_sdk_tracer_provider::OpentelemetrySdkTracerProvider::from(
            opentelemetry_sdk::trace::SdkTracerProvider::builder()
                .with_batch_exporter(exporter)
                .with_resource(
                    opentelemetry_sdk::Resource::builder()
                        .with_service_name(service_name.get())
                        .build(),
                )
                .build(),
        ),
    ))
}
