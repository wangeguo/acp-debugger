// Copyright (c) wangeguo. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    thread::JoinHandle,
};

use futures::{channel::mpsc, StreamExt};
use serde_json::Value;

use crate::models::AgentConfig;

use super::error::ClientError;

/// A message received from the agent's stdout.
#[derive(Debug)]
pub enum TransportMessage {
    /// A successfully parsed JSON-RPC message.
    Json(Value),
    /// The stdout stream has closed (process exited).
    Closed,
    /// A line that failed to parse as JSON.
    ParseError { line: String, error: String },
}

/// Handle for a running agent subprocess with stdio transport.
///
/// Manages the child process and two I/O threads (reader + writer)
/// that communicate via channels.
pub struct StdioTransport {
    child: Child,
    outgoing_tx: mpsc::UnboundedSender<String>,
    incoming_rx: Option<mpsc::UnboundedReceiver<TransportMessage>>,
    _writer_handle: JoinHandle<()>,
    _reader_handle: JoinHandle<()>,
}

impl StdioTransport {
    /// Spawn the agent subprocess and start I/O threads.
    pub fn spawn(config: &AgentConfig) -> Result<Self, ClientError> {
        let mut cmd = Command::new(&config.endpoint);
        cmd.args(&config.args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());

        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        let mut child = cmd.spawn().map_err(ClientError::SpawnFailed)?;

        let stdin = child.stdin.take().expect("stdin was piped");
        let stdout = child.stdout.take().expect("stdout was piped");

        // Outgoing channel: main thread → writer thread → child stdin
        let (outgoing_tx, outgoing_rx) = mpsc::unbounded::<String>();

        // Incoming channel: child stdout → reader thread → main thread
        let (incoming_tx, incoming_rx) = mpsc::unbounded::<TransportMessage>();

        let writer_handle = std::thread::spawn(move || {
            writer_loop(stdin, outgoing_rx);
        });

        let reader_handle = std::thread::spawn(move || {
            reader_loop(stdout, incoming_tx);
        });

        Ok(Self {
            child,
            outgoing_tx,
            incoming_rx: Some(incoming_rx),
            _writer_handle: writer_handle,
            _reader_handle: reader_handle,
        })
    }

    /// Take the incoming message receiver (can only be called once).
    pub fn take_incoming(&mut self) -> Option<mpsc::UnboundedReceiver<TransportMessage>> {
        self.incoming_rx.take()
    }

    /// Send a JSON string to the agent's stdin (non-blocking).
    pub fn send(&self, json_line: String) -> Result<(), ClientError> {
        self.outgoing_tx.unbounded_send(json_line).map_err(|_| ClientError::TransportClosed)
    }

    /// Kill the child process and clean up.
    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }
}

/// Writer thread: consumes outgoing messages and writes to stdin.
fn writer_loop(mut stdin: std::process::ChildStdin, mut rx: mpsc::UnboundedReceiver<String>) {
    futures::executor::block_on(async {
        while let Some(line) = rx.next().await {
            if writeln!(stdin, "{}", line).is_err() {
                break;
            }
            if stdin.flush().is_err() {
                break;
            }
        }
    });
}

/// Reader thread: reads lines from stdout and sends parsed messages.
fn reader_loop(stdout: std::process::ChildStdout, tx: mpsc::UnboundedSender<TransportMessage>) {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => {
                let msg = match serde_json::from_str::<Value>(&line) {
                    Ok(value) => TransportMessage::Json(value),
                    Err(e) => TransportMessage::ParseError { line, error: e.to_string() },
                };
                if tx.unbounded_send(msg).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    let _ = tx.unbounded_send(TransportMessage::Closed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_message_variants() {
        let json_msg = TransportMessage::Json(serde_json::json!({"jsonrpc": "2.0"}));
        assert!(matches!(json_msg, TransportMessage::Json(_)));

        let closed = TransportMessage::Closed;
        assert!(matches!(closed, TransportMessage::Closed));

        let err = TransportMessage::ParseError {
            line: "bad json".into(),
            error: "expected value".into(),
        };
        assert!(matches!(err, TransportMessage::ParseError { .. }));
    }
}
