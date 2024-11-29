//! RusticScheduler
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

#![allow(non_local_definitions)]

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;

pub(crate) mod message;
pub(crate) mod scheduler;
