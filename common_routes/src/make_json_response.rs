use super::{AxumJsonPayload, JsonRes};

pub(crate) fn make_json_response<T>(payload: T) -> JsonRes<T> {
    JsonRes {
        payload: AxumJsonPayload::from(axum::Json(payload)),
    }
}
