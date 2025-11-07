// Layer2 Data Service Library
// gRPC-only microservice for Layer 2 data aggregation

pub mod cache;
pub mod config;
pub mod grpc_service;
pub mod models;
pub mod performance;
pub mod services;

// Re-export key components
pub use cache::CacheSystemIsland;
pub use config::Config;
pub use services::ExternalApisIsland;
