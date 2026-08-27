#[path = "outbound_url/outbound_address_disposition.rs"]
mod outbound_address_disposition;
#[path = "outbound_url/outbound_allowed_host.rs"]
mod outbound_allowed_host;
#[path = "outbound_url/outbound_host_allowlist.rs"]
mod outbound_host_allowlist;
#[path = "outbound_url/outbound_host_allowlist_error.rs"]
mod outbound_host_allowlist_error;
#[path = "outbound_url/outbound_host_policy.rs"]
mod outbound_host_policy;
#[path = "outbound_url/outbound_ip_addr.rs"]
mod outbound_ip_addr;
#[path = "outbound_url/outbound_url_error.rs"]
mod outbound_url_error;
#[path = "outbound_url/outbound_url_policy.rs"]
mod outbound_url_policy;
#[path = "outbound_url/outbound_url_scheme.rs"]
mod outbound_url_scheme;
#[path = "outbound_url/outbound_url_text_ref.rs"]
mod outbound_url_text_ref;
#[path = "outbound_url/reqwest_outbound_url.rs"]
mod reqwest_outbound_url;

use outbound_address_disposition::{OutboundAddressDisposition, outbound_address_disposition};
pub use outbound_allowed_host::OutboundAllowedHost;
pub use outbound_host_allowlist::OutboundHostAllowlist;
pub use outbound_host_allowlist_error::OutboundHostAllowlistError;
pub use outbound_host_policy::OutboundHostPolicy;
pub use outbound_ip_addr::OutboundIpAddr;
pub use outbound_url_error::OutboundUrlError;
pub use outbound_url_policy::OutboundUrlPolicy;
pub use outbound_url_scheme::OutboundUrlScheme;
pub use outbound_url_text_ref::OutboundUrlTextRef;
pub use reqwest_outbound_url::ReqwestOutboundUrl;

#[cfg(test)]
mod tests {
    const POLICY: super::OutboundUrlPolicy = super::OutboundUrlPolicy::new(
        &[
            super::OutboundUrlScheme::Http,
            super::OutboundUrlScheme::Https,
        ],
        super::OutboundHostPolicy::RejectPrivate,
    );

    #[test]
    fn public_url_and_address_are_accepted() {
        let url = POLICY
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect("a275c7bf public_url_and_address_are_accepted invariant must hold");
        assert_eq!(url.scheme(), super::OutboundUrlScheme::Https);
        assert_eq!(
            POLICY.validate_resolved_addresses(&[std::net::IpAddr::V4(std::net::Ipv4Addr::new(
                8u8, 8u8, 8u8, 8u8
            ))
            .into(),]),
            Ok(())
        );
    }

    #[test]
    fn local_literal_hostname_and_encoded_control_are_rejected() {
        assert!(matches!(
            POLICY.validate(constants_str::HTTP_LOCALHOST.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_LOOPBACK_HTTP_URL.into()),
            Err(super::OutboundUrlError::ForbiddenHost)
        ));
        assert!(matches!(
            POLICY.validate(constants_str::TEST_URL_WITH_ENCODED_NEWLINE.into()),
            Err(super::OutboundUrlError::ControlCharacter)
        ));
    }

    #[test]
    fn non_global_special_addresses_are_rejected() {
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
                    Err(super::OutboundUrlError::ForbiddenHost)
                )
            })
        );
    }

    #[test]
    fn allowlist_requires_exact_host_and_url_rejects_userinfo() {
        let allowed_host = super::OutboundAllowedHost::try_from(String::from(
            constants_str::TEST_PUBLIC_HOST,
        ))
        .expect(
            "3e5decb1 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold",
        );
        let allowlist = super::OutboundHostAllowlist::try_from(vec![allowed_host]).expect(
            "920be78f allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold",
        );
        let allowed = POLICY
            .validate(constants_str::TEST_PUBLIC_HTTPS_URL.into())
            .expect("27a67a96 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold");
        assert_eq!(allowlist.validate(&allowed), Ok(()));
        let other = POLICY
            .validate(constants_str::TEST_OTHER_PUBLIC_HTTPS_URL.into())
            .expect("b3981504 allowlist_requires_exact_host_and_url_rejects_userinfo invariant must hold");
        assert_eq!(
            allowlist.validate(&other),
            Err(super::OutboundHostAllowlistError::HostNotAllowed)
        );
        assert!(matches!(
            POLICY.validate(constants_str::TEST_PUBLIC_HTTPS_URL_WITH_USERINFO.into()),
            Err(super::OutboundUrlError::UserInfo)
        ));
    }
}
