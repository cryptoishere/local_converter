pub(crate) trait AddressUnit {
    fn validate(&self) -> anyhow::Result<bool>;

    fn get(&self) -> &str;
}
