use std::time::Duration;

use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

use crate::{client::Client, core::session::URLPart};

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    #[serde(rename = "accountId")]
    account_id: String,

    #[serde(rename = "blobId")]
    blob_id: String,

    #[serde(rename = "type")]
    type_: String,

    #[serde(rename = "size")]
    size: usize,
}

impl Client {
    pub async fn upload(
        &self,
        account_id: &str,
        content_type: Option<&str>,
        blob: Vec<u8>,
    ) -> crate::Result<UploadResponse> {
        let mut upload_url =
            String::with_capacity(self.session().upload_url().len() + account_id.len());

        for part in self.upload_url() {
            match part {
                URLPart::Value(value) => {
                    upload_url.push_str(value);
                }
                URLPart::Parameter(param) => {
                    if let super::URLParameter::AccountId = param {
                        upload_url.push_str(account_id);
                    }
                }
            }
        }

        serde_json::from_slice::<UploadResponse>(
            &Client::handle_error(
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(self.timeout()))
                    .default_headers(self.headers().clone())
                    .build()?
                    .post(upload_url)
                    .header(
                        CONTENT_TYPE,
                        content_type.unwrap_or("application/octet-stream"),
                    )
                    .body(blob)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )
        .map_err(|err| err.into())
    }
}

impl UploadResponse {
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn blob_id(&self) -> &str {
        &self.blob_id
    }

    pub fn content_type(&self) -> &str {
        &self.type_
    }

    pub fn size(&self) -> usize {
        self.size
    }
}