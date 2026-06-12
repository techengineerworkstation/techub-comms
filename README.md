# Techub Comms — Enterprise Communications Platform

A multi-versatile communications platform built with **Rust** and **Leptos** (WASM), featuring voice calls, texting (SMS/WhatsApp/MMS), video conferencing, and recording capabilities. Powered by the Vonage Communications API.

## Architecture

```
techub-comms/
├── apps/
│   ├── backend_api/        # Rust backend (Actix-web)
│   ├── web_leptos/         # Leptos WASM frontend
│   ├── desktop/            # Tauri desktop app
│   ├── mobile/             # React Native/Expo mobile app
│   └── server/             # Node.js Express server (legacy)
├── packages/
│   ├── shared_core/        # Shared Rust types & API client
│   └── shared/             # Shared TypeScript types & stores
├── Dockerfile              # Multi-stage Docker build
├── render.yaml             # Render deployment config
└── .github/workflows/      # CI/CD pipelines
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Frontend (Web)** | Leptos 0.7 (Rust → WASM) |
| **Backend** | Actix-web 4 (Rust) |
| **Desktop** | Tauri 2 (Rust + WebView) |
| **Mobile** | React Native / Expo |
| **Theme** | Metallic Beige Tanned Turquoise |
| **API** | Vonage Video, Voice, Messages |
| **Deployment** | Docker, Render, GitHub Actions |

## Quick Start

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# WASM tools
cargo install trunk wasm-bindgen-cli

# Tauri (for desktop builds)
cargo install tauri-cli
```

### Development

```bash
# Clone
git clone https://github.com/YOUR_USERNAME/techub-comms.git
cd techub-comms

# Backend API
cd apps/backend_api
cp ../../.env.example .env   # Configure Vonage credentials
cargo run

# Frontend (in another terminal)
cd apps/web_leptos
trunk serve

# Open http://localhost:3038
```

### Environment Variables

Create `.env` in `apps/backend_api/`:

```env
VONAGE_API_KEY=your_api_key
VONAGE_API_SECRET=your_api_secret
VONAGE_APPLICATION_ID=your_app_id
VONAGE_PRIVATE_KEY_B64=base64_encoded_private_key
VONAGE_NUMBER=+1234567890
PORT=3039
BASE_URL=https://api.thbtechub.sbs
FRONTEND_URL=https://thbtechub.sbs
RUST_LOG=info
```

---

## Build Targets

### Web (WASM)

```bash
cd apps/web_leptos
trunk build --release
# Output: apps/web_leptos/dist/
```

### Backend Binary

```bash
cargo build --release --package backend_api
# Output: target/release/backend_api
```

### Docker

```bash
docker build -t techub-comms .
docker run -p 3039:3039 \
  -e VONAGE_API_KEY=key \
  -e VONAGE_API_SECRET=secret \
  -e VONAGE_APPLICATION_ID=app_id \
  techub-comms
```

### Desktop (Tauri)

#### macOS (.dmg / .app)

```bash
cd apps/desktop
npm install
cargo tauri build --target universal-apple-darwin
# Output: src-tauri/target/universal-apple-darwin/release/bundle/dmg/
```

#### Linux (.AppImage / .deb)

```bash
cd apps/desktop
npm install
cargo tauri build
# Output: src-tauri/target/release/bundle/appimage/
# Output: src-tauri/target/release/bundle/deb/
```

#### Windows (.exe / .msi)

```bash
cd apps/desktop
npm install
cargo tauri build --target x86_64-pc-windows-msvc
# Output: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/
```

### Mobile (React Native / Expo)

#### Android (.apk / .aab)

```bash
cd apps/mobile
npm install
npx expo prebuild
cd android
./gradlew assembleRelease    # .apk
./gradlew bundleRelease      # .aab
# Output: app/build/outputs/
```

#### iOS (.ipa)

```bash
cd apps/mobile
npm install
npx expo prebuild
cd ios
xcodebuild -workspace TechubComms.xcworkspace \
  -scheme TechubComms \
  -configuration Release \
  -archivePath build/TechubComms.xcarchive archive
# Export IPA via Xcode or xcodebuild -exportArchive
```

---

## Deployment

### Render (Recommended)

1. Push to GitHub
2. Connect repository to [Render](https://render.com)
3. Render reads `render.yaml` automatically
4. Set environment variables in Render dashboard:
   - `VONAGE_API_KEY`
   - `VONAGE_API_SECRET`
   - `VONAGE_APPLICATION_ID`
   - `VONAGE_PRIVATE_KEY_B64`
   - `VONAGE_NUMBER`

### Custom Domain (thbtechub.sbs)

1. In Render dashboard → Settings → Custom Domains
2. Add `thbtechub.sbs` and `api.thbtechub.sbs`
3. In Hostinger DNS settings, add:
   - `A` record → Render's IP
   - `CNAME` record for `api` → your-app.onrender.com

### GitHub Container Registry

Docker images are automatically pushed on merge to `main`:
```bash
ghcr.io/YOUR_USERNAME/techub-comms:latest
ghcr.io/YOUR_USERNAME/techub-comms:<commit-sha>
```

---

## Security

- **Content Security Policy** (CSP) headers on all responses
- **Rate limiting** (60 req/min per IP)
- **Input validation** (phone, room, DTMF, UUID, text sanitization)
- **HSTS** with preload
- **X-Frame-Options**: DENY
- **X-Content-Type-Options**: nosniff
- **XSS Protection**: 1; mode=block
- **Referrer Policy**: strict-origin-when-cross-origin
- **Permissions Policy**: camera=(), microphone=(), geolocation=()
- **Webhook verification tokens** for Vonage callbacks
- **CORS** restricted to configured origins
- **Non-root Docker user**

---

## Theme

The app uses a **Metallic Beige Tanned Turquoise** theme:

| Color | Hex | Usage |
|-------|-----|-------|
| Beige 50 | `#fdf8f0` | Background |
| Beige 100 | `#f5ead6` | Cards, borders |
| Beige 500 | `#c4a06a` | Metallic accents |
| Teal 500 | `#009999` | Primary actions |
| Teal 600 | `#007a7a` | Hover states |
| Teal 700 | `#005c5c` | Text emphasis |

### Glow Effects

- **glow-card**: Cards with teal glow on hover + metallic shine sweep
- **glow-tab**: Navigation tabs with hued gradient glow and active indicator
- **glow-sidebar**: Sidebar with turquoise edge glow on hover
- **glow-header**: Header with bottom glow line on hover

---

## CI/CD

GitHub Actions workflows:

- **ci.yml**: Check → Build → Docker test on every push/PR
- **deploy.yml**: Auto-deploy to Render + push Docker image to GHCR on main

---

## License

MIT License — see [LICENSE](LICENSE)
