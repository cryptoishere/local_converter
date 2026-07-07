pub trait AddressUnit: Send + Sync + Sized {
    type Address;

    fn new(addr: impl Into<String>) -> Self;

    fn validate(&self) -> anyhow::Result<bool>;

    fn get(&self) -> &str;

    fn get_as_type(&self) -> anyhow::Result<Self::Address>;
}
