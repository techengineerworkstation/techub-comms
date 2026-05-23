import 'dotenv/config';

// Resolve private key: prefer base64 env var (for Railway), fall back to file path (local dev)
function resolvePrivateKey(): string {
  const b64 = process.env.VONAGE_PRIVATE_KEY_B64;
  if (b64) {
    return Buffer.from(b64, 'base64').toString('utf-8');
  }
  return process.env.VONAGE_PRIVATE_KEY_PATH || './keys/private.key';
}

export const config = {
  vonage: {
    apiKey: process.env.VONAGE_API_KEY || '',
    apiSecret: process.env.VONAGE_API_SECRET || '',
    applicationId: process.env.VONAGE_APPLICATION_ID || '',
    privateKey: resolvePrivateKey(),
    number: process.env.VONAGE_NUMBER || '',
  },
  callbacks: {
    monitoring: process.env.VONAGE_MONITORING_CALLBACK || '/monitoring-event',
    recording: process.env.VONAGE_RECORDING_CALLBACK || '/recording-event',
    broadcast: process.env.VONAGE_BROADCAST_CALLBACK || '/broadcast-event',
    composer: process.env.VONAGE_COMPOSER_CALLBACK || '/composer-event',
    captions: process.env.VONAGE_CAPTIONS_CALLBACK || '/captions-event',
    sipMonitoring: process.env.VONAGE_SIP_MONITORING_CALLBACK || '/sip-monitoring-event',
  },
  verificationTokens: {
    monitoring: process.env.VONAGE_MONITORING_TOKEN || '',
    recording: process.env.VONAGE_RECORDING_TOKEN || '',
    broadcast: process.env.VONAGE_BROADCAST_TOKEN || '',
    composer: process.env.VONAGE_COMPOSER_TOKEN || '',
    captions: process.env.VONAGE_CAPTIONS_TOKEN || '',
    sipMonitoring: process.env.VONAGE_SIP_MONITORING_TOKEN || '',
  },
  server: {
    port: parseInt(process.env.PORT || '3039'),
    baseUrl: process.env.BASE_URL || 'https://thbtechub.sbs',
  },
  frontend: {
    url: process.env.FRONTEND_URL || 'https://techub-comms.vercel.app',
  },
};
