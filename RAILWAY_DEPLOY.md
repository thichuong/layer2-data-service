# Deploying Layer2 Data Service to Railway

## Prerequisites
- Railway account
- Redis database (add from Railway marketplace)
- PostgreSQL database (optional, for future use)

## Step 1: Create New Service

1. Go to Railway dashboard
2. Click "New Project" → "Empty Project"
3. Click "New" → "GitHub Repo"
4. Select your repository containing `layer2-data-service`

## Step 2: Configure Build

Railway will automatically detect the Dockerfile. If not:

1. Go to service settings
2. Set **Root Directory**: `layer2-data-service`
3. Set **Dockerfile Path**: `Dockerfile`

## Step 3: Set Environment Variables

In Railway dashboard, add these environment variables:

### Required Variables:

```env
# API Keys (get from respective services)
CMC_API_KEY=your_coinmarketcap_api_key
FINNHUB_API_KEY=your_finnhub_api_key
TAAPI_SECRET=your_taapi_jwt_token
TWELVE_DATA_API_KEY=your_twelvedata_api_key

# Redis (from Railway Redis service)
REDIS_URL=${{Redis.REDIS_URL}}

# Database (optional, for future features)
DATABASE_URL=${{Postgres.DATABASE_URL}}

# Service Configuration
SERVICE_PORT=${{PORT}}
HOST=0.0.0.0
RUST_LOG=info,layer2_data_service=debug
```

### Railway-Provided Variables:
- `PORT` - Automatically set by Railway (maps to our SERVICE_PORT)
- `RAILWAY_PUBLIC_DOMAIN` - Your service URL
- `RAILWAY_ENVIRONMENT` - production/development

## Step 4: Add Redis Database

1. Click "New" → "Database" → "Add Redis"
2. Wait for Redis to provision
3. Reference Redis URL in environment: `${{Redis.REDIS_URL}}`

## Step 5: Deploy

1. Railway will auto-deploy on every git push
2. Or manually: Click "Deploy" button
3. Wait for build to complete (~5-10 minutes first time)

## Step 6: Verify Deployment

### Health Check:
```bash
curl https://your-service.railway.app/health
```

Expected response:
```json
{
  "status": "healthy",
  "service": "Layer2 Data Service",
  "version": "0.1.0"
}
```

### Check Logs:
```
Railway Dashboard → Service → Logs
```

Look for:
```
✅ Layer2 Data Service ready
   HTTP API: http://0.0.0.0:8001
   gRPC API: grpc://0.0.0.0:50051
```

## Ports

Railway will expose:
- **HTTP API**: Port assigned by Railway (8001 internally)
- **gRPC API**: Port 50051 (for internal use)

**Note:** Railway's public URL will route to HTTP port. For gRPC, use Railway's private networking or expose separately.

## Connecting from Monolith

Set in monolith's environment:

```env
# If both deployed on Railway (private networking)
LAYER2_GRPC_URL=http://layer2-data-service.railway.internal:50051

# Or use public HTTP fallback
LAYER2_HTTP_URL=https://your-layer2-service.railway.app
```

## Troubleshooting

### Build Fails:
- Check Dockerfile path is correct
- Ensure `proto/` directory exists
- Verify protobuf-compiler installed in Dockerfile

### Service Crashes:
- Check environment variables are set
- Verify Redis connection: `${{Redis.REDIS_URL}}`
- Check logs for error messages

### Health Check Fails:
- Verify SERVICE_PORT matches Railway's PORT
- Check if service is listening on 0.0.0.0
- Ensure /health endpoint responds within timeout

### gRPC Connection Issues:
- Use Railway's private networking for gRPC
- Or deploy both services in same project
- Check firewall/network settings

## Performance Tips

1. **Enable caching**: Redis is critical for performance
2. **Monitor logs**: Watch for API rate limits
3. **Scale if needed**: Railway allows horizontal scaling
4. **Use private networking**: Connect monolith via internal network

## Cost Optimization

- Use Railway's free tier (500 hours/month)
- Sleep services during inactivity
- Monitor API usage to avoid rate limits
- Use Redis efficiently (set appropriate TTLs)

## Updating

Railway auto-deploys on git push:

```bash
git add .
git commit -m "Update layer2-data-service"
git push origin main
```

Railway will:
1. Pull latest code
2. Build new Docker image
3. Run health check
4. Switch traffic to new version
5. Zero-downtime deployment ✅
