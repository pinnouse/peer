pub trait NetClient {
    fn request(&self, url: &str, data: &[u8]) -> &[u8];
}