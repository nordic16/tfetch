pub trait Provider {
    pub fn get_name(name: &str);
    pub fn get_base_url(url: &str);
    pub fn fetch_results(query: &str);
}
