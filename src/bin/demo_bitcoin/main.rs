//! Main entry point for DemoBitcoin

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use demo_bitcoin::application::APP;

/// Boot DemoBitcoin
fn main() {
    abscissa_core::boot(&APP);
}
