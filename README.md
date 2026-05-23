# Techub Comms

A Vonage-powered, full-scale voice, text, and video communication platform with web, desktop, and mobile clients.

## Architecture

```
techub-comms/           Turborepo monorepo root
  apps/
    web/                React + Vite web app (deployed to Vercel)
    server/             Express API server (deployed to Railway)
    desktop/            Tauri 2 + React desktop app (AppImage/DMG/EXE)
    mobile/             Expo + React Native mobile app (APK/AAB/IPA)
  packages/
    shared/             Shared types, stores (Zustand), API client, constants
```

## Features

- **Video Calls** — Vonage Video API (OpenTok) with recording, screen sharing, captions, and in-session chat
- **Voice Calls** — Vonage Voice API with outbound calls, IVR menus, TTS, DTMF, conferencing, and call recording
- **Messaging** — SMS, MMS, and WhatsApp via Vonage Messages API
- **Recordings** — List, play back, and download video archives

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Web frontend | React 18, Vite, Tailwind CSS, Zustand, TanStack Query |
| Desktop app | Tauri 2 (Rust), React, Vite |
| Mobile app | Expo SDK 52, React Native, Expo Router |
| Server | Express, Vonage SDK (Auth, Video, Voice, Messages), Zod |
| Shared | Zustand stores, TypeScript types, API client |
| Deploy (web) | Vercel |
| Deploy (server) | Railway |

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `VONAGE_API_KEY` | Vonage API key | Yes |
| `VONAGE_API_SECRET` | Vonage API secret | Yes |
| `VONAGE_APPLICATION_ID` | Vonage Application ID | Yes |
| `VONAGE_PRIVATE_KEY_PATH` | Path to private key file | Yes |
| `VONAGE_NUMBER` | Vonage phone number | For voice/SMS |
| `PORT` | Server port (default: 3039) | No |
| `BASE_URL` | Server public URL | Yes |
| `FRONTEND_URL` | Web app URL (for CORS) | Yes |
| `VITE_API_URL` | API URL for web client | Yes |
| `VITE_VONAGE_API_KEY` | Vonage key for web client | Yes |

## Quick Start

```bash
# Install dependencies
npm install

# Start all apps in development
npm run dev

# Or start individually
npm run dev:web        # http://localhost:3038
npm run dev:server     # http://localhost:3039
npm run dev:mobile     # Expo dev server
npm run dev:desktop    # Tauri dev
```

## Building

### Web
```bash
npm run build:web
```

### Server
```bash
npm run build:server
```

### Desktop (Tauri)
```bash
# Linux AppImage
npm run build:desktop:linux

# macOS DMG (Intel + Apple Silicon)
npm run build:desktop:mac

# Windows EXE
npm run build:desktop:win
```

### Mobile (Expo/EAS)
```bash
cd apps/mobile

# Android APK (preview)
eas build --platform android --profile preview

# Android AAB (production)
eas build --platform android --profile production

# iOS IPA
eas build --platform ios --profile production
```

## Deployment

See [DEPLOYMENT.md](./DEPLOYMENT.md) for detailed deployment instructions.

- **Web** — Deployed on Vercel at `thbtechub.sbs`
- **Server** — Deployed on Railway at `api.thbtechub.sbs`
- **Domain** — `thbtechub.sbs`

## License

MIT
