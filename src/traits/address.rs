pub(crate) trait AddressUnit: Send + Sync {
    fn validate(&self) -> anyhow::Result<bool>;

    fn get(&self) -> &str;
}
