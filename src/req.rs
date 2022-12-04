use crate::cli::RunArgs;
use crate::utils;
use anyhow::Result;
use reqwest::{Method, Response};
use std::str::FromStr;

#[derive(Debug)]
pub struct RequestExt {
    pub url: String,
    pub method: Method,
}

#[derive(Debug)]
pub struct ResponseExt {
    pub head: String,
    pub path: String,
}

impl RequestExt {
    pub fn new(url: &str, method: &str) -> Result<Self> {
        if !utils::is_validated_method(&method) {
            return Err(anyhow::anyhow!("Invalid method: {}", method));
        }
        let method = Method::from_str(&method)?;
        Ok(Self {
            url: url.to_string(),
            method,
        })
    }
    /// Send the request
    pub async fn send(&self) -> Result<ResponseExt> {
        let client = reqwest::Client::new();
        let request_builder = client.request(self.method.clone(), &self.url);
        let response = request_builder.send().await?;

        let request_builder = client.request(self.method.clone(), &self.url);
        let content = request_builder.send().await?.text().await?;

        println!("Response content: {}", content);

        Ok(response.into())
    }
}

impl TryFrom<&RunArgs> for RequestExt {
    type Error = anyhow::Error;
    fn try_from(args: &RunArgs) -> Result<Self> {
        Self::new(&args.url, &args.method)
    }
}

impl From<Response> for ResponseExt {
    fn from(resp: Response) -> Self {
        Self {
            path: resp.url().path().to_string(),
            // like HTTP/1.1 200 OK
            head: format!("{:?} {}", resp.version(), resp.status(),),
        }
    }
}
