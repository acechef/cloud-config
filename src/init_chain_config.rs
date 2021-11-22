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

use crate::config::chain_config::ChainConfig;
use crate::error::Error;
use crate::util::write_toml;
use clap::Clap;

/// A subcommand for run
#[derive(Clap, Debug, Clone)]
pub struct InitChainConfigOpts {
    /// set chain name
    #[clap(long = "chain-name", default_value = "test-chain")]
    chain_name: String,
    /// set config file directory, default means current directory
    #[clap(long = "config-dir", default_value = ".")]
    config_dir: String,
}

/// init chain config
/// $(config_dir)
/// --  $(chain_name)
/// ------  chain_config.toml
pub fn execute_init_chain_config(opts: InitChainConfigOpts) -> Result<(), Error> {
    let chain_config = ChainConfig::new().build();

    // TODO: add more args

    let file_name = format!(
        "{}/{}/{}",
        &opts.config_dir, &opts.chain_name, "chain_config.toml"
    );
    write_toml(&chain_config, file_name);

    Ok(())
}
