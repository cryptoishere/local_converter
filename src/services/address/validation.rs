use crate::traits::address::AddressUnit;

pub struct AddressService<F, T>
where
    F: AddressUnit,
    T: AddressUnit,
{
    from: F,
    to: T,
}

impl<F, T> AddressService<F, T>
where
    F: AddressUnit,
    T: AddressUnit,
{
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self {
            from: F::new(from),
            to: T::new(to),
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if !self.from.validate()? {
            anyhow::bail!("Invalid from address");
        }

        if !self.to.validate()? {
            anyhow::bail!("Invalid to address");
        }

        Ok(())
    }
}

impl<F, T> AddressService<F, T>
where
    F: AddressUnit,
    T: AddressUnit,
{
    pub fn get_from(&self) -> &str {
        self.from.get()
    }

    pub fn get_to(&self) -> &str {
        self.to.get()
    }

    pub fn from(&self) -> anyhow::Result<F::Address> {
        self.from.get_as_type()
    }

    pub fn to(&self) -> anyhow::Result<T::Address> {
        self.to.get_as_type()
    }
}
