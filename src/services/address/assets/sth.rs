use crate::sth::SthAddress;
use crate::traits::address::AddressUnit;

use arkecosystem_crypto::enums::Network;
use arkecosystem_crypto::identities::address;

pub struct STH {
    path: String,
}

impl AddressUnit for STH {
    type Address = SthAddress;

    fn new(addr: impl Into<String>) -> Self {
        Self {
            path: addr.into(),
        }
    }

    fn validate(&self) -> anyhow::Result<bool> {
        let is_valid = address::validate(
            &self.path,
            Some(Network::Mainnet.version())
        );

        is_valid
    }

    fn get(&self) -> &str {
        &self.path
    }

    fn get_as_type(&self) -> anyhow::Result<Self::Address> {
        let addr: SthAddress =
            self.path.parse()?;

        Ok(addr)
    }
}
