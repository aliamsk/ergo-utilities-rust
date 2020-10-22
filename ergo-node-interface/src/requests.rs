use crate::node_interface::{NodeError, NodeInterface, Result};
use json::JsonValue;
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{HeaderValue, CONTENT_TYPE};

impl NodeInterface {
    /// Builds a `HeaderValue` to use for requests with the api key specified
    pub fn get_node_api_header(&self) -> HeaderValue {
        match HeaderValue::from_str(&self.api_key) {
            Ok(k) => k,
            _ => HeaderValue::from_static("None"),
        }
    }

    /// Sets required headers for a request
    pub fn set_req_headers(&self, rb: RequestBuilder) -> RequestBuilder {
        rb.header("accept", "application/json")
            .header("api_key", self.get_node_api_header())
            .header(CONTENT_TYPE, "application/json")
    }

    /// Sends a GET request to the Ergo node
    pub fn send_get_req(&self, endpoint: &str) -> Result<Response> {
        let url = self.node_url().to_owned() + endpoint;
        let client = reqwest::blocking::Client::new().get(&url);
        self.set_req_headers(client)
            .send()
            .map_err(|_| NodeError::NodeUnreachable)
    }

    /// Sends a POST request to the Ergo node
    pub fn send_post_req(&self, endpoint: &str, body: String) -> Result<Response> {
        let url = self.node_url().to_owned() + endpoint;
        let client = reqwest::blocking::Client::new().post(&url);
        self.set_req_headers(client)
            .body(body)
            .send()
            .map_err(|_| NodeError::NodeUnreachable)
    }

    /// Parses response from node into JSON
    pub fn parse_response_to_json(&self, resp: Result<Response>) -> Result<JsonValue> {
        let text = resp?.text().map_err(|_| {
            NodeError::FailedParsingNodeResponse(
                "Node Response Not Parseable into Text.".to_string(),
            )
        })?;
        let json = json::parse(&text).map_err(|_| NodeError::FailedParsingNodeResponse(text))?;
        Ok(json)
    }
}
