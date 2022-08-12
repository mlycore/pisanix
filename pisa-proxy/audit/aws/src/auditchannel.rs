// Copyright 2022 SphereEx Authors
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

use crate::aws::{CloudWatchLog, CloudWatchSinker};

// pub trait AuditLog {
//     fn get_message(&self) -> String;
//     fn get_timestamp(&self) -> i64;
// }

pub struct AuditLog {
    timestamp: i64,
    message: String,
}

// impl AuditLog {
//     Set
// }

// pub trait AuditSinker {
//     pub fn async() -> Result<>;
// }

pub struct AuditSinker {
    pub cloudwatch_sinker: CloudWatchSinker,
    pub channel: AuditChannel,
}

impl AuditSinker {
    pub fn run(&self) {
        tokio::task::spawn_blocking(move || {
            // let ch = self.channel.clone();
            loop {
                let input = self.channel.audit_rx.recv().unwrap();
                let e = CloudWatchLog {
                    message: e.message,
                    timestamp: e.timestamp,
                    log_group_name: "".to_string(),
                    log_stream_name: "".to_string(),
                };
                self.cloudwatch_sinker.send(e);
                std::thread::sleep(std::time::Duration::from_millis(monitor_interval));
            }
        });
    }
}

pub struct AuditChannel {
    pub audit_tx: crossbeam_channel::Sender<AuditLog>,
    pub audit_rx: crossbeam_channel::Receiver<AuditLog>,
    // pub sender
}

impl AuditChannel {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        AuditChannel { audit_tx: tx, audit_rx: rx }
    }
}
