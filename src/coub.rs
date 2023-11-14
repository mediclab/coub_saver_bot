use serde_json::Value;

pub struct CoubClient {
    client: reqwest::Client,
}

impl CoubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_file_url(&self, url: String) -> Option<String> {
        match self.client.get(url).send().await {
            Ok(res) => {
                if let Ok(content) = res.json::<Value>().await {
                    return content["file_versions"]["share"]["default"]
                        .as_str()
                        .map(|r| r.to_string());
                }
            }
            Err(e) => {
                error!("Error occurred {:?}", e);
            }
        }

        None
    }
}
