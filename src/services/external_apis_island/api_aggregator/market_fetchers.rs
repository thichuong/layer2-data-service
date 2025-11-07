//! Market Data Fetchers Component
//!
//! This module contains all the market data fetching methods with caching
//! for global market data, Fear & Greed Index, RSI, and US stock indices.

use anyhow::Result;
use std::sync::Arc;
use super::aggregator_core::ApiAggregator;

impl ApiAggregator {
    /// Fetch global data with type-safe automatic caching
    ///
    /// ✨ NEW: Uses get_or_compute_typed() for automatic caching
    pub async fn fetch_global_with_cache(&self) -> Result<serde_json::Value> {
        if let Some(ref cache) = self.cache_system {
            let market_api = Arc::clone(&self.market_api);

            cache.cache_manager.get_or_compute_typed(
                "global_coingecko_1h",
                crate::cache::CacheStrategy::MediumTerm, // 1 hour
                || async move {
                    println!("🔄 Fetching global data from API...");
                    let data = market_api.fetch_global_data().await?;
                    println!("✅ Global data fetched");
                    Ok(data)
                }
            ).await
        } else {
            // No cache - direct API call
            println!("⚠️ No cache system - calling API directly");
            self.market_api.fetch_global_data().await
        }
    }

    /// Fetch Fear & Greed with type-safe automatic caching
    ///
    /// ✨ NEW: Uses get_or_compute_typed() for automatic caching
    pub async fn fetch_fng_with_cache(&self) -> Result<serde_json::Value> {
        if let Some(ref cache) = self.cache_system {
            let market_api = Arc::clone(&self.market_api);

            cache.cache_manager.get_or_compute_typed(
                "fng_alternative_5m",
                crate::cache::CacheStrategy::ShortTerm, // 5 minutes
                || async move {
                    println!("🔄 Fetching Fear & Greed Index from API...");
                    let data = market_api.fetch_fear_greed_index().await?;
                    println!("✅ Fear & Greed Index fetched");
                    Ok(data)
                }
            ).await
        } else {
            // No cache - direct API call
            println!("⚠️ No cache system - calling API directly");
            self.market_api.fetch_fear_greed_index().await
        }
    }

    /// Fetch RSI with type-safe automatic caching
    ///
    /// ✨ NEW: Uses get_or_compute_typed() for automatic caching
    pub async fn fetch_btc_rsi_14_with_cache(&self) -> Result<serde_json::Value> {
        if let Some(ref cache) = self.cache_system {
            let market_api = Arc::clone(&self.market_api);

            cache.cache_manager.get_or_compute_typed(
                "btc_rsi_14_taapi_3h",
                crate::cache::CacheStrategy::LongTerm, // 3 hours
                || async move {
                    println!("🔄 Fetching BTC RSI-14 from API...");
                    let data = market_api.fetch_btc_rsi_14().await?;
                    println!("✅ BTC RSI-14 fetched");
                    Ok(data)
                }
            ).await
        } else {
            // No cache - direct API call
            println!("⚠️ No cache system - calling API directly");
            self.market_api.fetch_btc_rsi_14().await
        }
    }

    /// Fetch US Stock Indices with type-safe automatic caching
    ///
    /// ✨ NEW: Uses get_or_compute_typed() for automatic caching
    pub async fn fetch_us_indices_with_cache(&self) -> Result<serde_json::Value> {
        if let Some(ref cache) = self.cache_system {
            let market_api = Arc::clone(&self.market_api);

            cache.cache_manager.get_or_compute_typed(
                "us_indices_finnhub_5m",
                crate::cache::CacheStrategy::ShortTerm, // 5 minutes
                || async move {
                    println!("🔄 Fetching US Stock Indices from API...");
                    let data = market_api.fetch_us_stock_indices().await?;
                    println!("✅ US Stock Indices fetched");
                    Ok(data)
                }
            ).await
        } else {
            // No cache - direct API call
            println!("⚠️ No cache system - calling API directly");
            self.market_api.fetch_us_stock_indices().await
        }
    }
}