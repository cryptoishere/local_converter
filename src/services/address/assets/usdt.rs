use alloy_primitives::Address;

use crate::ether::EtherAddress;
use crate::traits::address::AddressUnit;

pub struct USDT {
    path: String,
}

impl AddressUnit for USDT {
    type Address = Address;

    fn new(addr: impl Into<String>) -> Self {
        Self {
            path: addr.into(),
        }
    }

    fn validate(&self) -> anyhow::Result<bool> {
        Ok(EtherAddress::is_valid_eth_address(&self.path))
    }

    fn get(&self) -> &str {
        &self.path
    }

    fn get_as_type(&self) -> anyhow::Result<Self::Address> {
        EtherAddress::to_address(&self.path)
    }
}
