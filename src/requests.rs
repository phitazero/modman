use crate::utils;
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_urlencoded;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::fs::File;
use std::io::Write;

const USER_AGENT: &str = "phitazero/modman";

fn serialize_params(params: &Vec<(&str, &str)>) -> Result<String, String> {
	serde_urlencoded::to_string(params)
		.map_err(|_| format!("failed to serialize parameters: {:?}", params))
}

pub fn sync_get<T>(url: &str, params: Vec<(&str, &str)>) -> Result<T, String>
where T: DeserializeOwned {
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

	let text = response.text()
		.map_err(|_| "failed to get response text".to_string())?;

	serde_json::from_str(&text)
		.map_err(|err|
			format!("failed to parse JSON\n{}", utils::format_json_error(err))
		)
}

pub fn sync_download<P>(path: P, url: &str) -> Result<(), String>
where P: AsRef<Path> {
	let path = path.as_ref();

	let response = reqwest::blocking::get(url)
		.map_err(|_| format!("GET request to {} failed", url))?;

	let status = response.status();

	if !status.is_success() {
		return Err(format!(
			"GET request to {} failed with status: {}",
			url, status
		));
	}

	let bytes = response
		.bytes()
		.map_err(|_| String::from("couldn't read download data"))?;

	let mut file = File::create(path)
		.map_err(|_| String::from("couldn't create file"))?;

	file.write_all(&bytes)
		.map_err(|_| String::from("couln't write to file"))?;

	Ok(())
}
