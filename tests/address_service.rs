use crypto_converter::{ether::EtherAddress, services::address::AddressService};

const CHECKSUMMED: &str =
    "0x413e8c21Dd266ea5f4e7EeBcd18498A66eC8dAC7";

const NON_CHECKSUMMED: &str =
    "0x413e8c21dd266ea5f4e7eebcd18498a66ec8dac7";

#[test]
fn valid_eth_address_accepts_both() {
    assert!(EtherAddress::is_valid_eth_address(CHECKSUMMED));
    assert!(EtherAddress::is_valid_eth_address(NON_CHECKSUMMED));
}

#[test]
fn strict_checksum_detection() {
    assert!(EtherAddress::is_strict_checksum(CHECKSUMMED));
    assert!(!EtherAddress::is_strict_checksum(NON_CHECKSUMMED));
}

#[test]
fn address_service_validate_passes() {
    let svc = AddressService::new(
        CHECKSUMMED.to_string(),
        NON_CHECKSUMMED.to_string(),
    );

    assert!(svc.validate().is_ok());
}

#[test]
fn strict_validate_accepts_only_checksummed() {
    let ok = AddressService::new(
        CHECKSUMMED.to_string(),
        CHECKSUMMED.to_string(),
    );

    assert!(ok.strict_validate().is_ok());

    let bad = AddressService::new(
        CHECKSUMMED.to_string(),
        NON_CHECKSUMMED.to_string(),
    );

    assert!(bad.strict_validate().is_err());
}
