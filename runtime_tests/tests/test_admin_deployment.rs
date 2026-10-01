#![allow(
    unused_crate_dependencies,
    reason = "this integration target uses the HTTP client and public contracts while other package dependencies serve the runtime binary"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires a provisioned application server; launched by browser acceptance deployment setup"]
    fn test_public_admin_deployment_endpoints() {
        let address = std::env::var(constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS)
            .expect(constants_str::DIAGNOSTIC_0102D093);
        let base_url = runtime_tests::service_base_url::ServiceBaseUrl::try_from(format!(
            "{}{address}",
            constants_str::VALUE_8C8DAC95
        ))
        .expect(constants_str::DIAGNOSTIC_2FC594DB);
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect(constants_str::DIAGNOSTIC_6B10A8A9);
        assert!(
            [
                common_routes::common_route::CommonRoute::HealthLive,
                common_routes::common_route::CommonRoute::HealthReady
            ]
            .into_iter()
            .all(|route| {
                client
                    .get(format!(
                        "{}{}",
                        base_url.as_ref(),
                        route.contract().path().as_ref()
                    ))
                    .send()
                    .expect(constants_str::DIAGNOSTIC_3F6C6637)
                    .status()
                    .as_u16()
                    == 200u16
            })
        );
        let branding_response = client
            .get(format!(
                "{}/{}/{}",
                base_url.as_ref(),
                constants_str::ADMIN_UI_BRANDING,
                stringify!(read)
            ))
            .send()
            .expect(constants_str::DIAGNOSTIC_AEE527FD);
        assert_eq!(branding_response.status().as_u16(), 200u16);
        let branding = branding_response
            .json::<std::collections::BTreeMap<String, Option<String>>>()
            .expect(constants_str::DIAGNOSTIC_04936E98);
        assert_eq!(
            branding
                .get(stringify!(default_admin_route))
                .and_then(Option::as_deref),
            Some(constants_str::VALUE_074B6E5E)
        );
        assert!(
            branding
                .get(stringify!(site_name))
                .is_some_and(Option::is_some)
        );
        let stylesheet = client
            .get(format!(
                "{}{}",
                base_url.as_ref(),
                constants_str::VALUE_688DB289
            ))
            .send()
            .expect(constants_str::DIAGNOSTIC_5B1652D3);
        assert_eq!(stylesheet.status().as_u16(), 200u16);
        assert!(
            stylesheet
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|header| header.to_str().ok())
                .is_some_and(|header| header.contains(constants_str::TEXT_CSS))
        );
        assert_eq!(
            stylesheet
                .headers()
                .get(constants_str::X_CONTENT_TYPE_OPTIONS)
                .and_then(|header| header.to_str().ok()),
            Some(constants_str::NOSNIFF)
        );
        let mut bytes = Vec::new();
        let _read_size = std::io::Read::read_to_end(
            &mut std::io::Read::take(stylesheet, 1_048_577u64),
            &mut bytes,
        )
        .expect(constants_str::DIAGNOSTIC_51625562);
        assert!(bytes.len() > 1_000usize && bytes.len() <= 1_048_576usize);
    }
}
