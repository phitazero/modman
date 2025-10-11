use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_urlencoded;

const USER_AGENT: &str = "phitazero/modman";

fn serialize_params(params: &Vec<(&str, &str)>) -> Result<String, String> {
	serde_urlencoded::to_string(params)
		.map_err(|_| format!("failed to serialize parameters: {:?}", params))
}

pub fn sync_get(url: &str, params: Vec<(&str, &str)>) -> Result<String, String> {
	let client = reqwest::blocking::Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

	let serialized_params = serialize_params(&params)?;
	let full_url = format!("{}?{}", url, serialized_params);

	let response = client
		.get(full_url)
		.headers(headers)
		.send()
		.map_err(|_| format!("GET request to {} failed", url))?;

	let status = response.status();

	if !status.is_success() {
		return Err(format!(
			"GET request to {} failed with status: {}",
			url, status
		));
	}

	response.text().map_err(|_| "failed to get response text".to_string())
}
