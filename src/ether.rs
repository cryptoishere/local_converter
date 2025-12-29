use alloy_primitives::Address;
use std::str::FromStr;

pub struct EtherAddress;

impl EtherAddress {
    pub fn is_valid_eth_address(addr: &str) -> bool {
        Address::from_str(addr).is_ok()
    }

    pub fn is_strict_checksum(addr: &str) -> bool {
        if let Ok(parsed) = Address::from_str(addr) {
            let checksummed = parsed.to_checksum(None);
            addr == checksummed
        } else {
            false
        }
    }
}
