use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::sync::Arc;

use crate::models::{DashboardSummaryRequest, HealthResponse};
use crate::services::ExternalApisIsland;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub external_apis: Arc<ExternalApisIsland>,
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "Layer2 Data Service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    (StatusCode::OK, Json(response))
}

/// Fetch dashboard summary with market data
///
/// POST /api/v1/market-data/dashboard-summary
pub async fn fetch_dashboard_summary(
    State(state): State<AppState>,
    Json(request): Json<DashboardSummaryRequest>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .fetch_dashboard_summary_v2(request.force_realtime_refresh)
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Fetch crypto prices only
///
/// GET /api/v1/market-data/crypto-prices
pub async fn fetch_crypto_prices(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .aggregator
        .fetch_all_crypto_prices_with_cache(false)
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Fetch global market data
///
/// GET /api/v1/market-data/global
pub async fn fetch_global_market_data(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .aggregator
        .fetch_global_with_cache()
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Fetch Fear & Greed Index
///
/// GET /api/v1/market-data/fear-greed
pub async fn fetch_fear_greed_index(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .aggregator
        .fetch_fng_with_cache()
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Fetch BTC RSI-14
///
/// GET /api/v1/market-data/btc-rsi
pub async fn fetch_btc_rsi(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .aggregator
        .fetch_btc_rsi_14_with_cache()
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Fetch US Stock Indices
///
/// GET /api/v1/market-data/us-indices
pub async fn fetch_us_indices(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let data = state
        .external_apis
        .aggregator
        .fetch_us_indices_with_cache()
        .await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

/// Service health check with component status
///
/// GET /api/v1/health
pub async fn service_health_check(
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let external_apis_healthy = state
        .external_apis
        .health_check()
        .await
        .unwrap_or(false);

    let response = json!({
        "status": if external_apis_healthy { "healthy" } else { "degraded" },
        "service": "Layer2 Data Service",
        "version": env!("CARGO_PKG_VERSION"),
        "components": {
            "external_apis": external_apis_healthy,
        }
    });

    Ok((StatusCode::OK, Json(response)).into_response())
}

/// Error type for API handlers
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Handler error: {:?}", self.0);

        let error_message = json!({
            "error": "Internal server error",
            "message": self.0.to_string(),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
