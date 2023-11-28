//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
// use crate::prelude::*;
use crate::types::Blockchain;

use crate::config::DemoBitcoinConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct RunCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for RunCmd {
    /// Start the application.
    fn run(&self) {
        let mut new_blockchain = Blockchain::new_genesis_block();
        if let Err(e) = new_blockchain.add_block("Send 1 BTC to Ivan".into()) {
            println!("Error: {}", e);
        }
        if let Err(e) = new_blockchain.add_block("Send 2 more BTC to Ivan".into()) {
            println!("Error: {}", e);
        }

        println!("{}", new_blockchain);
    }
}

impl config::Override<DemoBitcoinConfig> for RunCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: DemoBitcoinConfig,
    ) -> Result<DemoBitcoinConfig, FrameworkError> {
        if !self.recipient.is_empty() {
            config.hello.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
