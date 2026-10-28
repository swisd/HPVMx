//! Network module — protobuf request/response dispatcher over TCP/UDP.
//!
//! Provides a generic dispatcher that routes incoming protobuf messages
//! to registered handlers, with configurable transport (TCP or UDP).

pub mod protobuf;
pub mod generated;
pub mod dispatcher;
pub mod transport;
