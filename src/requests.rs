use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_urlencoded;

const USER_AGENT: &str = "phitazero/modman";

pub fn sync_get(url: &str, params: Vec<(&str, &str)>) -> String {
	let client = reqwest::blocking::Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

	let serialized_params = serde_urlencoded::to_string(params).unwrap();
	let full_url = format!("{}?{}", url, serialized_params);

	let response = client
		.get(full_url)
		.headers(headers)
		.send()
		.unwrap();

	response.text().unwrap()
}
