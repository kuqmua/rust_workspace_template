#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok,
}

#[must_use]
pub const fn get_only_one<Value>(_value: &Value) -> StatusCode {
    StatusCode::Ok
}
