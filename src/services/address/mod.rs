use anyhow::anyhow;
use alloy_primitives::Address;

use std::str::FromStr;

use crate::ether::EtherAddress;
use crate::services::address::directions::{From, To};
use crate::traits::address::AddressUnit;

mod directions;

pub struct AddressService {
    from: Box<dyn AddressUnit>,
    to:  Box<dyn AddressUnit>,
}

impl AddressService {
    pub fn new(
        from:  String,
        to:  String,
    ) -> Self {
        Self {
            from: Box::new(From::new(from)),
            to: Box::new(To::new(to)),
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        match self.from.validate() {
            Ok(is_valid) => {
                match is_valid {
                    true => {}
                    false => return Err(anyhow!("Invalid from address")),
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed validate from address: {e}"));
            }
        };

        match self.to.validate() {
            Ok(is_valid) => {
                match is_valid {
                    true => {}
                    false => return Err(anyhow!("Invalid to address")),
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed validate to address: {e}"));
            }
        };

        Ok(())
    }

    pub fn strict_validate(&self) -> anyhow::Result<Address> {
        let raw = self.to.get();

        let addr = Address::from_str(raw)
            .map_err(|e| anyhow!("Invalid ETH address: {e}"))?;

        if !EtherAddress::is_strict_checksum(raw) {
            return Err(anyhow!("Address is not EIP-55 checksummed"));
        }

        Ok(addr)
    }

    pub fn get_from(&self) -> &str {
        self.from.get()
    }

    pub fn get_to(&self) -> &str {
        self.to.get()
    }
}
