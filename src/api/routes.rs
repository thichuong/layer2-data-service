use axum::{
    routing::{get, post},
    Router,
};

use super::handlers::{
    health_check, fetch_dashboard_summary, fetch_crypto_prices,
    fetch_global_market_data, fetch_fear_greed_index, fetch_btc_rsi,
    fetch_us_indices, service_health_check, AppState,
};

/// Configure all API routes
pub fn configure_routes(state: AppState) -> Router {
    Router::new()
        // Health check (root)
        .route("/health", get(health_check))

        // API v1 routes
        .route("/api/v1/health", get(service_health_check))
        .route(
            "/api/v1/market-data/dashboard-summary",
            post(fetch_dashboard_summary),
        )
        .route(
            "/api/v1/market-data/crypto-prices",
            get(fetch_crypto_prices),
        )
        .route(
            "/api/v1/market-data/global",
            get(fetch_global_market_data),
        )
        .route(
            "/api/v1/market-data/fear-greed",
            get(fetch_fear_greed_index),
        )
        .route("/api/v1/market-data/btc-rsi", get(fetch_btc_rsi))
        .route("/api/v1/market-data/us-indices", get(fetch_us_indices))
        .with_state(state)
}
