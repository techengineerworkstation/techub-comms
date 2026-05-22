import 'dotenv/config';

export const config = {
  vonage: {
    apiKey: process.env.VONAGE_API_KEY || '',
    apiSecret: process.env.VONAGE_API_SECRET || '',
    applicationId: process.env.VONAGE_APPLICATION_ID || '',
    privateKeyPath: process.env.VONAGE_PRIVATE_KEY_PATH || './keys/private.key',
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
  server: {
    port: parseInt(process.env.PORT || '3039'),
    baseUrl: process.env.BASE_URL || 'https://thbtechub.sbs',
  },
  frontend: {
    url: process.env.FRONTEND_URL || 'https://techub-comms.vercel.app',
  },
};
