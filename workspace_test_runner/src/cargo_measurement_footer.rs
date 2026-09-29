#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct CargoMeasurementFooter<'text_lt> {
    #[getters(copy)]
    major_page_faults: crate::stderr_text_ref::StderrTextRef<'text_lt>,
    #[getters(copy)]
    minor_page_faults: crate::stderr_text_ref::StderrTextRef<'text_lt>,
    #[getters(copy)]
    peak_rss_kb: crate::stderr_text_ref::StderrTextRef<'text_lt>,
    #[getters(copy)]
    program_text: crate::stderr_text_ref::StderrTextRef<'text_lt>,
}
