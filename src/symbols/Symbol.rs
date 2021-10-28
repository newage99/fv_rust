pub trait Symbol: Sync {
    fn symbol(&self) -> &str;
}