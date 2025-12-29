use anyhow::anyhow;
use alloy_primitives::hex::FromHexError;
use alloy_primitives::Address;

use std::str::FromStr;

use crate::ether::EtherAddress;
use crate::services::address::directions::{From, To};
use crate::traits::address::AddressUnit;

mod directions;

pub struct AddressService<'a> {
    from: Box<dyn AddressUnit + 'a>,
    to:  Box<dyn AddressUnit + 'a>,
}

impl<'a> AddressService<'a> {
    pub fn new(
        from:  &'a str,
        to:  &'a str,
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

    pub fn strict_validate(&self) -> anyhow::Result<Address, FromHexError> {
        let addr = match EtherAddress::is_strict_checksum(self.to.get()) {
            true => {
                Address::from_str(self.to.get())
            }
            false => {
                let addr = Address::from_str(self.to.get())?;

                if EtherAddress::is_strict_checksum(&addr.to_string()) {
                    return Err(FromHexError::InvalidStringLength);
                }

                Ok(addr)
            }
        }?;

        Ok(addr)
    }

    pub fn get_from(&self) -> &str {
        self.from.get()
    }

    pub fn get_to(&self) -> &str {
        self.to.get()
    }
}
