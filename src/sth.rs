use bs58;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SthAddress {
    version: u8,
    hash: [u8; 20],
}

impl SthAddress {
    pub fn new(version: u8, hash: [u8; 20]) -> Self {
        Self { version, hash }
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn hash(&self) -> &[u8; 20] {
        &self.hash
    }

    pub fn validate(value: &str, expected_version: u8) -> anyhow::Result<()> {
        let bytes = bs58::decode(value)
            .with_check(None)
            .into_vec()?;

        if bytes.len() != 21 {
            anyhow::bail!("Invalid STH address length");
        }

        if bytes[0] != expected_version {
            anyhow::bail!("Invalid network");
        }

        Ok(())
    }
}

impl FromStr for SthAddress {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(value)
            .with_check(None)
            .into_vec()?;

        if bytes.len() != 21 {
            anyhow::bail!("Invalid address");
        }

        let version = bytes[0];

        let hash: [u8; 20] = bytes[1..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash"))?;

        Ok(Self {
            version,
            hash,
        })
    }
}

impl fmt::Display for SthAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = vec![self.version];
        bytes.extend_from_slice(&self.hash);

        let encoded = bs58::encode(bytes)
            .with_check()
            .into_string();

        f.write_str(&encoded)
    }
}
