#[test]
fn test_session_context_ignores_source_port_for_ipv4_ipv6_and_mapped_peers() {
    let headers = http::HeaderMap::new();
    assert!(
        [
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            std::net::IpAddr::V6(std::net::Ipv4Addr::LOCALHOST.to_ipv6_mapped()),
        ]
        .into_iter()
        .all(|address| {
            let hash = |port| {
                crate::authorization_session_context_hash::authorization_session_context_hash(
                    crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                    crate::admin_peer_addr::AdminPeerAddr::from(
                        server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                            std::net::SocketAddr::new(address, port),
                        ),
                    ),
                )
            };
            hash(0u16).is_ok_and(|first| {
                hash(u16::MAX)
                    .is_ok_and(|second| first.expose().as_ref() == second.expose().as_ref())
            })
        })
    );
}

#[test]
fn test_session_context_uses_first_user_agent_even_when_later_value_is_valid() {
    assert!(
        http::HeaderValue::from_bytes(&[255u8]).is_ok_and(|nontext| {
            [
                http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_1),
                http::HeaderValue::from_static(constants_str::EMPTY),
                nontext,
            ]
            .into_iter()
            .all(|first_value| {
                let mut headers = http::HeaderMap::new();
                let _previous = headers.insert(http::header::USER_AGENT, first_value);
                let peer = crate::admin_peer_addr::AdminPeerAddr::from(
                    server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                        std::net::SocketAddr::from((std::net::Ipv6Addr::LOCALHOST, 443u16)),
                    ),
                );
                let first =
                    crate::authorization_session_context_hash::authorization_session_context_hash(
                        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                        peer,
                    );
                let _appended = headers.append(
                    http::header::USER_AGENT,
                    http::HeaderValue::from_static(constants_str::ADMIN_CLIENT_2),
                );
                first.is_ok_and(|original| {
                    crate::authorization_session_context_hash::authorization_session_context_hash(
                        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&headers),
                        peer,
                    )
                    .is_ok_and(|with_duplicate| {
                        original.expose().as_ref() == with_duplicate.expose().as_ref()
                    })
                })
            })
        })
    );
}

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
