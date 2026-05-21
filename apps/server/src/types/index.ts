export interface VideoSession {
  sessionId: string;
  token: string;
  apiKey: string;
  captionsId?: string;
}

export interface Archive {
  id: string;
  name: string;
  sessionId: string;
  status: string;
  createdAt: string;
  duration?: number;
  url?: string;
}

export interface VoiceCall {
  uuid: string;
  conversationUuid: string;
  status: string;
  direction: 'inbound' | 'outbound';
  from: string;
  to: string;
}

export interface NCCOAction {
  action: string;
  [key: string]: unknown;
}

export interface SendMessageOptions {
  to: string;
  from: string;
  channel: 'sms' | 'whatsapp' | 'mms';
  text?: string;
  mediaUrl?: string[];
}
