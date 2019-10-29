// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::error::ElasticsearchError;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct IlmDeleteLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmDeleteLifecycle {
    pub fn new(client: Elasticsearch, policy: String) -> Self {
        IlmDeleteLifecycle {
            client,
            policy: policy,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmDeleteLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmExplainLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmExplainLifecycle {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmExplainLifecycle {
            client,
            index: index,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmExplainLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmGetLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetLifecycle {
    pub fn new(client: Elasticsearch) -> Self {
        IlmGetLifecycle {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmGetLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmGetStatus {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmGetStatus {
    pub fn new(client: Elasticsearch) -> Self {
        IlmGetStatus {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmGetStatus {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmMoveToStep {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmMoveToStep {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmMoveToStep {
            client,
            index: index,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmMoveToStep {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmPutLifecycle {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    policy: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmPutLifecycle {
    pub fn new(client: Elasticsearch, policy: String) -> Self {
        IlmPutLifecycle {
            client,
            policy: policy,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmPutLifecycle {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmRemovePolicy {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmRemovePolicy {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmRemovePolicy {
            client,
            index: index,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmRemovePolicy {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmRetry {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: String,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmRetry {
    pub fn new(client: Elasticsearch, index: String) -> Self {
        IlmRetry {
            client,
            index: index,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmRetry {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmStart {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmStart {
    pub fn new(client: Elasticsearch) -> Self {
        IlmStart {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmStart {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct IlmStop {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl IlmStop {
    pub fn new(client: Elasticsearch) -> Self {
        IlmStop {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for IlmStop {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[doc = "Ilm APIs"]
pub struct Ilm {
    client: Elasticsearch,
}
impl Ilm {
    pub fn new(client: Elasticsearch) -> Self {
        Ilm { client }
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-delete-lifecycle.html"]
    pub fn delete_lifecycle(&self, policy: String) -> IlmDeleteLifecycle {
        IlmDeleteLifecycle::new(self.client.clone(), policy)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-explain-lifecycle.html"]
    pub fn explain_lifecycle(&self, index: String) -> IlmExplainLifecycle {
        IlmExplainLifecycle::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-lifecycle.html"]
    pub fn get_lifecycle(&self) -> IlmGetLifecycle {
        IlmGetLifecycle::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-get-status.html"]
    pub fn get_status(&self) -> IlmGetStatus {
        IlmGetStatus::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-move-to-step.html"]
    pub fn move_to_step(&self, index: String) -> IlmMoveToStep {
        IlmMoveToStep::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-put-lifecycle.html"]
    pub fn put_lifecycle(&self, policy: String) -> IlmPutLifecycle {
        IlmPutLifecycle::new(self.client.clone(), policy)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-remove-policy.html"]
    pub fn remove_policy(&self, index: String) -> IlmRemovePolicy {
        IlmRemovePolicy::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-retry-policy.html"]
    pub fn retry(&self, index: String) -> IlmRetry {
        IlmRetry::new(self.client.clone(), index)
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-start.html"]
    pub fn start(&self) -> IlmStart {
        IlmStart::new(self.client.clone())
    }
    #[doc = "https://www.elastic.co/guide/en/elasticsearch/reference/current/ilm-stop.html"]
    pub fn stop(&self) -> IlmStop {
        IlmStop::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Ilm APIs"]
    pub fn ilm(&self) -> Ilm {
        Ilm::new(self.clone())
    }
}