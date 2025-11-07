// gRPC Service Implementation for Market Data

use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::services::ExternalApisIsland;

// Include generated protobuf code
pub mod market_data {
    tonic::include_proto!("market_data");
}

use market_data::market_data_service_server::MarketDataService;
use market_data::*;

pub struct MarketDataGrpcService {
    external_apis: Arc<ExternalApisIsland>,
}

impl MarketDataGrpcService {
    pub fn new(external_apis: Arc<ExternalApisIsland>) -> Self {
        Self { external_apis }
    }
}

#[tonic::async_trait]
impl MarketDataService for MarketDataGrpcService {
    async fn get_dashboard_summary(
        &self,
        request: Request<DashboardSummaryRequest>,
    ) -> Result<Response<DashboardSummaryResponse>, Status> {
        let req = request.into_inner();

        // Fetch data from external APIs
        let data = self
            .external_apis
            .fetch_dashboard_summary_v2(req.force_realtime_refresh)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch dashboard summary: {}", e)))?;

        // Convert JSON to protobuf response
        let response = DashboardSummaryResponse {
            btc_price_usd: data.get("btc_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            btc_change_24h: data.get("btc_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            eth_price_usd: data.get("eth_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            eth_change_24h: data.get("eth_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            sol_price_usd: data.get("sol_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            sol_change_24h: data.get("sol_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            xrp_price_usd: data.get("xrp_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            xrp_change_24h: data.get("xrp_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            ada_price_usd: data.get("ada_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            ada_change_24h: data.get("ada_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            link_price_usd: data.get("link_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            link_change_24h: data.get("link_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            bnb_price_usd: data.get("bnb_price_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            bnb_change_24h: data.get("bnb_change_24h").and_then(|v| v.as_f64()).unwrap_or(0.0),
            market_cap_usd: data.get("market_cap_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            volume_24h_usd: data.get("volume_24h_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            market_cap_change_percentage_24h_usd: data.get("market_cap_change_percentage_24h_usd").and_then(|v| v.as_f64()).unwrap_or(0.0),
            btc_market_cap_percentage: data.get("btc_market_cap_percentage").and_then(|v| v.as_f64()).unwrap_or(0.0),
            eth_market_cap_percentage: data.get("eth_market_cap_percentage").and_then(|v| v.as_f64()).unwrap_or(0.0),
            fng_value: data.get("fng_value").and_then(|v| v.as_u64()).unwrap_or(50) as u32,
            btc_rsi_14: data.get("btc_rsi_14").and_then(|v| v.as_f64()).unwrap_or(50.0),
            us_stock_indices: Self::parse_us_indices(&data),
            partial_failure: data.get("partial_failure").and_then(|v| v.as_bool()).unwrap_or(false),
            fetch_duration_ms: data.get("fetch_duration_ms").and_then(|v| v.as_u64()).unwrap_or(0),
            timestamp: data.get("timestamp").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_crypto_prices(
        &self,
        request: Request<CryptoPricesRequest>,
    ) -> Result<Response<CryptoPricesResponse>, Status> {
        let req = request.into_inner();

        let prices_map = self
            .external_apis
            .aggregator
            .fetch_all_crypto_prices_with_cache(req.force_refresh)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch crypto prices: {}", e)))?;

        let mut prices = std::collections::HashMap::new();
        for (symbol, price_data) in prices_map {
            if let Some(obj) = price_data.as_object() {
                prices.insert(
                    symbol,
                    CryptoPrice {
                        symbol: obj.get("symbol").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        last_price: obj.get("last_price").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
                        price_change_percent: obj.get("price_change_percent").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
                    },
                );
            }
        }

        Ok(Response::new(CryptoPricesResponse { prices }))
    }

    async fn get_global_market_data(
        &self,
        _request: Request<GlobalMarketDataRequest>,
    ) -> Result<Response<GlobalMarketDataResponse>, Status> {
        let data = self
            .external_apis
            .aggregator
            .fetch_global_with_cache()
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch global market data: {}", e)))?;

        let response = GlobalMarketDataResponse {
            total_market_cap_usd: data
                .get("total_market_cap")
                .and_then(|v| v.get("usd"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            total_volume_usd: data
                .get("total_volume")
                .and_then(|v| v.get("usd"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            market_cap_change_percentage_24h_usd: data
                .get("market_cap_change_percentage_24h_usd")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            btc_market_cap_percentage: data
                .get("market_cap_percentage")
                .and_then(|v| v.get("btc"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            eth_market_cap_percentage: data
                .get("market_cap_percentage")
                .and_then(|v| v.get("eth"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
        };

        Ok(Response::new(response))
    }

    async fn get_fear_greed_index(
        &self,
        _request: Request<FearGreedIndexRequest>,
    ) -> Result<Response<FearGreedIndexResponse>, Status> {
        let data = self
            .external_apis
            .aggregator
            .fetch_fng_with_cache()
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch Fear & Greed Index: {}", e)))?;

        let value = data.get("value").and_then(|v| v.as_str()).unwrap_or("50");
        let value_num: u32 = value.parse().unwrap_or(50);

        let classification = match value_num {
            0..=24 => "Extreme Fear",
            25..=49 => "Fear",
            50..=74 => "Greed",
            75..=100 => "Extreme Greed",
            _ => "Unknown",
        };

        Ok(Response::new(FearGreedIndexResponse {
            value: value_num,
            value_classification: classification.to_string(),
        }))
    }

    async fn get_btc_rsi(
        &self,
        _request: Request<BtcRsiRequest>,
    ) -> Result<Response<BtcRsiResponse>, Status> {
        let data = self
            .external_apis
            .aggregator
            .fetch_btc_rsi_14_with_cache()
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch BTC RSI: {}", e)))?;

        Ok(Response::new(BtcRsiResponse {
            value: data.get("value").and_then(|v| v.as_f64()).unwrap_or(50.0),
            timestamp: data.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0),
        }))
    }

    async fn get_us_indices(
        &self,
        _request: Request<UsIndicesRequest>,
    ) -> Result<Response<UsIndicesResponse>, Status> {
        let data = self
            .external_apis
            .aggregator
            .fetch_us_indices_with_cache()
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch US indices: {}", e)))?;

        let indices = Self::parse_us_indices(&data);

        Ok(Response::new(UsIndicesResponse {
            indices,
        }))
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let external_apis_healthy = self.external_apis.health_check().await.unwrap_or(false);

        Ok(Response::new(HealthCheckResponse {
            status: if external_apis_healthy { "healthy" } else { "degraded" }.to_string(),
            service: "Layer2 Data Service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            external_apis_healthy,
        }))
    }
}

impl MarketDataGrpcService {
    fn parse_us_indices(data: &serde_json::Value) -> Option<UsStockIndices> {
        let indices = data.get("us_stock_indices")?.as_object()?;

        Some(UsStockIndices {
            spx: Self::parse_stock_index(indices.get("SPX")),
            dji: Self::parse_stock_index(indices.get("DJI")),
            ixic: Self::parse_stock_index(indices.get("IXIC")),
        })
    }

    fn parse_stock_index(data: Option<&serde_json::Value>) -> Option<StockIndex> {
        let obj = data?.as_object()?;

        Some(StockIndex {
            current_price: obj.get("c").and_then(|v| v.as_f64()).unwrap_or(0.0),
            daily_change: obj.get("d").and_then(|v| v.as_f64()).unwrap_or(0.0),
            daily_change_percentage: obj.get("dp").and_then(|v| v.as_f64()).unwrap_or(0.0),
            timestamp: obj.get("t").and_then(|v| v.as_i64()).unwrap_or(0),
        })
    }
}
