// API module for Layer2 Data Service

pub mod handlers;
pub mod routes;

// Re-export for convenience
pub use handlers::AppState;
pub use routes::configure_routes;
