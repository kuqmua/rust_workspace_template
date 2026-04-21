use std::path::PathBuf;

#[must_use]
pub fn build_calculation_report_golden_file_path(manifest_directory: &str) -> PathBuf {
    PathBuf::from(manifest_directory)
        .join("tests")
        .join("golden")
        .join("calculation_report.txt")
}

#[must_use]
pub const fn build_standard_division_operands() -> (i64, i64) {
    (27, 3)
}

#[must_use]
pub const fn build_standard_operand_text_triplet() -> (&'static str, &'static str, &'static str) {
    ("27", "/", "3")
}

#[must_use]
pub const fn build_invalid_wire_format_with_extra_parts() -> &'static str {
    "1|+|2|extra"
}
