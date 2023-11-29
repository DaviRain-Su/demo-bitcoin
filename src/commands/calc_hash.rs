//! `calc hash` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::config::DemoBitcoinConfig;
use crate::pow::TARGET_BITS;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use num_bigint::BigUint;
use num_traits::One;
use sha2::{Digest, Sha256};

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct CalcHashCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

fn calca_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_result = hasher.finalize().to_vec();
    let mut hash1 = [0; 32];
    hash1.copy_from_slice(&hash_result);
    hash1
}

impl Runnable for CalcHashCmd {
    /// Start the application.
    fn run(&self) {
        let data1 = b"I like donuts";
        let data2 = b"I like donutsca07ca";

        let one: BigUint = One::one();
        let target = one << (256u16 - TARGET_BITS);

        let data1_hash = calca_hash(data1);
        let data2_hash = calca_hash(data2);
        println!("{}", hex::encode(data1_hash));
        println!("{:64x}", target);
        println!("{}", hex::encode(data2_hash));
    }
}

impl config::Override<DemoBitcoinConfig> for CalcHashCmd {
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
