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
}
