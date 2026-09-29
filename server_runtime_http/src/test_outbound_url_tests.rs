#[cfg(test)]
mod tests {
    const POLICY: crate::outbound_url_policy::OutboundUrlPolicy =
        crate::outbound_url_policy::OutboundUrlPolicy::new(
            &[
                crate::outbound_url_scheme::OutboundUrlScheme::Http,
                crate::outbound_url_scheme::OutboundUrlScheme::Https,
            ],
            crate::outbound_host_policy::OutboundHostPolicy::RejectPrivate,
        );

    #[test]
    fn test_public_url_and_address_are_accepted() {
        let url = POLICY
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect(constants_str::DIAGNOSTIC_A275C7BF);
        assert_eq!(
            url.scheme(),
            crate::outbound_url_scheme::OutboundUrlScheme::Https
        );
        assert_eq!(
            POLICY.validate_resolved_addresses(&[std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                8u8, 8u8, 8u8, 8u8
            ))
            .into(),]),
            Ok(())
        );
    }

    #[test]
    fn test_local_literal_hostname_and_encoded_control_are_rejected() {
        assert!(matches!(
            POLICY.validate(constants_str::HTTP_LOCALHOST.into()),
            Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_LOOPBACK_HTTP_URL.into()),
            Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_URL_WITH_ENCODED_NEWLINE.into()),
            Err(crate::outbound_url_error::OutboundUrlError::ControlCharacter)
        ));
    }

    #[test]
    fn test_private_ipv6_url_is_rejected() {
        assert!(matches!(
            POLICY.validate(constants_str::TEST_LOOPBACK_IPV6_HTTP_URL.into()),
            Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
        ));
    }

    #[test]
    fn test_non_global_special_addresses_are_rejected() {
        assert!(
            [
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    100u8,
                    64u8,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(192u8, constants_u8::ZERO, 2u8, 1u8)),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    198u8,
                    18u8,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(198u8, 51u8, 100u8, 1u8)),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    203u8,
                    constants_u8::ZERO,
                    113u8,
                    1u8
                )),
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                    240u8,
                    constants_u8::ZERO,
                    constants_u8::ZERO,
                    1u8
                )),
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x2001u16,
                    0x0db8u16,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    constants_u16::ZERO,
                    1u16,
                )),
            ]
            .into_iter()
            .all(|address| {
                matches!(
                    POLICY.validate_resolved_addresses(&[address.into()]),
                    Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
                )
            })
        );
    }

    #[test]
    fn test_outbound_ipv6_rejects_non_global_special_prefixes() {
        assert!(
            [
                [0x64u16, 0xff9b, 1, 0, 0, 0, 0, 1],
                [0x64, 0xff9b, 0, 0, 0, 0, 0x7f00, 1],
                [0x64, 0xff9b, 0, 0, 0, 0, 0xa9fe, 0xa9fe],
                [0x100, 0, 0, 0, 0, 0, 0, 1],
                [0x100, 0, 0, 1, 0, 0, 0, 1],
                [0x2001, 1, 0, 0, 0, 0, 0, 4],
                [0x2001, 2, 0, 0, 0, 0, 0, 1],
                [0x2001, 0x10, 0, 0, 0, 0, 0, 1],
                [
                    0x2001, 0x1ff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
                ],
                [0x3fff, 0, 0, 0, 0, 0, 0, 1],
                [
                    0x3fff, 0xfff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff
                ],
                [0x5f00, 0xffff, 0, 0, 0, 0, 0, 1],
                [0xfec0, 0, 0, 0, 0, 0, 0, 1],
                [0xfeff, 0xffff, 0, 0, 0, 0, 0, 1],
            ]
            .into_iter()
            .all(|segments| matches!(
                POLICY.validate_resolved_addresses(&[std::net::IpAddr::V6(
                    std::net::Ipv6Addr::from(segments)
                )
                .into()]),
                Err(crate::outbound_url_error::OutboundUrlError::ForbiddenHost)
            ))
        );
    }

    #[test]
    fn test_outbound_ipv6_preserves_allowed_special_and_adjacent_prefixes() {
        assert!(
            [
                [0x64u16, 0xff9b, 0, 0, 0, 0, 0x808, 0x808],
                [0x100, 0, 0, 2, 0, 0, 0, 1],
                [0x2001, 0, 0, 0, 0, 0, 0, 1],
                [0x2001, 1, 0, 0, 0, 0, 0, 1],
                [0x2001, 1, 0, 0, 0, 0, 0, 2],
                [0x2001, 1, 0, 0, 0, 0, 0, 3],
                [0x2001, 3, 0, 0, 0, 0, 0, 1],
                [0x2001, 4, 0x112, 0, 0, 0, 0, 1],
                [0x2001, 0x20, 0, 0, 0, 0, 0, 1],
                [0x2001, 0x2f, 0xffff, 0, 0, 0, 0, 1],
                [0x2001, 0x30, 0, 0, 0, 0, 0, 1],
                [0x2001, 0x3f, 0xffff, 0, 0, 0, 0, 1],
                [0x2001, 0x200, 0, 0, 0, 0, 0, 1],
                [0x2001, 0x4860, 0, 0, 0, 0, 0, 0x8888],
                [0x2002, 0x808, 0x808, 0, 0, 0, 0, 1],
                [0x3fff, 0x1000, 0, 0, 0, 0, 0, 1],
                [0x5f01, 0, 0, 0, 0, 0, 0, 1],
            ]
            .into_iter()
            .all(|segments| POLICY
                .validate_resolved_addresses(&[std::net::IpAddr::V6(std::net::Ipv6Addr::from(
                    segments
                ))
                .into()])
                .is_ok())
        );
    }

    #[test]
    fn test_allow_private_policy_accepts_special_ipv6_addresses() {
        let policy = crate::outbound_url_policy::OutboundUrlPolicy::new(
            &[
                crate::outbound_url_scheme::OutboundUrlScheme::Http,
                crate::outbound_url_scheme::OutboundUrlScheme::Https,
            ],
            crate::outbound_host_policy::OutboundHostPolicy::AllowPrivate,
        );
        assert_eq!(
            policy.validate_resolved_addresses(&[
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    0x64u16, 0xff9b, 0, 0, 0, 0, 0x7f00, 1,
                ))
                .into(),
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(0xfec0u16, 0, 0, 0, 0, 0, 0, 1,))
                    .into(),
            ]),
            Ok(())
        );
    }

    #[test]
    fn test_allowlist_requires_exact_host_and_url_rejects_userinfo() {
        let allowed_host = crate::outbound_allowed_host::OutboundAllowedHost::try_from(
            String::from(constants_str::TEST_PUBLIC_HOST),
        )
        .expect(constants_str::DIAGNOSTIC_3E5DECB1);
        let allowlist =
            crate::outbound_host_allowlist::OutboundHostAllowlist::try_from(vec![allowed_host])
                .expect(constants_str::DIAGNOSTIC_920BE78F);
        let allowed = POLICY
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect(constants_str::DIAGNOSTIC_27A67A96);
        assert_eq!(allowlist.validate(&allowed), Ok(()));
        let other = POLICY
            .validate(constants_str::TEST_OTHER_PUBLIC_HTTPS_URL.into())
            .expect(constants_str::DIAGNOSTIC_B3981504);
        assert_eq!(
            allowlist.validate(&other),
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::HostNotAllowed)
        );
        assert!(matches!(
            POLICY.validate(constants_str::TEST_PUBLIC_HTTPS_URL_WITH_USERINFO.into()),
            Err(crate::outbound_url_error::OutboundUrlError::UserInfo)
        ));
    }
    #[test]
    fn test_allowlist_rejects_host_with_port() {
        let host_with_port = format!("{}:{}", constants_str::TEST_PUBLIC_HOST, 443u16);
        assert_eq!(
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(host_with_port),
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost)
        );
        let ipv6_host = format!("[{}]", std::net::Ipv6Addr::LOCALHOST);
        assert_eq!(
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(ipv6_host.clone())
                .map(|_host| ()),
            Ok(())
        );
        assert_eq!(
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(format!(
                "{ipv6_host}:{}",
                443u16
            )),
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost)
        );
        let mut expanded = std::net::Ipv6Addr::LOCALHOST.segments().into_iter().fold(
            String::from('['),
            |mut text, segment| {
                if text.len() > constants_usize::ONE {
                    text.push(':');
                }
                text.push_str(segment.to_string().as_str());
                text
            },
        );
        expanded.push(']');
        assert_eq!(
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(expanded)
                .map(|host| host.as_str().to_owned()),
            Ok(ipv6_host)
        );
    }
    #[test]
    fn test_allowlist_rejects_numeric_host_alias() {
        let address = std::net::Ipv4Addr::new(8u8, 8u8, 8u8, 8u8);
        let mut alias = String::from('0');
        alias.push('x');
        alias.push_str(format!("{:x}", u32::from(address)).as_str());
        assert_eq!(
            crate::outbound_allowed_host::OutboundAllowedHost::try_from(alias),
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost)
        );
    }
}
