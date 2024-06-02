pub fn get_filename_from_url(url: &str) -> String {
    let url_clean = url.split(&['?', '#'][..]).next().unwrap();
    url_clean.split('/').last().unwrap().to_string()
}
