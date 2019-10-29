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
pub struct ClusterAllocationExplain {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterAllocationExplain {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterAllocationExplain {
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
    #[doc = "Return information about disk usage and shard sizes (default: false)"]
    pub fn include_disk_info(mut self, include_disk_info: Option<bool>) -> Self {
        self.include_disk_info = include_disk_info;
        self
    }
    #[doc = "Return 'YES' decisions in explanation (default: false)"]
    pub fn include_yes_decisions(mut self, include_yes_decisions: Option<bool>) -> Self {
        self.include_yes_decisions = include_yes_decisions;
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
impl Sender for ClusterAllocationExplain {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterGetSettings {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterGetSettings {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterGetSettings {
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether to return all default clusters setting."]
    pub fn include_defaults(mut self, include_defaults: Option<bool>) -> Self {
        self.include_defaults = include_defaults;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for ClusterGetSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterHealth {
    client: Elasticsearch,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    level: Option<Level>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
    wait_for_active_shards: Option<String>,
    wait_for_events: Option<WaitForEvents>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<String>,
    wait_for_status: Option<WaitForStatus>,
}
impl ClusterHealth {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterHealth {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
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
    #[doc = "Specify the level of detail for returned information"]
    pub fn level(mut self, level: Option<Level>) -> Self {
        self.level = level;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
    #[doc = "Wait until the specified number of shards is active"]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: Option<String>) -> Self {
        self.wait_for_active_shards = wait_for_active_shards;
        self
    }
    #[doc = "Wait until all currently queued events with the given priority are processed"]
    pub fn wait_for_events(mut self, wait_for_events: Option<WaitForEvents>) -> Self {
        self.wait_for_events = wait_for_events;
        self
    }
    #[doc = "Whether to wait until there are no initializing shards in the cluster"]
    pub fn wait_for_no_initializing_shards(
        mut self,
        wait_for_no_initializing_shards: Option<bool>,
    ) -> Self {
        self.wait_for_no_initializing_shards = wait_for_no_initializing_shards;
        self
    }
    #[doc = "Whether to wait until there are no relocating shards in the cluster"]
    pub fn wait_for_no_relocating_shards(
        mut self,
        wait_for_no_relocating_shards: Option<bool>,
    ) -> Self {
        self.wait_for_no_relocating_shards = wait_for_no_relocating_shards;
        self
    }
    #[doc = "Wait until the specified number of nodes is available"]
    pub fn wait_for_nodes(mut self, wait_for_nodes: Option<String>) -> Self {
        self.wait_for_nodes = wait_for_nodes;
        self
    }
    #[doc = "Wait until cluster is in a specific state"]
    pub fn wait_for_status(mut self, wait_for_status: Option<WaitForStatus>) -> Self {
        self.wait_for_status = wait_for_status;
        self
    }
}
impl Sender for ClusterHealth {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterPendingTasks {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterPendingTasks {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPendingTasks {
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
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
impl Sender for ClusterPendingTasks {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterPutSettings {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterPutSettings {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterPutSettings {
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for ClusterPutSettings {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterRemoteInfo {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
}
impl ClusterRemoteInfo {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterRemoteInfo {
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
impl Sender for ClusterRemoteInfo {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterReroute {
    client: Elasticsearch,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    master_timeout: Option<String>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    retry_failed: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterReroute {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterReroute {
            client,
            ..Default::default()
        }
    }
    #[doc = "Simulate the operation only and return the resulting state"]
    pub fn dry_run(mut self, dry_run: Option<bool>) -> Self {
        self.dry_run = dry_run;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Return an explanation of why the commands can or cannot be executed"]
    pub fn explain(mut self, explain: Option<bool>) -> Self {
        self.explain = explain;
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Limit the information returned to the specified metrics. Defaults to all but metadata"]
    pub fn metric(mut self, metric: Option<Vec<String>>) -> Self {
        self.metric = metric;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Retries allocation of shards that are blocked due to too many subsequent allocation failures"]
    pub fn retry_failed(mut self, retry_failed: Option<bool>) -> Self {
        self.retry_failed = retry_failed;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for ClusterReroute {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterState {
    client: Elasticsearch,
    allow_no_indices: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<ExpandWildcards>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    metric: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<String>,
}
impl ClusterState {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterState {
            client,
            ..Default::default()
        }
    }
    #[doc = "Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified)"]
    pub fn allow_no_indices(mut self, allow_no_indices: Option<bool>) -> Self {
        self.allow_no_indices = allow_no_indices;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: Option<ExpandWildcards>) -> Self {
        self.expand_wildcards = expand_wildcards;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Whether specified concrete indices should be ignored when unavailable (missing or closed)"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Specify timeout for connection to master"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
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
    #[doc = "Wait for the metadata version to be equal or greater than the specified metadata version"]
    pub fn wait_for_metadata_version(mut self, wait_for_metadata_version: Option<i64>) -> Self {
        self.wait_for_metadata_version = wait_for_metadata_version;
        self
    }
    #[doc = "The maximum time to wait for wait_for_metadata_version before timing out"]
    pub fn wait_for_timeout(mut self, wait_for_timeout: Option<String>) -> Self {
        self.wait_for_timeout = wait_for_timeout;
        self
    }
}
impl Sender for ClusterState {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[derive(Default)]
pub struct ClusterStats {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    flat_settings: Option<bool>,
    human: Option<bool>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    source: Option<String>,
    timeout: Option<String>,
}
impl ClusterStats {
    pub fn new(client: Elasticsearch) -> Self {
        ClusterStats {
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
    #[doc = "Return settings in flat format (default: false)"]
    pub fn flat_settings(mut self, flat_settings: Option<bool>) -> Self {
        self.flat_settings = flat_settings;
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: Option<String>) -> Self {
        self.timeout = timeout;
        self
    }
}
impl Sender for ClusterStats {
    fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let response = self.client.send::<()>(HttpMethod::Post, "/", None, None)?;
        Ok(response)
    }
}
#[doc = "Cluster APIs"]
pub struct Cluster {
    client: Elasticsearch,
}
impl Cluster {
    pub fn new(client: Elasticsearch) -> Self {
        Cluster { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-allocation-explain.html"]
    pub fn allocation_explain(&self) -> ClusterAllocationExplain {
        ClusterAllocationExplain::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn get_settings(&self) -> ClusterGetSettings {
        ClusterGetSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-health.html"]
    pub fn health(&self) -> ClusterHealth {
        ClusterHealth::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-pending.html"]
    pub fn pending_tasks(&self) -> ClusterPendingTasks {
        ClusterPendingTasks::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html"]
    pub fn put_settings(&self) -> ClusterPutSettings {
        ClusterPutSettings::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-remote-info.html"]
    pub fn remote_info(&self) -> ClusterRemoteInfo {
        ClusterRemoteInfo::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-reroute.html"]
    pub fn reroute(&self) -> ClusterReroute {
        ClusterReroute::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html"]
    pub fn state(&self) -> ClusterState {
        ClusterState::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-stats.html"]
    pub fn stats(&self) -> ClusterStats {
        ClusterStats::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.clone())
    }
}