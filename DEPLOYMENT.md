# Techub Comms - Deployment Guide

## Prerequisites

- GitHub repo: `techengineerworkstation/techub-comms`
- Vonage Application ID: `7e59865f-d02d-441c-9409-0ed517fcebd7`
- Private key file: `apps/server/keys/private.key` (NOT in repo — you must create this)
- Domain: `thbtechub.sbs`

---

## Step 1: Add the Private Key

The Vonage private key is gitignored. You must create it manually on the server (or locally for testing):

```bash
# Create the keys directory if it doesn't exist
mkdir -p apps/server/keys

# Paste your private key (from Vonage Dashboard > Applications > Edit)
cat > apps/server/keys/private.key << 'EOF'
-----BEGIN PRIVATE KEY-----
YOUR_PRIVATE_KEY_HERE
-----END PRIVATE KEY-----
EOF
```

---

## Step 2: Deploy Server to Railway

### Option A: Railway Dashboard (Recommended)

1. Go to [railway.app](https://railway.app) and log in
2. Click **"New Project"** > **"Deploy from GitHub Repo"**
3. Select `techengineerworkstation/techub-comms`
4. Railway will detect the `railway.json` config automatically
5. Set the **Root Directory** to `/` (monorepo root — railway.json handles the rest)
6. Go to **Variables** tab and add all environment variables from `apps/server/.env.example`:

   | Variable | Value |
   |----------|-------|
   | `VONAGE_API_KEY` | Your Vonage API key |
   | `VONAGE_API_SECRET` | Your Vonage API secret |
   | `VONAGE_APPLICATION_ID` | `7e59865f-d02d-441c-9409-0ed517fcebd7` |
   | `VONAGE_PRIVATE_KEY_PATH` | `./keys/private.key` |
   | `VONAGE_NUMBER` | Your Vonage phone number |
   | `BASE_URL` | `https://thbtechub.sbs` |
   | `FRONTEND_URL` | `https://thbtechub.sbs` |
   | `VONAGE_MONITORING_TOKEN` | `_5TZiliad-_H33o5QjT3ZDM6lnYEmMVswhrkYATn3HY` |
   | `VONAGE_RECORDING_TOKEN` | `sOdeJ-n4lBzxVwwnikUJmjHInnXozp-nqNEy0UTXnM4` |
   | `VONAGE_BROADCAST_TOKEN` | `9pSBRhJZcw__eHA8YyZw_IHgJHq-7cExhTjJmuvgG9Y` |
   | `VONAGE_COMPOSER_TOKEN` | `7qGspBn_4T-UPoBRhVSZLJf5a_5zD7QN6TYkHsAzcdY` |
   | `VONAGE_CAPTIONS_TOKEN` | `41dl4aDqN8EhA2iyvGzZvRjt2mRUvZ4zP-4SaU7ANmk` |
   | `VONAGE_SIP_MONITORING_TOKEN` | `LLR7ioj122iXcEi9iWPDnX6ioVN04KD2rZRVTmkjSJM` |

7. **Important**: The private key cannot be set as a simple env var. You have two options:
   - **Option A**: Use Railway's **Volume** feature to mount a persistent disk with the key file
   - **Option B**: Encode the private key as base64 and add it as `VONAGE_PRIVATE_KEY_B64`, then decode it at startup (requires code change)

8. Go to **Settings** > **Networking** > **Custom Domain** and add `thbtechub.sbs`
9. Railway will give you a DNS target (CNAME record). Update your domain's DNS:
   - Add a CNAME record: `thbtechub.sbs` → Railway's provided target

### Option B: Railway CLI

```bash
# Install Railway CLI
npm i -g @railway/cli

# Login
railway login

# Link to project (or create new)
railway link

# Set environment variables
railway variables set VONAGE_API_KEY="your_key"
railway variables set VONAGE_API_SECRET="your_secret"
railway variables set VONAGE_APPLICATION_ID="7e59865f-d02d-441c-9409-0ed517fcebd7"
railway variables set BASE_URL="https://thbtechub.sbs"
railway variables set FRONTEND_URL="https://thbtechub.sbs"
# ... (set all other variables from .env.example)

# Deploy
railway up
```

---

## Step 3: Add Domain to Vercel

### Option A: Vercel Dashboard (Recommended)

1. Go to [vercel.com](https://vercel.com) > project **techub-comms**
2. Go to **Settings** > **Domains**
3. Add `thbtechub.sbs`
4. Vercel will show DNS instructions:
   - If using the domain for the frontend only: Add an **A record** pointing to `76.76.21.21`
   - If sharing the domain between frontend and backend: Use a **CNAME** to `cname.vercel-dns.com` for the root, and route `/api/*` to Railway via Vercel rewrites
5. Also add `www.thbtechub.sbs` if needed

### Option B: Vercel CLI

```bash
# Install Vercel CLI
npm i -g vercel

# Login
vercel login

# Link to project
vercel link

# Add domain
vercel domains add thbtechub.sbs
```

---

## Step 4: DNS Configuration for thbtechub.sbs

Since both Vercel (frontend) and Railway (backend) need to be accessible on the same domain, you have two approaches:

### Approach A: Subdomain Split (Recommended)

| Record | Type | Value |
|--------|------|-------|
| `thbtechub.sbs` | A | `76.76.21.21` (Vercel) |
| `api.thbtechub.sbs` | CNAME | Railway's CNAME target |

Then update the server config:
- `BASE_URL=https://api.thbtechub.sbs`
- Update Vonage Dashboard callback URLs to use `https://api.thbtechub.sbs/...`

### Approach B: Vercel as Proxy (Advanced)

Use Vercel rewrites to proxy `/api/*` to Railway:

```json
// vercel.json
{
  "rewrites": [
    { "source": "/api/:path*", "destination": "https://your-railway-app.up.railway.app/api/:path*" },
    { "source": "/(.*)", "destination": "/index.html" }
  ]
}
```

This keeps everything on `thbtechub.sbs` but adds latency for API calls.

---

## Step 5: Update Vonage Dashboard Callback URLs

Once the server is deployed, update the Vonage Dashboard:

1. Go to [dashboard.nexmo.com](https://dashboard.nexmo.com) > **Video** > **Projects**
2. Select your project (Application ID: `7e59865f-...`)
3. Update callback URLs to point to your Railway server:
   - Session monitoring: `https://api.thbtechub.sbs/monitoring-event`
   - Recordings: `https://api.thbtechub.sbs/recording-event`
   - Broadcast: `https://api.thbtechub.sbs/broadcast-event`
   - Composer: `https://api.thbtechub.sbs/composer-event`
   - Captions: `https://api.thbtechub.sbs/captions-event`
   - SIP monitoring: `https://api.thbtechub.sbs/sip-monitoring-event`

---

## Verify Deployment

```bash
# Check server health
curl https://api.thbtechub.sbs/health

# Check frontend
curl -I https://thbtechub.sbs

# Test a webhook (should return 200)
curl -X POST https://api.thbtechub.sbs/monitoring-event \
  -H "Content-Type: application/json" \
  -d '{"test": true}'
```
