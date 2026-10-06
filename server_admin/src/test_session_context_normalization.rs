#[test]
fn test_session_context_user_agent_normalization_preserves_limits_and_fallbacks() {
    let hash_context = |http_header_value| {
        let mut headers = http::HeaderMap::new();
        if let Some(header_value) = http_header_value {
            let _previous_user_agent = headers.insert(http::header::USER_AGENT, header_value);
        }
        crate::authorization_session_context_hash::authorization_session_context_hash(
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
            crate::admin_peer_addr::AdminPeerAddr::from(
                server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                    std::net::SocketAddr::from(([192u8, 0u8, 2u8, 10u8], 443u16)),
                ),
            ),
        )
    };
    let cases = [
        (None, Some(constants_str::EMPTY.to_owned()), true),
        (None, Some(constants_str::SPACE.repeat(3usize)), true),
        (
            None,
            Some(constants_str::UNKNOWN_USER_AGENT.to_owned()),
            true,
        ),
        (None, Some(constants_str::NON_ASCII_U_E9.to_owned()), true),
        (None, Some(constants_str::X.repeat(8_193usize)), true),
        (None, Some(constants_str::X.repeat(8_192usize)), false),
        (
            Some(constants_str::X.repeat(256usize)),
            Some(constants_str::X.repeat(8_192usize)),
            true,
        ),
        (
            Some(constants_str::X.repeat(256usize)),
            Some(constants_str::X.repeat(255usize)),
            false,
        ),
        (
            Some(constants_str::X.repeat(256usize)),
            Some(format!(
                "{}{}",
                constants_str::X.repeat(256usize),
                constants_str::ADMIN_CLIENT_1,
            )),
            true,
        ),
        (
            Some(constants_str::X.repeat(256usize)),
            Some(format!(
                "{}{}{}",
                constants_str::SPACE,
                constants_str::X.repeat(8_192usize),
                constants_str::SPACE,
            )),
            true,
        ),
    ];
    assert!(cases.into_iter().all(|(first, second, equal)| {
        let first_header = first
            .map(|user_agent| http::HeaderValue::from_bytes(user_agent.as_bytes()))
            .transpose();
        let second_header = second
            .map(|user_agent| http::HeaderValue::from_bytes(user_agent.as_bytes()))
            .transpose();
        first_header.is_ok_and(|first_header| {
            second_header.is_ok_and(|second_header| {
                hash_context(first_header).is_ok_and(|first_hash| {
                    hash_context(second_header).is_ok_and(|second_hash| {
                        (first_hash.expose().as_ref() == second_hash.expose().as_ref()) == equal
                    })
                })
            })
        })
    }));
}
