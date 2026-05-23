import { Auth } from '@vonage/auth';
import { Messages } from '@vonage/messages';
import { config } from '../config';
import type { SendMessageOptions } from '../types';

class MessageService {
  private credentials: Auth;
  private messages: Messages;

  constructor() {
    this.credentials = new Auth({
      apiKey: config.vonage.apiKey,
      apiSecret: config.vonage.apiSecret,
    });
    this.messages = new Messages(this.credentials);
  }

  async sendSMS(to: string, from: string, text: string) {
    return this.messages.send({
      to,
      from,
      channel: 'sms',
      messageType: 'text',
      text,
    } as any);
  }

  async sendMMS(to: string, from: string, text: string, mediaUrl: string[]) {
    return this.messages.send({
      to,
      from,
      channel: 'mms',
      messageType: 'image',
      image: { url: mediaUrl[0] },
    } as any);
  }

  async sendWhatsApp(to: string, from: string, text: string) {
    return this.messages.send({
      to,
      from,
      channel: 'whatsapp',
      messageType: 'text',
      text,
    } as any);
  }

  async send(options: SendMessageOptions) {
    switch (options.channel) {
      case 'sms':
        return this.sendSMS(options.to, options.from, options.text || '');
      case 'mms':
        return this.sendMMS(options.to, options.from, options.text || '', options.mediaUrl || []);
      case 'whatsapp':
        return this.sendWhatsApp(options.to, options.from, options.text || '');
    }
  }
}

export const messageService = new MessageService();
