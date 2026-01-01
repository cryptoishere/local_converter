use crate::ether::EtherAddress;
use crate::traits::address::AddressUnit;

#[cfg(feature = "smartholdem")]
use arkecosystem_crypto::enums::Network;
#[cfg(feature = "smartholdem")]
use arkecosystem_crypto::identities::address;

pub struct From {
    path: String,
}

impl From {
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            path: addr.into(),
        }
    }
}

impl AddressUnit for From {
    fn validate(&self) -> anyhow::Result<bool> {
        #[cfg(feature = "smartholdem")]
        let is_valid = address::validate(&self.path, Some(Network::Mainnet.version()));

        #[cfg(not(feature = "smartholdem"))]
        let is_valid = Ok(EtherAddress::is_strict_checksum(&self.path));

        is_valid
    }

    fn get(&self) -> &str {
        &self.path
    }
}

pub struct To {
    path: String,
}

impl To {
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            path: addr.into(),
        }
    }
}

impl AddressUnit for To {
    fn validate(&self) -> anyhow::Result<bool> {
        Ok(EtherAddress::is_valid_eth_address(&self.path))
    }

    fn get(&self) -> &str {
        &self.path
    }
}
