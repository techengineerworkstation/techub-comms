import { Auth } from '@vonage/auth';
import { Messages } from '@vonage/messages';
import type { SendMessageOptions } from '../types';

class MessageService {
  private credentials: Auth;
  private messages: Messages;

  constructor() {
    this.credentials = new Auth({
      apiKey: process.env.VONAGE_API_KEY!,
      apiSecret: process.env.VONAGE_API_SECRET!,
    });
    this.messages = new Messages(this.credentials);
  }

  async sendSMS(to: string, from: string, text: string) {
    return this.messages.send({
      to: [{ type: 'phone', number: to }],
      from: { type: 'phone', number: from },
      channel: 'sms' as any,
      message_type: 'text' as any,
      text,
    });
  }

  async sendMMS(to: string, from: string, text: string, mediaUrl: string[]) {
    return this.messages.send({
      to: [{ type: 'phone', number: to }],
      from: { type: 'phone', number: from },
      channel: 'mms' as any,
      message_type: 'image' as any,
      image: { url: mediaUrl[0] },
      text,
    });
  }

  async sendWhatsApp(to: string, from: string, text: string) {
    return this.messages.send({
      to: [{ type: 'phone', number: to }],
      from: { type: 'phone', number: from },
      channel: 'whatsapp' as any,
      message_type: 'text' as any,
      text,
    });
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
