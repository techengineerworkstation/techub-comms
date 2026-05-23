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

## Step 2: Push to GitHub

```bash
cd /home/hptechworkpc/Apps/techub-comms

# Stage all changes
git add .

# Commit
git commit -m "Fix critical issues: video subscribers, OpenTok CDN, mobile SDK"

# Push to GitHub
git push origin main
```

---

## Step 3: Deploy Server to Railway

### Option A: Railway Dashboard (Recommended)

1. Go to [railway.app](https://railway.app) and log in
2. Click **"New Project"** > **"Deploy from GitHub Repo"**
3. Select `techengineerworkstation/techub-comms`
4. Railway will detect the `railway.json` config automatically
5. Set the **Root Directory** to `/` (monorepo root — railway.json handles the rest)
6. Go to **Variables** tab and add all environment variables:

   | Variable | Value |
   |----------|-------|
   | `VONAGE_API_KEY` | Your Vonage API key |
   | `VONAGE_API_SECRET` | Your Vonage API secret |
   | `VONAGE_APPLICATION_ID` | `7e59865f-d02d-441c-9409-0ed517fcebd7` |
   | `VONAGE_PRIVATE_KEY_PATH` | `./keys/private.key` |
   | `VONAGE_NUMBER` | Your Vonage phone number |
   | `BASE_URL` | `https://api.thbtechub.sbs` |
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

8. Go to **Settings** > **Networking** > **Custom Domain** and add `api.thbtechub.sbs`
9. Railway will give you a DNS target (CNAME record). Save this for DNS setup.

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
railway variables set BASE_URL="https://api.thbtechub.sbs"
railway variables set FRONTEND_URL="https://thbtechub.sbs"
# ... (set all other variables from .env.example)

# Deploy
railway up
```

---

## Step 4: Deploy Frontend to Vercel

### Option A: Vercel Dashboard (Recommended)

1. Go to [vercel.com](https://vercel.com) and log in
2. Click **"Add New"** > **"Project"**
3. Import `techengineerworkstation/techub-comms` from GitHub
4. Configure:
   - **Framework Preset**: Vite
   - **Root Directory**: `apps/web`
   - **Build Command**: `npx turbo build --filter=web`
   - **Output Directory**: `dist`
5. Add **Environment Variables**:

   | Variable | Value |
   |----------|-------|
   | `VITE_API_URL` | `https://api.thbtechub.sbs` |
   | `VITE_VONAGE_API_KEY` | `ff261ddc` |

6. Click **Deploy**
7. After deploy, go to **Settings** > **Domains** and add `thbtechub.sbs`
8. Vercel will show DNS instructions. Save these for DNS setup.

### Option B: Vercel CLI

```bash
# Install Vercel CLI
npm i -g vercel

# Login
vercel login

# Deploy from web app directory
cd apps/web
vercel --prod

# Add domain
vercel domains add thbtechub.sbs
```

---

## Step 5: DNS Configuration for thbtechub.sbs

Since both Vercel (frontend) and Railway (backend) need to be accessible on the same domain, use a **subdomain split**:

### DNS Records to Add in Hostinger

| Record Type | Name | Value | TTL |
|-------------|------|-------|-----|
| **A** | `@` | `76.76.21.21` | 3600 |
| **CNAME** | `www` | `cname.vercel-dns.com` | 3600 |
| **CNAME** | `api` | `<your-railway-cname-target>` | 3600 |

### Step-by-Step in Hostinger hPanel

1. Log in to [hpanel.hostinger.com](https://hpanel.hostinger.com)
2. Go to **Domains** > **thbtechub.sbs** > **DNS / Nameservers**
3. Add the three records above:
   - **A record**: Name `@`, Value `76.76.21.21` (Vercel's IP)
   - **CNAME record**: Name `www`, Value `cname.vercel-dns.com`
   - **CNAME record**: Name `api`, Value your Railway CNAME target (e.g., `xxx.up.railway.app`)
4. Save changes. DNS propagation takes 5-30 minutes typically.

### Finding Your Railway CNAME Target

1. Go to Railway dashboard > your project > **Settings** > **Networking**
2. Click **Custom Domain** > add `api.thbtechub.sbs`
3. Railway will show a CNAME target (e.g., `b76wt8si.up.railway.app`)
4. Use this as the value for your `api` CNAME record in Hostinger

---

## Step 6: Update Vonage Dashboard Callback URLs

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

---

## Environment Variables Summary

### Railway (Server)

| Variable | Value |
|----------|-------|
| `VONAGE_API_KEY` | Your Vonage API key |
| `VONAGE_API_SECRET` | Your Vonage API secret |
| `VONAGE_APPLICATION_ID` | `7e59865f-d02d-441c-9409-0ed517fcebd7` |
| `VONAGE_PRIVATE_KEY_PATH` | `./keys/private.key` |
| `VONAGE_NUMBER` | Your Vonage phone number |
| `BASE_URL` | `https://api.thbtechub.sbs` |
| `FRONTEND_URL` | `https://thbtechub.sbs` |
| `VONAGE_MONITORING_TOKEN` | `_5TZiliad-_H33o5QjT3ZDM6lnYEmMVswhrkYATn3HY` |
| `VONAGE_RECORDING_TOKEN` | `sOdeJ-n4lBzxVwwnikUJmjHInnXozp-nqNEy0UTXnM4` |
| `VONAGE_BROADCAST_TOKEN` | `9pSBRhJZcw__eHA8YyZw_IHgJHq-7cExhTjJmuvgG9Y` |
| `VONAGE_COMPOSER_TOKEN` | `7qGspBn_4T-UPoBRhVSZLJf5a_5zD7QN6TYkHsAzcdY` |
| `VONAGE_CAPTIONS_TOKEN` | `41dl4aDqN8EhA2iyvGzZvRjt2mRUvZ4zP-4SaU7ANmk` |
| `VONAGE_SIP_MONITORING_TOKEN` | `LLR7ioj122iXcEi9iWPDnX6ioVN04KD2rZRVTmkjSJM` |

### Vercel (Frontend)

| Variable | Value |
|----------|-------|
| `VITE_API_URL` | `https://api.thbtechub.sbs` |
| `VITE_VONAGE_API_KEY` | `ff261ddc` |

---

## Step 7: Build Desktop App (Tauri 2)

The desktop app uses **Tauri 2** (Rust backend + React frontend) and builds natively for each platform. You must build **on** the target platform or use cross-compilation.

### Prerequisites

- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- System dependencies (Linux): `sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

### Build Commands

```bash
cd /home/hptechworkpc/Apps/techub-comms

# Linux AppImage
npm run build:desktop:linux

# macOS DMG (Apple Silicon)
npm run build:desktop:mac

# Windows EXE (must be built on Windows or via CI)
npm run build:desktop:win
```

Output binaries are in `apps/desktop/src-tauri/target/release/bundle/`:
- Linux: `*.AppImage`
- macOS: `*.dmg`
- Windows: `*.msi` / `*.exe`

### CI/CD with GitHub Actions

For cross-platform builds, set up GitHub Actions with matrix builds. The `tauri-apps/tauri-action` GitHub Action can build all platforms automatically on push/release.

---

## Step 8: Build Mobile App (Expo/EAS)

The mobile app uses **Expo SDK 52** with Expo Router. Builds are done via **EAS Build** (Expo Application Services) in the cloud.

### Prerequisites

```bash
# Install EAS CLI
npm install -g eas-cli

# Login to Expo
eas login

# Configure project
cd apps/mobile
eas build:configure
```

### Build Commands

```bash
cd apps/mobile

# Android APK (for testing/sideloading)
eas build --platform android --profile preview

# Android AAB (for Google Play Store)
eas build --platform android --profile production

# iOS IPA (requires Apple Developer account)
eas build --platform ios --profile production
```

### Submit to Stores

```bash
# Google Play Store
eas submit --platform android

# Apple App Store
eas submit --platform ios
```

### Environment Variables for Mobile

The mobile app reads the API URL from `apps/mobile/app/config.ts`:
- Default: `https://api.thbtechub.sbs`
- Override: Set `EXPO_PUBLIC_API_URL` environment variable

---

## Verify Deployment

```bash
# Check server health
curl https://api.thbtechub.sbs/health
