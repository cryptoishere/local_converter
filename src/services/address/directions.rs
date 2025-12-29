use crate::ether::EtherAddress;
use crate::traits::address::AddressUnit;

#[cfg(feature = "smartholdem")]
use arkecosystem_crypto::enums::Network;
#[cfg(feature = "smartholdem")]
use arkecosystem_crypto::identities::address;

pub struct From<'ba> {
    path: &'ba str,
}

impl<'ba> From<'ba> {
    pub fn new(addr: &'ba str) -> Self {
        Self {
            path: addr,
        }
    }
}

impl<'ba> AddressUnit for From<'ba> {
    fn validate(&self) -> anyhow::Result<bool> {
        #[cfg(feature = "smartholdem")]
        let is_valid = address::validate(self.path, Some(Network::Mainnet.version()));

        #[cfg(not(feature = "smartholdem"))]
        let is_valid = Ok(EtherAddress::is_strict_checksum(self.path));

        is_valid
    }

    fn get(&self) -> &str {
        self.path
    }
}

pub struct To<'ba> {
    path: &'ba str,
}

impl<'ba> To<'ba> {
    pub fn new(addr: &'ba str) -> Self {
        Self {
            path: addr,
        }
    }
}

impl<'ba> AddressUnit for To<'ba> {
    fn validate(&self) -> anyhow::Result<bool> {
        Ok(EtherAddress::is_valid_eth_address(self.path))
    }

    fn get(&self) -> &str {
        self.path
    }
}
