// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_derive::Serialize;
use std::path;

#[derive(Debug, Serialize, Clone)]
pub struct AdminConfig {
    pub db_key: String,

    pub key_id: u64,

    pub db_path: String,

    pub admin_address: String,
}

impl AdminConfig {
    pub fn write_to_file(&self, path: impl AsRef<path::Path>) {
        crate::util::write_to_file(&self, path, "kms_sm".to_string());
    }
}
