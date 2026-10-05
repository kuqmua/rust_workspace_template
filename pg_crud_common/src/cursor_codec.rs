#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
pub struct CursorCodec {
    key: crate::cursor_signing_key::CursorSigningKey,
    maximum_length: crate::cursor_maximum_length::CursorMaximumLength,
}

#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "cursor codec keeps declaration order aligned with generated layout or processing flow"
)]
impl CursorCodec {
    pub fn encode(
        &self,
        cursor_payload: &crate::cursor_payload::CursorPayload,
    ) -> Result<crate::signed_cursor::SignedCursor, crate::cursor_encode_error::CursorEncodeError>
    {
        let encoded_payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            cursor_payload.as_ref().as_bytes(),
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
        signed_cursor: &crate::signed_cursor::SignedCursor,
    ) -> Result<crate::cursor_payload::CursorPayload, crate::cursor_decode_error::CursorDecodeError>
    {
        if signed_cursor.as_ref().len() > self.maximum_length.get_inner().get() {
            return Err(crate::cursor_decode_error::CursorDecodeError::MaximumLengthExceeded);
        }
        let mut parts = signed_cursor.as_ref().split('.');
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
        fn test_signed_cursor_round_trips_generated_payloads(payload_text in constants_str::TEST_CURSOR_PAYLOAD_PATTERN) {
            let domain_payload = crate::cursor_payload::CursorPayload::try_from(payload_text).expect(constants_str::VALUE_28167829);
            let cursor = codec().encode(&domain_payload).expect(constants_str::VALUE_58718EC8);
            proptest::prop_assert_eq!(codec().decode(&cursor), Ok(domain_payload));
        }

        #[test]
        #[cfg_attr(miri, ignore = constants_str::VALUE_BF7C931C)]
        fn test_changing_signature_is_always_rejected(payload_text in constants_str::TEST_CURSOR_PAYLOAD_PATTERN) {
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
                .expect(constants_str::DIAGNOSTIC_556F25AE),
            crate::cursor_maximum_length::CursorMaximumLength::try_from(1_024usize)
                .expect(constants_str::DIAGNOSTIC_30C8F351),
        )
    }

    #[test]
    fn test_signed_cursor_round_trip_preserves_payload() {
        let payload = crate::cursor_payload::CursorPayload::try_from(String::from(
            constants_str::CURSOR_TEST_JSON_PAYLOAD,
        ))
        .expect(constants_str::DIAGNOSTIC_EAD70A9E);
        let cursor = codec()
            .encode(&payload)
            .expect(constants_str::DIAGNOSTIC_47AD934B);
        assert_eq!(
            codec()
                .decode(&cursor)
                .expect(constants_str::DIAGNOSTIC_CC4BF589),
            payload
        );
    }

    #[test]
    fn test_modified_cursor_is_rejected() {
        let payload = crate::cursor_payload::CursorPayload::try_from(String::from(
            constants_str::CURSOR_TEST_PAYLOAD,
        ))
        .expect(constants_str::DIAGNOSTIC_256860A7);
        let cursor = codec()
            .encode(&payload)
            .expect(constants_str::DIAGNOSTIC_22FC1CE9);
        let modified =
            crate::signed_cursor::SignedCursor::try_from(format!("{}x", cursor.as_ref()))
                .expect(constants_str::DIAGNOSTIC_64B5F541);
        assert_eq!(
            codec().decode(&modified),
            Err(crate::cursor_decode_error::CursorDecodeError::InvalidSignature)
        );
    }

    #[test]
    fn test_cursor_decoding_distinguishes_malformed_structure_and_signatures() {
        [
            constants_str::X.to_owned(),
            [constants_str::CURSOR_VERSION_V1, constants_str::X].join(constants_str::DOT),
            [constants_str::X, constants_str::X, constants_str::X].join(constants_str::DOT),
            [
                constants_str::CURSOR_VERSION_V1,
                constants_str::X,
                constants_str::X,
                constants_str::X,
            ]
            .join(constants_str::DOT),
        ]
        .into_iter()
        .fold((), |(), text| {
            let cursor_result = crate::signed_cursor::SignedCursor::try_from(text);
            assert!(cursor_result.is_ok());
            let Ok(cursor) = cursor_result else {
                return;
            };
            assert_eq!(
                codec().decode(&cursor),
                Err(crate::cursor_decode_error::CursorDecodeError::InvalidFormat)
            );
        });
        [constants_str::X.to_owned(), '!'.to_string(), String::new()]
            .into_iter()
            .fold((), |(), signature| {
                let text = [
                    constants_str::CURSOR_VERSION_V1,
                    constants_str::X,
                    signature.as_str(),
                ]
                .join(constants_str::DOT);
                let cursor_result = crate::signed_cursor::SignedCursor::try_from(text);
                assert!(cursor_result.is_ok());
                let Ok(cursor) = cursor_result else {
                    return;
                };
                assert_eq!(
                    codec().decode(&cursor),
                    Err(crate::cursor_decode_error::CursorDecodeError::InvalidSignature)
                );
            });
    }

    #[test]
    fn test_authenticated_cursor_payload_rejects_invalid_base64_utf8_and_empty_text() {
        let signed_payload = |encoded_payload| {
            let signed_text =
                [constants_str::CURSOR_VERSION_V1, encoded_payload].join(constants_str::DOT);
            let mac_result =
                <hmac::Hmac<sha2::Sha256> as hmac::KeyInit>::new_from_slice(&[7u8; 32usize]);
            assert!(mac_result.is_ok());
            let Ok(mut mac) = mac_result else {
                return None;
            };
            hmac::Mac::update(&mut mac, signed_text.as_bytes());
            let signature = base64::Engine::encode(
                &base64::engine::general_purpose::URL_SAFE_NO_PAD,
                hmac::Mac::finalize(mac).into_bytes(),
            );
            let cursor_result = crate::signed_cursor::SignedCursor::try_from(
                [signed_text.as_str(), signature.as_str()].join(constants_str::DOT),
            );
            assert!(cursor_result.is_ok());
            cursor_result.ok()
        };
        let invalid_base64 = '!'.to_string();
        let invalid_utf8 =
            base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, [255u8]);
        [
            invalid_base64.as_str(),
            invalid_utf8.as_str(),
            constants_str::EMPTY,
        ]
        .into_iter()
        .fold((), |(), encoded_payload| {
            assert_eq!(
                signed_payload(encoded_payload).map(|cursor| codec().decode(&cursor)),
                Some(Err(
                    crate::cursor_decode_error::CursorDecodeError::InvalidPayload
                ))
            );
        });
    }

    #[test]
    fn test_cursor_length_limits_preserve_exact_boundary_and_storage_limit() {
        let codec_with_limit = |maximum_length| {
            let key_result =
                crate::cursor_signing_key::CursorSigningKey::try_from(vec![7u8; 32usize]);
            assert!(key_result.is_ok());
            let Ok(key) = key_result else {
                return None;
            };
            let maximum_result =
                crate::cursor_maximum_length::CursorMaximumLength::try_from(maximum_length);
            assert!(
                maximum_result
                    .as_ref()
                    .is_ok_and(|maximum| maximum.get_inner().get() == maximum_length)
            );
            let Ok(maximum) = maximum_result else {
                return None;
            };
            Some(crate::cursor_codec::CursorCodec::new(key, maximum))
        };
        let payload_result =
            crate::cursor_payload::CursorPayload::try_from(constants_str::X.to_owned());
        assert!(payload_result.is_ok());
        let Ok(payload) = payload_result else {
            return;
        };
        let cursor_result = codec().encode(&payload);
        assert!(cursor_result.is_ok());
        let Ok(cursor) = cursor_result else {
            return;
        };
        let encoded_length = cursor.as_ref().len();
        let exact_result = codec_with_limit(encoded_length);
        assert!(exact_result.is_some());
        let Some(exact) = exact_result else {
            return;
        };
        assert!(
            exact
                .encode(&payload)
                .is_ok_and(|encoded| encoded == cursor)
        );
        assert!(
            exact
                .decode(&cursor)
                .is_ok_and(|decoded| decoded == payload)
        );
        let smaller_result = codec_with_limit(encoded_length.saturating_sub(1usize));
        assert!(smaller_result.is_some());
        let Some(smaller) = smaller_result else {
            return;
        };
        assert_eq!(
            smaller.encode(&payload),
            Err(crate::cursor_encode_error::CursorEncodeError::MaximumLengthExceeded)
        );
        assert_eq!(
            smaller.decode(&cursor),
            Err(crate::cursor_decode_error::CursorDecodeError::MaximumLengthExceeded)
        );
        let larger_result = codec_with_limit(100_000usize);
        assert!(larger_result.is_some());
        let Some(larger) = larger_result else {
            return;
        };
        let large_payload_result =
            crate::cursor_payload::CursorPayload::try_from(constants_str::X.repeat(65_536usize));
        assert!(large_payload_result.is_ok());
        let Ok(large_payload) = large_payload_result else {
            return;
        };
        assert_eq!(
            larger.encode(&large_payload),
            Err(crate::cursor_encode_error::CursorEncodeError::MaximumLengthExceeded)
        );
    }
}
