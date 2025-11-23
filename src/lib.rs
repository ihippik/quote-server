//! # Quote Server
//!
//! A client–server application that emulates stock quote streaming over TCP and UDP.
//!
//! This crate provides:
//! - a TCP/UDP quote **server** that generates and streams stock data,
//! - a TCP/UDP **client** that subscribes to selected tickers and receives quotes.

#![warn(missing_docs)]

/// Server-side components:
/// - TCP command handling
/// - quote generation
/// - UDP streaming to subscribers
pub mod server;

/// Client-side components:
/// - TCP communication helpers
/// - UDP receiver and ping logic
pub mod client;