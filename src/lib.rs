/// All global parameters
pub mod app;

/// Business logic
pub mod domain;

/// External resources (db, s3, microservices)
pub mod infrastructure;

/// Universal utilities
pub mod util;

/// All external API for connecting to this app
pub mod api;
