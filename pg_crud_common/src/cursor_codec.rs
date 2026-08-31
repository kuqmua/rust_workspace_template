#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct CursorCodec {
    key: crate::cursor_signing_key::CursorSigningKey,
    maximum_length: crate::cursor_maximum_length::CursorMaximumLength,
}

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
impl CursorCodec {
    #[must_use]
    pub const fn new(
        key: crate::cursor_signing_key::CursorSigningKey,
        maximum_length: crate::cursor_maximum_length::CursorMaximumLength,
    ) -> Self {
        Self {
            key,
            maximum_length,
        }
    }

    pub fn encode(
        &self,
        payload: &crate::cursor_payload::CursorPayload,
    ) -> Result<crate::signed_cursor::SignedCursor, crate::cursor_encode_error::CursorEncodeError>
    {
        let encoded_payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            payload.as_ref().as_bytes(),
        );
        let signed_text = format!("{}.{encoded_payload}", constants_str::CURSOR_VERSION_V1);
        let mut mac = <hmac::Hmac<sha2::Sha256> as hmac::KeyInit>::new_from_slice(
            self.key.get_inner().as_slice(),
        )
        .map_err(|_error| crate::cursor_encode_error::CursorEncodeError::InvalidSigningKey)?;
        hmac::Mac::update(&mut mac, signed_text.as_bytes());
        let encoded_signature = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            hmac::Mac::finalize(mac).into_bytes(),
        );
        let cursor_text = format!("{signed_text}.{encoded_signature}");
        if cursor_text.len() > self.maximum_length.get_inner().get() {
            return Err(crate::cursor_encode_error::CursorEncodeError::MaximumLengthExceeded);
        }
        crate::signed_cursor::SignedCursor::try_from(cursor_text)
            .map_err(|_error| crate::cursor_encode_error::CursorEncodeError::MaximumLengthExceeded)
    }

    pub fn decode(
        &self,
        cursor: &crate::signed_cursor::SignedCursor,
    ) -> Result<crate::cursor_payload::CursorPayload, crate::cursor_decode_error::CursorDecodeError>
    {
        if cursor.as_ref().len() > self.maximum_length.get_inner().get() {
            return Err(crate::cursor_decode_error::CursorDecodeError::MaximumLengthExceeded);
        }
        let mut parts = cursor.as_ref().split('.');
        let version = parts
            .next()
            .ok_or(crate::cursor_decode_error::CursorDecodeError::InvalidFormat)?;
        let encoded_payload = parts
            .next()
            .ok_or(crate::cursor_decode_error::CursorDecodeError::InvalidFormat)?;
        let encoded_signature = parts
            .next()
            .ok_or(crate::cursor_decode_error::CursorDecodeError::InvalidFormat)?;
        if parts.next().is_some() || version != constants_str::CURSOR_VERSION_V1 {
            return Err(crate::cursor_decode_error::CursorDecodeError::InvalidFormat);
        }
        let signature = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded_signature,
        )
        .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidSignature)?;
        let mut mac = <hmac::Hmac<sha2::Sha256> as hmac::KeyInit>::new_from_slice(
            self.key.get_inner().as_slice(),
        )
        .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidSigningKey)?;
        hmac::Mac::update(&mut mac, format!("{version}.{encoded_payload}").as_bytes());
        hmac::Mac::verify_slice(mac, signature.as_slice())
            .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidSignature)?;
        let payload_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            encoded_payload,
        )
        .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidPayload)?;
        let payload_text = String::from_utf8(payload_bytes)
            .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidPayload)?;
        crate::cursor_payload::CursorPayload::try_from(payload_text)
            .map_err(|_error| crate::cursor_decode_error::CursorDecodeError::InvalidPayload)
    }
}

#[cfg(test)]
mod tests {
    proptest::proptest! {
        #![proptest_config(proptest::test_runner::Config {
            cases: 64u32,
            rng_seed: proptest::test_runner::RngSeed::Fixed(0x5a17_c0deu64),
            ..proptest::test_runner::Config::default()
        })]

        #[test]
        #[cfg_attr(miri, ignore = constants_str::VALUE_BF7C931C)]
        fn signed_cursor_round_trips_generated_payloads(payload_text in constants_str::TEST_CURSOR_PAYLOAD_PATTERN) {
            let domain_payload = crate::cursor_payload::CursorPayload::try_from(payload_text).expect(constants_str::VALUE_28167829);
            let cursor = codec().encode(&domain_payload).expect(constants_str::VALUE_58718EC8);
            proptest::prop_assert_eq!(codec().decode(&cursor), Ok(domain_payload));
        }

        #[test]
        #[cfg_attr(miri, ignore = constants_str::VALUE_BF7C931C)]
        fn changing_signature_is_always_rejected(payload_text in constants_str::TEST_CURSOR_PAYLOAD_PATTERN) {
            let domain_payload = crate::cursor_payload::CursorPayload::try_from(payload_text).expect(constants_str::VALUE_52BB899A);
            let cursor = codec().encode(&domain_payload).expect(constants_str::VALUE_5E1A9245);
            let mut modified_bytes = cursor.as_ref().as_bytes().to_vec();
            let signature_start = modified_bytes.iter().rposition(|byte| *byte == b'.').and_then(|index| index.checked_add(constants_usize::ONE)).expect(constants_str::VALUE_02A18550);
            let signature_byte = modified_bytes.get_mut(signature_start).expect(constants_str::VALUE_EB8B9918);
            *signature_byte = if *signature_byte == b'A' { b'B' } else { b'A' };
            let modified_text = String::from_utf8(modified_bytes).expect(constants_str::VALUE_130A34B8);
            let modified_cursor = crate::signed_cursor::SignedCursor::try_from(modified_text).expect(constants_str::VALUE_D1169A2F);
            proptest::prop_assert_eq!(codec().decode(&modified_cursor), Err(crate::cursor_decode_error::CursorDecodeError::InvalidSignature));
        }
    }

    fn codec() -> crate::cursor_codec::CursorCodec {
        crate::cursor_codec::CursorCodec::new(
            crate::cursor_signing_key::CursorSigningKey::try_from(vec![7u8; 32usize])
                .expect("556f25ae codec invariant must hold"),
            crate::cursor_maximum_length::CursorMaximumLength::try_from(1_024usize)
                .expect("30c8f351 codec invariant must hold"),
        )
    }

    #[test]
    fn signed_cursor_round_trip_preserves_payload() {
        let payload = crate::cursor_payload::CursorPayload::try_from(String::from(
            constants_str::CURSOR_TEST_JSON_PAYLOAD,
        ))
        .expect("ead70a9e signed_cursor_round_trip_preserves_payload invariant must hold");
        let cursor = codec()
            .encode(&payload)
            .expect("47ad934b signed_cursor_round_trip_preserves_payload invariant must hold");
        assert_eq!(
            codec()
                .decode(&cursor)
                .expect("cc4bf589 signed_cursor_round_trip_preserves_payload invariant must hold"),
            payload
        );
    }

    #[test]
    fn modified_cursor_is_rejected() {
        let payload = crate::cursor_payload::CursorPayload::try_from(String::from(
            constants_str::CURSOR_TEST_PAYLOAD,
        ))
        .expect("256860a7 modified_cursor_is_rejected invariant must hold");
        let cursor = codec()
            .encode(&payload)
            .expect("22fc1ce9 modified_cursor_is_rejected invariant must hold");
        let modified =
            crate::signed_cursor::SignedCursor::try_from(format!("{}x", cursor.as_ref()))
                .expect("64b5f541 modified_cursor_is_rejected invariant must hold");
        assert_eq!(
            codec().decode(&modified),
            Err(crate::cursor_decode_error::CursorDecodeError::InvalidSignature)
        );
    }
}
