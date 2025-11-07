# Layer2 Data Service

Microservice độc lập để xử lý tất cả external API calls và data aggregation, tách ra từ Web-server-Report monolith.

## Chức năng

Layer2 Data Service cung cấp các API endpoints để fetch và aggregate market data từ nhiều nguồn:

- **Binance** - Crypto prices (BTC, ETH, SOL, XRP, ADA, LINK, BNB)
- **CoinGecko** - Global market data, fallback prices
- **CoinMarketCap** - Optional fallback source
- **TAAPI.io** - Technical indicators (RSI-14)
- **Finnhub** - US stock indices (SPX, DJI, IXIC)

## API Endpoints

### Health Check
```
GET  /health                                    - Basic health check
GET  /api/v1/health                             - Detailed health with component status
```

### Market Data
```
POST /api/v1/market-data/dashboard-summary      - Comprehensive dashboard data
GET  /api/v1/market-data/crypto-prices          - All crypto prices
GET  /api/v1/market-data/global                 - Global market statistics
GET  /api/v1/market-data/fear-greed             - Fear & Greed Index
GET  /api/v1/market-data/btc-rsi                - Bitcoin RSI-14
GET  /api/v1/market-data/us-indices             - US Stock Indices
```

### Dashboard Summary Request
```json
POST /api/v1/market-data/dashboard-summary
Content-Type: application/json

{
  "force_realtime_refresh": false
}
```

### Response Format
```json
{
  "btc_price_usd": 45000.0,
  "btc_change_24h": 2.5,
  "eth_price_usd": 2400.0,
  "eth_change_24h": 1.8,
  "sol_price_usd": 100.0,
  "xrp_price_usd": 0.6,
  "ada_price_usd": 0.5,
  "link_price_usd": 15.0,
  "bnb_price_usd": 300.0,
  "market_cap_usd": 1500000000000.0,
  "volume_24h_usd": 80000000000.0,
  "market_cap_change_percentage_24h_usd": 1.5,
  "btc_market_cap_percentage": 52.0,
  "eth_market_cap_percentage": 17.0,
  "fng_value": 65,
  "btc_rsi_14": 55.0,
  "us_stock_indices": {
    "SPX": { "c": 4500.0, "d": 20.5, "dp": 0.45 },
    "DJI": { "c": 35000.0, "d": 150.0, "dp": 0.43 },
    "IXIC": { "c": 14000.0, "d": 80.0, "dp": 0.57 }
  },
  "partial_failure": false,
  "fetch_duration_ms": 1250,
  "timestamp": "2025-11-07T12:00:00Z"
}
```

## Configuration

### Environment Variables

Required:
- `TAAPI_SECRET` - TAAPI.io API key (required)

Optional:
- `SERVICE_PORT` - Service port (default: 8001)
- `HOST` - Bind address (default: 0.0.0.0)
- `REDIS_URL` - Redis connection string (default: redis://127.0.0.1:6379)
- `CMC_API_KEY` - CoinMarketCap API key (enables fallback)
- `FINNHUB_API_KEY` - Finnhub API key (enables US indices)
- `RUST_LOG` - Logging level (default: info)
- `DEBUG` - Debug mode (0 or 1)

### Setup

1. Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

2. Fill in required values in `.env`:
```bash
TAAPI_SECRET=your_actual_taapi_secret
```

## Development

### Build and Run Locally

```bash
# Build
cargo build

# Run with debug logs
RUST_LOG=debug cargo run

# Build release
cargo build --release
./target/release/layer2_data_service
```

### Docker

```bash
# Build image
docker build -t layer2-data-service .

# Run container
docker run -p 8001:8001 \
  -e TAAPI_SECRET=your_secret \
  -e REDIS_URL=redis://host.docker.internal:6379 \
  layer2-data-service
```

### Docker Compose

From parent directory:
```bash
cd ..
docker-compose up layer2-service
```

## Architecture

### Caching Strategy

Layer2 service uses multi-tier caching:
- **L1 Cache** - Moka in-memory (2000 entries, 5min TTL)
- **L2 Cache** - Redis (1hr default TTL)
- **Cache Manager** - Unified interface with stampede protection

### Cache TTL by Data Type

| Data Type | Strategy | TTL |
|-----------|----------|-----|
| Crypto Prices | RealTime | 30s |
| Dashboard Summary | ShortTerm | 5min |
| Global Market Stats | MediumTerm | 1hr |
| Technical Indicators | LongTerm | 3hr |

### Fault Tolerance

- **Circuit Breaker** - Automatic fallback when APIs fail
- **Retry Logic** - Exponential backoff for transient errors
- **Partial Failures** - Return available data even if some APIs fail

## Performance

Expected performance metrics:
- **Latency**:
  - Cache hit: 2-5ms
  - Cache miss: 20-50ms
- **Throughput**: 8,000-10,000 RPS (with 90% cache hit rate)
- **Availability**: 99.9%+

## Monitoring

### Health Check
```bash
curl http://localhost:8001/health
curl http://localhost:8001/api/v1/health
```

### Logs
Service logs include:
- Request/response times
- Cache hit/miss rates
- API call success/failure
- Error details

## Integration

### From Monolith

Replace direct Layer2 calls with HTTP requests:

```rust
// OLD: Direct function call
let data = external_apis.fetch_dashboard_summary_v2(false).await?;

// NEW: HTTP request
let client = reqwest::Client::new();
let response = client
    .post("http://layer2-service:8001/api/v1/market-data/dashboard-summary")
    .json(&serde_json::json!({
        "force_realtime_refresh": false
    }))
    .send()
    .await?;
let data = response.json::<serde_json::Value>().await?;
```

### Service Discovery

Configure Layer2 service URL in monolith `.env`:
```env
LAYER2_SERVICE_URL=http://localhost:8001
```

## Troubleshooting

### Common Issues

**Service won't start**:
- Check if port 8001 is available
- Verify TAAPI_SECRET is set
- Ensure Redis is running

**Slow responses**:
- Check Redis connection
- Verify API keys are valid
- Check external API rate limits

**Cache not working**:
- Verify Redis connection in logs
- Check cache TTL settings
- Monitor cache hit/miss rates

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug DEBUG=1 cargo run
```

## License

Apache-2.0

## Contact

For issues or questions, please create an issue in the repository.
