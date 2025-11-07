# Layer2 Data Service - API Documentation

## Base URL
```
http://localhost:8001
```

## Authentication
Currently no authentication required. Add API keys or JWT tokens in production.

## Endpoints

### 1. Root Health Check
Basic health check endpoint.

**Endpoint**: `GET /health`

**Response**:
```json
{
  "status": "healthy",
  "service": "Layer2 Data Service",
  "version": "0.1.0"
}
```

**Status Codes**:
- `200 OK` - Service is healthy

---

### 2. Service Health Check
Detailed health check with component status.

**Endpoint**: `GET /api/v1/health`

**Response**:
```json
{
  "status": "healthy",
  "service": "Layer2 Data Service",
  "version": "0.1.0",
  "components": {
    "external_apis": true
  }
}
```

**Status Codes**:
- `200 OK` - Service is healthy
- `503 Service Unavailable` - Service is degraded

---

### 3. Dashboard Summary
Comprehensive market data for dashboard display. This is the PRIMARY endpoint.

**Endpoint**: `POST /api/v1/market-data/dashboard-summary`

**Request Body**:
```json
{
  "force_realtime_refresh": false
}
```

**Parameters**:
- `force_realtime_refresh` (boolean, optional): If `true`, bypasses cache and fetches fresh data. Default: `false`

**Response**:
```json
{
  "btc_price_usd": 45000.50,
  "btc_change_24h": 2.5,
  "eth_price_usd": 2400.75,
  "eth_change_24h": 1.8,
  "sol_price_usd": 100.25,
  "sol_change_24h": 3.2,
  "xrp_price_usd": 0.62,
  "xrp_change_24h": 1.5,
  "ada_price_usd": 0.52,
  "ada_change_24h": 0.9,
  "link_price_usd": 15.30,
  "link_change_24h": 2.1,
  "bnb_price_usd": 305.50,
  "bnb_change_24h": 1.3,
  "market_cap_usd": 1500000000000.0,
  "volume_24h_usd": 80000000000.0,
  "market_cap_change_percentage_24h_usd": 1.5,
  "btc_market_cap_percentage": 52.3,
  "eth_market_cap_percentage": 17.2,
  "fng_value": 65,
  "btc_rsi_14": 55.4,
  "us_stock_indices": {
    "SPX": {
      "c": 4500.25,
      "d": 20.50,
      "dp": 0.45,
      "t": 1699372800
    },
    "DJI": {
      "c": 35000.75,
      "d": 150.25,
      "dp": 0.43,
      "t": 1699372800
    },
    "IXIC": {
      "c": 14000.50,
      "d": 80.25,
      "dp": 0.57,
      "t": 1699372800
    }
  },
  "data_sources": {
    "crypto_prices": "binance",
    "global_data": "coingecko",
    "fng": "alternative.me",
    "btc_rsi": "taapi",
    "us_indices": "finnhub"
  },
  "partial_failure": false,
  "fetch_duration_ms": 1250,
  "last_updated": "2025-11-07T12:00:00Z",
  "timestamp": "2025-11-07T12:00:00Z"
}
```

**Field Descriptions**:

| Field | Type | Description |
|-------|------|-------------|
| `btc_price_usd` | float | Bitcoin price in USD |
| `btc_change_24h` | float | BTC 24h price change percentage |
| `eth_price_usd` | float | Ethereum price in USD |
| `eth_change_24h` | float | ETH 24h price change percentage |
| `sol_price_usd` | float | Solana price in USD |
| `sol_change_24h` | float | SOL 24h price change percentage |
| `xrp_price_usd` | float | Ripple price in USD |
| `ada_price_usd` | float | Cardano price in USD |
| `link_price_usd` | float | Chainlink price in USD |
| `bnb_price_usd` | float | Binance Coin price in USD |
| `market_cap_usd` | float | Total crypto market cap in USD |
| `volume_24h_usd` | float | Total 24h trading volume in USD |
| `market_cap_change_percentage_24h_usd` | float | Market cap 24h change percentage |
| `btc_market_cap_percentage` | float | BTC dominance percentage |
| `eth_market_cap_percentage` | float | ETH dominance percentage |
| `fng_value` | integer | Fear & Greed Index (0-100) |
| `btc_rsi_14` | float | Bitcoin RSI-14 indicator |
| `us_stock_indices` | object | US stock market indices |
| `us_stock_indices.SPX` | object | S&P 500 index data |
| `us_stock_indices.DJI` | object | Dow Jones index data |
| `us_stock_indices.IXIC` | object | NASDAQ index data |
| `data_sources` | object | Sources used for each data point |
| `partial_failure` | boolean | True if some APIs failed |
| `fetch_duration_ms` | integer | Time taken to fetch data (ms) |
| `timestamp` | string | ISO 8601 timestamp |

**US Stock Index Fields**:
- `c` - Current price/level
- `d` - Daily change
- `dp` - Daily change percentage
- `t` - Unix timestamp

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

**Cache Behavior**:
- Default (force_realtime_refresh=false): Returns cached data if available (30s TTL)
- Force refresh (force_realtime_refresh=true): Bypasses cache and fetches fresh data

---

### 4. Crypto Prices
All cryptocurrency prices only.

**Endpoint**: `GET /api/v1/market-data/crypto-prices`

**Response**:
```json
{
  "BTC": {
    "symbol": "BTCUSDT",
    "last_price": "45000.50",
    "price_change_percent": "2.50"
  },
  "ETH": {
    "symbol": "ETHUSDT",
    "last_price": "2400.75",
    "price_change_percent": "1.80"
  },
  ...
}
```

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

---

### 5. Global Market Data
Global cryptocurrency market statistics.

**Endpoint**: `GET /api/v1/market-data/global`

**Response**:
```json
{
  "total_market_cap": {
    "usd": 1500000000000.0
  },
  "total_volume": {
    "usd": 80000000000.0
  },
  "market_cap_change_percentage_24h_usd": 1.5,
  "market_cap_percentage": {
    "btc": 52.3,
    "eth": 17.2
  }
}
```

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

---

### 6. Fear & Greed Index
Crypto market sentiment indicator.

**Endpoint**: `GET /api/v1/market-data/fear-greed`

**Response**:
```json
{
  "value": "65",
  "value_classification": "Greed"
}
```

**Value Classifications**:
- 0-24: Extreme Fear
- 25-49: Fear
- 50-74: Greed
- 75-100: Extreme Greed

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

---

### 7. Bitcoin RSI-14
Bitcoin Relative Strength Index (14-period).

**Endpoint**: `GET /api/v1/market-data/btc-rsi`

**Response**:
```json
{
  "value": 55.4,
  "timestamp": 1699372800
}
```

**RSI Interpretation**:
- 0-30: Oversold
- 30-70: Neutral
- 70-100: Overbought

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

---

### 8. US Stock Indices
Major US stock market indices.

**Endpoint**: `GET /api/v1/market-data/us-indices`

**Response**:
```json
{
  "SPX": {
    "c": 4500.25,
    "d": 20.50,
    "dp": 0.45,
    "t": 1699372800
  },
  "DJI": {
    "c": 35000.75,
    "d": 150.25,
    "dp": 0.43,
    "t": 1699372800
  },
  "IXIC": {
    "c": 14000.50,
    "d": 80.25,
    "dp": 0.57,
    "t": 1699372800
  }
}
```

**Status Codes**:
- `200 OK` - Success
- `500 Internal Server Error` - Failed to fetch data

---

## Error Responses

All endpoints may return error responses in the following format:

```json
{
  "error": "Internal server error",
  "message": "Detailed error message"
}
```

**Common Status Codes**:
- `400 Bad Request` - Invalid request parameters
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error
- `503 Service Unavailable` - Service temporarily unavailable

---

## Rate Limiting

Currently no rate limiting implemented. Consider adding rate limiting in production:
- Recommended: 100 requests per minute per IP
- Dashboard summary: 60 requests per minute (due to real-time data)

---

## Caching

Layer2 service implements multi-tier caching:

| Data Type | Cache Strategy | TTL |
|-----------|----------------|-----|
| Crypto Prices | RealTime | 30s |
| Dashboard Summary | ShortTerm | 5min |
| Global Market | MediumTerm | 1hr |
| Fear & Greed | ShortTerm | 5min |
| BTC RSI | LongTerm | 3hr |
| US Indices | ShortTerm | 5min |

**Cache Bypass**: Use `force_realtime_refresh: true` in dashboard summary request.

---

## Examples

### cURL Examples

**Dashboard Summary**:
```bash
curl -X POST http://localhost:8001/api/v1/market-data/dashboard-summary \
  -H "Content-Type: application/json" \
  -d '{"force_realtime_refresh": false}'
```

**Crypto Prices**:
```bash
curl http://localhost:8001/api/v1/market-data/crypto-prices
```

**Health Check**:
```bash
curl http://localhost:8001/api/v1/health
```

### JavaScript/TypeScript Example

```typescript
const fetchDashboardData = async (forceRefresh: boolean = false) => {
  const response = await fetch('http://localhost:8001/api/v1/market-data/dashboard-summary', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      force_realtime_refresh: forceRefresh
    })
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return await response.json();
};

// Usage
try {
  const data = await fetchDashboardData(false);
  console.log('BTC Price:', data.btc_price_usd);
  console.log('ETH Price:', data.eth_price_usd);
} catch (error) {
  console.error('Failed to fetch dashboard data:', error);
}
```

### Python Example

```python
import requests

def fetch_dashboard_data(force_refresh=False):
    url = "http://localhost:8001/api/v1/market-data/dashboard-summary"
    payload = {"force_realtime_refresh": force_refresh}

    response = requests.post(url, json=payload)
    response.raise_for_status()

    return response.json()

# Usage
try:
    data = fetch_dashboard_data(force_refresh=False)
    print(f"BTC Price: ${data['btc_price_usd']}")
    print(f"ETH Price: ${data['eth_price_usd']}")
except requests.exceptions.RequestException as e:
    print(f"Error fetching data: {e}")
```

### Rust Example

```rust
use reqwest::Client;
use serde_json::{json, Value};

async fn fetch_dashboard_data(force_refresh: bool) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8001/api/v1/market-data/dashboard-summary")
        .json(&json!({
            "force_realtime_refresh": force_refresh
        }))
        .send()
        .await?;

    let data = response.json::<Value>().await?;
    Ok(data)
}

// Usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_dashboard_data(false).await?;

    println!("BTC Price: ${}", data["btc_price_usd"]);
    println!("ETH Price: ${}", data["eth_price_usd"]);

    Ok(())
}
```

---

## Changelog

### v0.1.0 (2025-11-07)
- Initial release
- Dashboard summary endpoint
- Individual market data endpoints
- Multi-tier caching
- Health checks
- Docker support

---

## Support

For issues or questions:
1. Check logs: `docker logs webreport-layer2-service`
2. Verify configuration: Check `.env` file
3. Test health endpoint: `curl http://localhost:8001/health`
4. Create issue in repository

---

## Future Enhancements

Planned features:
- WebSocket support for real-time updates
- GraphQL API
- API authentication (JWT)
- Rate limiting
- Metrics endpoint (Prometheus)
- Additional data sources
- Historical data endpoints
