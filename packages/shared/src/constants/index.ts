export const API_ROUTES = {
  VIDEO: {
    SESSION: (room: string) => `/api/video/session/${room}`,
    START_ARCHIVE: (room: string) => `/api/video/session/${room}/startArchive`,
    STOP_ARCHIVE: (room: string, archiveId: string) => `/api/video/session/${room}/${archiveId}/stopArchive`,
    ARCHIVES: (room: string) => `/api/video/session/${room}/archives`,
    ENABLE_CAPTIONS: (room: string) => `/api/video/session/${room}/enableCaptions`,
    DISABLE_CAPTIONS: (room: string, captionsId: string) => `/api/video/session/${room}/${captionsId}/disableCaptions`,
  },
  VOICE: {
    CALL: '/api/voice/call',
    MODIFY_CALL: (uuid: string) => `/api/voice/call/${uuid}`,
    TALK: (uuid: string) => `/api/voice/talk/${uuid}`,
    STREAM: (uuid: string) => `/api/voice/stream/${uuid}`,
    DTMF: (uuid: string) => `/api/voice/dtmf/${uuid}`,
  },
  MESSAGE: {
    SEND: '/api/message/send',
    SEND_MMS: '/api/message/send-mms',
    SEND_WHATSAPP: '/api/message/send-whatsapp',
  },
} as const;

export const VONAGE_DEFAULTS = {
  LANGUAGE: 'en-US',
  VOICE_NAME: 'Amy',
  MAX_PARTICIPANTS: 6,
  RECORDING_FORMAT: 'mp3',
  RECORDING_TIMEOUT: 60,
} as const;
