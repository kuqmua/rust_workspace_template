#[test]
fn route_contract_tests() {
    let metadata = <super::AdminSignInRoute as frontend_contract::TypedRoute>::metadata();
    assert_eq!(
        metadata.authentication(),
        frontend_contract::AuthenticationRequirement::Public
    );
    assert_eq!(metadata.path().as_ref(), "/auth/sign_in");
}
