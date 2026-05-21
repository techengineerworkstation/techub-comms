export interface VideoSession {
  sessionId: string;
  token: string;
  apiKey: string;
  captionsId?: string;
}

export interface Participant {
  id: string;
  connectionId: string;
  name?: string;
  hasVideo: boolean;
  hasAudio: boolean;
  isScreenSharing: boolean;
}

export interface Archive {
  id: string;
  name: string;
  sessionId: string;
  status: 'started' | 'stopped' | 'uploaded' | 'deleted' | 'failed';
  createdAt: string;
  duration?: number;
  url?: string;
  size?: number;
}

export interface VoiceCall {
  uuid: string;
  conversationUuid?: string;
  status: string;
  direction: 'inbound' | 'outbound';
  from: string;
  to: string;
}

export interface Message {
  id: string;
  to: string;
  from: string;
  channel: 'sms' | 'mms' | 'whatsapp';
  text: string;
  mediaUrl?: string[];
  status: 'sent' | 'delivered' | 'failed' | 'received';
  timestamp: string;
  direction: 'inbound' | 'outbound';
}

export interface SendMessageOptions {
  to: string;
  from?: string;
  channel: 'sms' | 'mms' | 'whatsapp';
  text: string;
  mediaUrl?: string[];
}
