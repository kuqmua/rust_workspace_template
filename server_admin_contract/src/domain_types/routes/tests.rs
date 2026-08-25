#[test]
fn sign_in_route_keeps_public_authentication_and_path() {
    let metadata =
        <super::AdminSignInRoute as frontend_contract::domain_types::TypedRoute>::metadata();
    assert_eq!(
        metadata.authentication(),
        frontend_contract::domain_types::AuthenticationRequirement::Public
    );
    assert_eq!(metadata.path().as_ref(), "/auth/sign_in");
}
