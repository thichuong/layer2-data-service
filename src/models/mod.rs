// Data models for API requests and responses
// Most models are defined in services/external_apis_island/models.rs

use serde::{Deserialize, Serialize};

/// Request body for dashboard summary endpoint
#[derive(Debug, Deserialize)]
pub struct DashboardSummaryRequest {
    #[serde(default)]
    pub force_realtime_refresh: bool,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}
