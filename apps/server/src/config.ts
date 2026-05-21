import 'dotenv/config';

export const config = {
  vonage: {
    apiKey: process.env.VONAGE_API_KEY || 'ff261ddc',
    apiSecret: process.env.VONAGE_API_SECRET || 'lGKYe7LtDyFMcoOz',
    applicationId: process.env.VONAGE_APPLICATION_ID || '7e59865f-d02d-441c-9409-0ed517fcebd7',
    privateKeyPath: process.env.VONAGE_PRIVATE_KEY_PATH || './private.key',
    number: process.env.VONAGE_NUMBER || '',
  },
  callbacks: {
    monitoring: process.env.VONAGE_MONITORING_CALLBACK || 'https://thbtechub.sbs/monitoring-event',
    recording: process.env.VONAGE_RECORDING_CALLBACK || 'https://thbtechub.sbs/recording-event',
    broadcast: process.env.VONAGE_BROADCAST_CALLBACK || 'https://thbtechub.sbs/broadcast-event',
    composer: process.env.VONAGE_COMPOSER_CALLBACK || 'https://thbtechub.sbs/composer-event',
    captions: process.env.VONAGE_CAPTIONS_CALLBACK || 'https://thbtechub.sbs/captions-event',
    sipMonitoring: process.env.VONAGE_SIP_MONITORING_CALLBACK || 'https://thbtechub.sbs/sip-monitoring-event',
  },
  server: {
    port: parseInt(process.env.PORT || '3039'),
    baseUrl: process.env.BASE_URL || 'https://thbtechub.sbs',
  },
  frontend: {
    url: process.env.FRONTEND_URL || 'https://techub-comms.vercel.app',
  },
};
