//! Main entry point for RusticScheduler

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use rustic_scheduler::application::RUSTIC_SCHEDULER_APP;

/// Boot RusticScheduler
fn main() {
    abscissa_core::boot(&RUSTIC_SCHEDULER_APP);
}
