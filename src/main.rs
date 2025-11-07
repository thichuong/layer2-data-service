use anyhow::Result;
use std::sync::Arc;
use tonic::transport::Server;
use tonic_health::server::health_reporter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use layer2_data_service::{
    cache::CacheSystemIsland,
    config::Config,
    grpc_service::{
        market_data::market_data_service_server::MarketDataServiceServer, MarketDataGrpcService,
    },
    services::ExternalApisIsland,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "layer2_data_service=debug,tower_http=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("📋 Configuration loaded");
    tracing::info!("   Host: {}", config.host);
    tracing::info!("   HTTP Port: {} (health check)", config.http_port);
    tracing::info!("   Redis URL: {}", config.redis_url);
    tracing::info!("   Debug mode: {}", config.debug);

    // Initialize cache system
    tracing::info!("🔧 Initializing cache system...");
    let cache_system = CacheSystemIsland::new().await?;
    let cache_arc = Arc::new(cache_system);
    tracing::info!("✅ Cache system initialized");

    // Initialize external APIs island
    tracing::info!("🌐 Initializing External APIs Island...");
    let external_apis = ExternalApisIsland::with_cache_and_all_keys(
        config.taapi_secret.clone(),
        config.cmc_api_key.clone(),
        config.finnhub_api_key.clone(),
        Some(cache_arc),
    )
    .await?;
    tracing::info!("✅ External APIs Island initialized");

    let external_apis_arc = Arc::new(external_apis);

    // Configure gRPC health reporter
    let (mut health_reporter, health_service) = health_reporter();

    // Configure gRPC market data service
    let grpc_service = MarketDataGrpcService::new(Arc::clone(&external_apis_arc));

    // Single server address (Railway PORT)
    let addr = format!("{}:50051", config.host); 
    let socket_addr: std::net::SocketAddr = addr.parse().expect("Invalid bind address");
    tracing::info!("🚀 Starting gRPC Server (with Health Check) on {}", addr);
    tracing::info!("✅ Layer2 Data Service ready");
    tracing::info!("   gRPC (Health + MarketData): grpc://{} (STATIC PORT)", addr);

    // Set health status to SERVING for our service
    health_reporter
        .set_serving::<MarketDataServiceServer<MarketDataGrpcService>>()
        .await;

    // Run gRPC server with health check and market data services
    Server::builder()
        .add_service(health_service)
        .add_service(MarketDataServiceServer::new(grpc_service))
        .serve(socket_addr)
        .await?;

    Ok(())
}
