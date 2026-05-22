import { Auth } from '@vonage/auth';
import { Voice } from '@vonage/voice';
import { config } from '../config';
import type { NCCOAction } from '../types';

class VoiceService {
  private credentials: Auth;
  private voice: Voice;

  constructor() {
    this.credentials = new Auth({
      applicationId: config.vonage.applicationId,
      privateKey: config.vonage.privateKey,
    });
    this.voice = new Voice(this.credentials);
  }

  async createOutboundCall(
    to: string,
    from: string,
    ncco: NCCOAction[],
    answerUrl?: string,
    eventUrl?: string
  ) {
    const payload: Record<string, unknown> = {
      to: [{ type: 'phone', number: to }],
      from: { type: 'phone', number: from },
    };

    if (ncco.length > 0) {
      payload.ncco = ncco;
    }
    if (answerUrl) {
      payload.answer_url = [answerUrl];
      payload.answer_method = 'POST';
    }
    if (eventUrl) {
      payload.event_url = [eventUrl];
      payload.event_method = 'POST';
    }

    return this.voice.createOutboundCall(payload as any);
  }

  async modifyCall(uuid: string, action: string) {
    return (this.voice as any).updateCall(uuid, { action });
  }

  async playTTS(uuid: string, text: string, language = 'en-US', voiceName = 'Amy') {
    return (this.voice as any).sendTTS(uuid, {
      text,
      language,
      voiceName,
      level: 0,
      loop: 1,
    });
  }

  async stopTTS(uuid: string) {
    return (this.voice as any).stopTTS(uuid);
  }

  async streamAudio(uuid: string, streamUrl: string[]) {
    return (this.voice as any).sendAudio(uuid, {
      streamUrl,
      loop: 1,
      level: 0,
    });
  }

  async stopStream(uuid: string) {
    return (this.voice as any).stopAudio(uuid);
  }

  async sendDTMF(uuid: string, digits: string) {
    return (this.voice as any).sendDTMF(uuid, { digits });
  }
}

export const voiceService = new VoiceService();
