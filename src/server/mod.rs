/// Quote server components:
/// - TCP server handling client commands
/// - Quote generator
/// - Stock quote types
/// - Internal UDP streaming helper
pub mod quote_server;

/// Quote generation logic.
pub mod generator;

/// Stock quote data structure.
pub mod stock;

/// Internal UDP stream handler.
mod quote_stream;

/// Public re-export of the quote server type.
pub use quote_server::QuoteServer;