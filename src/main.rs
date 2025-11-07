use anyhow::Result;
use std::sync::Arc;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use layer2_data_service::{
    api::{configure_routes, AppState},
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
    tracing::info!("   Service will bind to: {}", config.bind_address());
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

    // Create application state for HTTP API
    let app_state = AppState {
        external_apis: Arc::clone(&external_apis_arc),
    };

    // Configure HTTP routes
    let app = configure_routes(app_state);

    // Configure gRPC service
    let grpc_service = MarketDataGrpcService::new(Arc::clone(&external_apis_arc));

    // Start HTTP server
    let http_addr = config.bind_address();
    tracing::info!("🚀 Starting HTTP Server on {}", http_addr);

    let http_addr: std::net::SocketAddr = http_addr.parse().expect("Invalid HTTP bind address");

    // Start gRPC server
    let grpc_addr = format!("{}:50051", config.host);
    tracing::info!("🚀 Starting gRPC Server on {}", grpc_addr);

    let grpc_addr: std::net::SocketAddr = grpc_addr.parse().expect("Invalid gRPC bind address");

    // Run both servers concurrently
    tracing::info!("✅ Layer2 Data Service ready");
    tracing::info!("   HTTP API: http://{}", http_addr);
    tracing::info!("   gRPC API: grpc://{}", grpc_addr);

    tokio::try_join!(
        // HTTP Server
        async {
            axum::Server::bind(&http_addr)
                .serve(app.into_make_service())
                .await
                .map_err(|e| anyhow::anyhow!("HTTP server error: {}", e))
        },
        // gRPC Server
        async {
            Server::builder()
                .add_service(MarketDataServiceServer::new(grpc_service))
                .serve(grpc_addr)
                .await
                .map_err(|e| anyhow::anyhow!("gRPC server error: {}", e))
        }
    )?;

    Ok(())
}
