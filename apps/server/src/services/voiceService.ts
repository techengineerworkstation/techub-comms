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
    return this.voice.updateCall(uuid, { action } as any);
  }

  async playTTS(uuid: string, text: string, language = 'en-US', voiceName = 'Amy') {
    const voice = this.voice as Record<string, any>;
    if (typeof voice.sendTTS === 'function') {
      return voice.sendTTS(uuid, { text, language, voiceName, level: 0, loop: 1 });
    }
    throw new Error('TTS not supported by current Vonage Voice SDK version');
  }

  async stopTTS(uuid: string) {
    const voice = this.voice as Record<string, any>;
    if (typeof voice.stopTTS === 'function') {
      return voice.stopTTS(uuid);
    }
    throw new Error('stopTTS not supported by current Vonage Voice SDK version');
  }

  async streamAudio(uuid: string, streamUrl: string[]) {
    const voice = this.voice as Record<string, any>;
    if (typeof voice.sendAudio === 'function') {
      return voice.sendAudio(uuid, { streamUrl, loop: 1, level: 0 });
    }
    throw new Error('Audio streaming not supported by current Vonage Voice SDK version');
  }

  async stopStream(uuid: string) {
    const voice = this.voice as Record<string, any>;
    if (typeof voice.stopAudio === 'function') {
      return voice.stopAudio(uuid);
    }
    throw new Error('stopAudio not supported by current Vonage Voice SDK version');
  }

  async sendDTMF(uuid: string, digits: string) {
    const voice = this.voice as Record<string, any>;
    if (typeof voice.sendDTMF === 'function') {
      return voice.sendDTMF(uuid, { digits });
    }
    throw new Error('DTMF not supported by current Vonage Voice SDK version');
  }
}

export const voiceService = new VoiceService();
