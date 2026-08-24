#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let Some((parser_byte, value_bytes)) = data.split_first() else {
        return;
    };
    if parser_byte & 7u8 == 7u8 {
        let Some((key_len_byte, remaining)) = value_bytes.split_first() else {
            return;
        };
        let key_len = usize::from(*key_len_byte).min(remaining.len()).min(64usize);
        let (key_bytes, cursor_bytes) = remaining.split_at(key_len);
        let Ok(key) = pg_crud_common::CursorSigningKey::try_from(key_bytes.to_vec()) else {
            return;
        };
        let Ok(maximum_length) = pg_crud_common::CursorMaximumLength::try_from(4_096usize) else {
            return;
        };
        let Ok(cursor_text) = std::str::from_utf8(cursor_bytes) else {
            return;
        };
        let Ok(cursor) = pg_crud_common::SignedCursor::try_from(cursor_text.to_owned()) else {
            return;
        };
        let codec = pg_crud_common::CursorCodec::new(key, maximum_length);
        drop(codec.decode(&cursor));
        return;
    }
    let Ok(value) = String::from_utf8(value_bytes.to_vec()) else {
        return;
    };
    match parser_byte & 7u8 {
        u8_constants::ZERO => drop(file_storage::StdStorageOperationId::try_from(value)),
        1u8 => drop(file_storage::StdStorageRelativePath::try_from(
            std::path::PathBuf::from(value),
        )),
        2u8 => drop(pg_crud_common::CursorPayload::try_from(value)),
        3u8 => drop(pg_crud_common::SignedCursor::try_from(value)),
        4u8 => drop(pg_crud_common::SqlIdentifier::try_from(value)),
        5u8 => drop(server_runtime_http::HttpTraceParent::try_from(value)),
        6u8 => drop(server_runtime_http::HttpTraceState::try_from(value)),
        _ => {}
    }
});
