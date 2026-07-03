#[derive(Debug, Clone, Copy)]
pub struct RustSourceFilePath;

#[must_use]
pub const fn rs_file_path() -> RustSourceFilePath {
    RustSourceFilePath
}
