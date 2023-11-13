use serde_json::Value;

pub struct CoubClient {
    client: reqwest::Client
}

impl CoubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_file_url(&self, url: String) -> Option<String> {
        if let Ok(response) = self.client.get(url).send().await {
            if let Ok(content) = response.json::<Value>().await {
                return content["file_versions"]["share"]["default"].as_str().map(|r| r.to_string())
            }
        }

        None
    }
}