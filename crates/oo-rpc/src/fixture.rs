// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/fixture.rs
// Purpose : Record and replay RPC observations without hidden network calls.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Record and replay RPC observations without hidden network calls.
//!
//! An experiment must be repeatable by another engineer, and a repetition that
//! silently reaches the network is not a repetition: the chain has moved, and a
//! difference between the two runs cannot be attributed to the thing under
//! study. Recorded fixtures make a run reproducible from stored material alone.
//!
//! Fixtures are keyed by the question, not by the request id. The id is chosen
//! by the caller and carries no meaning, so keying by it would let two
//! different questions collide and answer each other.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use oo_utils::{fs as utils_fs, Digest};
use serde::{Deserialize, Serialize};

use crate::{
    endpoint::RpcEndpoint,
    error::{RpcError, RpcResult},
    request::RpcRequest,
    response::RpcResponse,
    transport::RpcTransport,
};

/// A recorded question and the answer an endpoint gave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixtureRecord {
    /// Endpoint that answered.
    pub endpoint: String,
    /// Method that was asked.
    pub method: String,
    /// Parameters that were asked.
    pub params: serde_json::Value,
    /// Response that was returned.
    pub response: RpcResponse,
}

impl FixtureRecord {
    /// Returns the key identifying the question this record answers.
    #[must_use]
    pub fn key(&self) -> String {
        fixture_key(&self.method, &self.params)
    }
}

/// Returns the canonical key for a method and its parameters.
///
/// Parameters are serialized through `serde_json`, whose object keys are
/// ordered, so the same question always produces the same key.
#[must_use]
pub fn fixture_key(method: &str, params: &serde_json::Value) -> String {
    let canonical = serde_json::to_string(params).unwrap_or_else(|_| "null".to_owned());
    Digest::of_str_parts([method, canonical.as_str()])
        .short(32)
        .to_owned()
}

/// In-memory replay transport.
#[derive(Debug, Clone, Default)]
pub struct FixtureTransport {
    records: BTreeMap<String, FixtureRecord>,
}

impl FixtureTransport {
    /// Creates an empty fixture transport.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a response for a method and its parameters.
    pub fn insert_for(
        &mut self,
        method: impl Into<String>,
        params: serde_json::Value,
        response: RpcResponse,
    ) {
        let record = FixtureRecord {
            endpoint: String::new(),
            method: method.into(),
            params,
            response,
        };
        self.records.insert(record.key(), record);
    }

    /// Registers a recorded observation.
    pub fn insert_record(&mut self, record: FixtureRecord) {
        self.records.insert(record.key(), record);
    }

    /// Returns the number of recorded answers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the fixture set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Loads every fixture in a directory.
    ///
    /// A directory that does not exist yields an empty set; a file that cannot
    /// be parsed is an error, because a silently skipped fixture would turn a
    /// replay into a live call.
    pub fn load_directory(directory: impl AsRef<Path>) -> RpcResult<Self> {
        let directory = directory.as_ref();
        if !directory.is_dir() {
            return Ok(Self::new());
        }

        let files = utils_fs::list_files(directory).map_err(|error| RpcError::FixtureStore {
            path: directory.display().to_string(),
            message: error.to_string(),
        })?;

        let mut transport = Self::new();
        for file in files {
            if file.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let raw = utils_fs::read_to_string(&file).map_err(|error| RpcError::FixtureStore {
                path: file.display().to_string(),
                message: error.to_string(),
            })?;
            let record: FixtureRecord =
                serde_json::from_str(&raw).map_err(|error| RpcError::FixtureStore {
                    path: file.display().to_string(),
                    message: error.to_string(),
                })?;
            transport.insert_record(record);
        }
        Ok(transport)
    }

    /// Writes every fixture into a directory, one file per question.
    pub fn save_directory(&self, directory: impl AsRef<Path>) -> RpcResult<Vec<PathBuf>> {
        let directory = directory.as_ref();
        let mut written = Vec::with_capacity(self.records.len());
        for (key, record) in &self.records {
            let path = directory.join(format!("{key}.json"));
            let body =
                serde_json::to_string_pretty(record).map_err(|error| RpcError::FixtureStore {
                    path: path.display().to_string(),
                    message: error.to_string(),
                })?;
            utils_fs::write_atomic_str(&path, format!("{body}\n")).map_err(|error| {
                RpcError::FixtureStore {
                    path: path.display().to_string(),
                    message: error.to_string(),
                }
            })?;
            written.push(path);
        }
        Ok(written)
    }
}

#[async_trait]
impl RpcTransport for FixtureTransport {
    async fn send(&self, _endpoint: &RpcEndpoint, request: &RpcRequest) -> RpcResult<RpcResponse> {
        let key = fixture_key(request.method(), request.params());
        self.records
            .get(&key)
            .map(|record| {
                // The recorded answer adopts the caller's request id, so a
                // replay is indistinguishable from the original exchange.
                record.response.clone().with_id(request.id())
            })
            .ok_or_else(|| RpcError::MissingFixture {
                method: request.method().to_owned(),
                params: serde_json::to_string(request.params())
                    .unwrap_or_else(|_| "null".to_owned()),
            })
    }
}

/// A transport that answers from an inner transport and records what it saw.
///
/// Recording is how a live run becomes a reproducible one: the observations are
/// captured once, then replayed by anyone repeating the experiment.
#[derive(Debug)]
pub struct RecordingTransport<T> {
    inner: T,
    records: std::sync::Mutex<BTreeMap<String, FixtureRecord>>,
}

impl<T> RecordingTransport<T> {
    /// Wraps a transport in a recorder.
    #[must_use]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            records: std::sync::Mutex::new(BTreeMap::new()),
        }
    }

    /// Returns the fixtures recorded so far.
    #[must_use]
    pub fn recorded(&self) -> FixtureTransport {
        let records = self
            .records
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        FixtureTransport {
            records: records.clone(),
        }
    }

    /// Writes the recorded fixtures into a directory.
    pub fn save_directory(&self, directory: impl AsRef<Path>) -> RpcResult<Vec<PathBuf>> {
        self.recorded().save_directory(directory)
    }
}

#[async_trait]
impl<T> RpcTransport for RecordingTransport<T>
where
    T: RpcTransport,
{
    async fn send(&self, endpoint: &RpcEndpoint, request: &RpcRequest) -> RpcResult<RpcResponse> {
        let response = self.inner.send(endpoint, request).await?;
        let record = FixtureRecord {
            endpoint: endpoint.url().to_string(),
            method: request.method().to_owned(),
            params: request.params().clone(),
            response: response.clone(),
        };
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        records.insert(record.key(), record);
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::*;

    fn endpoint() -> RpcEndpoint {
        RpcEndpoint::parse("https://rpc.example.invalid").expect("endpoint")
    }

    fn scratch(name: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("oo-rpc-fixture-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("scratch directory");
        path
    }

    #[tokio::test]
    async fn a_fixture_answers_the_question_it_recorded() {
        let mut transport = FixtureTransport::new();
        transport.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(0, json!("0x1")),
        );

        let request = RpcRequest::new(42, "eth_chainId", json!([])).expect("request");
        let response = transport
            .send(&endpoint(), &request)
            .await
            .expect("response");

        assert_eq!(
            response.id(),
            42,
            "the replay adopts the caller's request id"
        );
        assert_eq!(response.into_result().expect("result"), json!("0x1"));
    }

    #[tokio::test]
    async fn different_questions_do_not_answer_each_other() {
        // Both requests carry id 1; keying by id would make the second return
        // the first one's answer.
        let mut transport = FixtureTransport::new();
        transport.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(1, json!("0x1")),
        );

        let other = RpcRequest::new(1, "eth_blockNumber", json!([])).expect("request");
        let error = transport
            .send(&endpoint(), &other)
            .await
            .expect_err("must fail");
        assert!(error.to_string().contains("eth_blockNumber"), "{error}");
    }

    #[tokio::test]
    async fn the_same_method_with_different_parameters_is_a_different_question() {
        let mut transport = FixtureTransport::new();
        transport.insert_for(
            "eth_getBalance",
            json!(["0xaaa", "0x1"]),
            RpcResponse::success(1, json!("0x64")),
        );

        let same = RpcRequest::new(1, "eth_getBalance", json!(["0xaaa", "0x1"])).expect("request");
        assert!(transport.send(&endpoint(), &same).await.is_ok());

        let other = RpcRequest::new(1, "eth_getBalance", json!(["0xbbb", "0x1"])).expect("request");
        assert!(transport.send(&endpoint(), &other).await.is_err());
    }

    #[test]
    fn a_key_is_stable_across_equivalent_json_orderings() {
        let first = fixture_key("eth_call", &json!([{"to": "0xabc", "data": "0x01"}]));
        let second = fixture_key("eth_call", &json!([{"data": "0x01", "to": "0xabc"}]));
        assert_eq!(first, second);
    }

    #[tokio::test]
    async fn recording_then_replaying_reproduces_the_run_without_the_network() {
        let mut live = FixtureTransport::new();
        live.insert_for(
            "eth_chainId",
            json!([]),
            RpcResponse::success(0, json!("0x1")),
        );

        let recorder = RecordingTransport::new(live);
        let request = RpcRequest::new(7, "eth_chainId", json!([])).expect("request");
        recorder
            .send(&endpoint(), &request)
            .await
            .expect("live call");

        let replay = recorder.recorded();
        assert_eq!(replay.len(), 1);
        let response = replay.send(&endpoint(), &request).await.expect("replay");
        assert_eq!(response.into_result().expect("result"), json!("0x1"));
    }

    #[tokio::test]
    async fn fixtures_survive_a_round_trip_through_a_directory() {
        let directory = scratch("round-trip");
        let mut transport = FixtureTransport::new();
        transport.insert_for(
            "eth_getCode",
            json!(["0xdac17f958d2ee523a2206206994597c13d831ec7", "0x1220000"]),
            RpcResponse::success(0, json!("0x60806040")),
        );

        let written = transport.save_directory(&directory).expect("save");
        assert_eq!(written.len(), 1);

        let loaded = FixtureTransport::load_directory(&directory).expect("load");
        let request = RpcRequest::new(
            3,
            "eth_getCode",
            json!(["0xdac17f958d2ee523a2206206994597c13d831ec7", "0x1220000"]),
        )
        .expect("request");
        let response = loaded.send(&endpoint(), &request).await.expect("replay");
        assert_eq!(response.into_result().expect("result"), json!("0x60806040"));

        let _ = fs::remove_dir_all(&directory);
    }

    #[test]
    fn a_missing_fixture_directory_yields_an_empty_set_rather_than_an_error() {
        let transport = FixtureTransport::load_directory("/nonexistent/origin-observer/fixtures")
            .expect("load");
        assert!(transport.is_empty());
    }

    #[test]
    fn an_unparsable_fixture_file_is_an_error() {
        let directory = scratch("unparsable");
        fs::write(directory.join("broken.json"), "{ not json").expect("write");
        let error = FixtureTransport::load_directory(&directory).expect_err("must fail");
        assert!(error.to_string().contains("broken.json"), "{error}");
        let _ = fs::remove_dir_all(&directory);
    }
}
