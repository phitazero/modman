use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_urlencoded;

const USER_AGENT: &str = "phitazero/modman";

async fn async_get(url: &str, params: Vec<(&str, &str)>) -> String {
	let client = reqwest::Client::new();

	let mut headers = HeaderMap::new();
	headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

	let serialized_params = serde_urlencoded::to_string(params).unwrap();
	let full_url = format!("{}?{}", url, serialized_params);

	let response = client
		.get(full_url)
		.headers(headers)
		.send()
		.await
		.unwrap();

	response.text().await.unwrap()
}

pub fn sync_get(url: &str, params: Vec<(&str, &str)>) -> String {
	let runtime = tokio::runtime::Runtime::new().unwrap();

	runtime.block_on(async_get(url, params))
}
