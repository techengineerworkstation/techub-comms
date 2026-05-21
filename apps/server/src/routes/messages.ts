import { Router, Request, Response } from 'express';
import { messageService } from '../services/messageService';

export const messageRouter = Router();

// Send SMS
messageRouter.post('/send', async (req: Request, res: Response) => {
  try {
    const { to, from, text } = req.body;
    const result = await messageService.sendSMS(to, from || process.env.VONAGE_NUMBER!, text);
    res.json({ messageId: result.messageUuid, status: 'sent' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Send MMS
messageRouter.post('/send-mms', async (req: Request, res: Response) => {
  try {
    const { to, from, text, mediaUrl } = req.body;
    const result = await messageService.sendMMS(to, from || process.env.VONAGE_NUMBER!, text, mediaUrl);
    res.json({ messageId: result.messageUuid, status: 'sent' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Send WhatsApp
messageRouter.post('/send-whatsapp', async (req: Request, res: Response) => {
  try {
    const { to, from, text } = req.body;
    const result = await messageService.sendWhatsApp(to, from || process.env.VONAGE_NUMBER!, text);
    res.json({ messageId: result.messageUuid, status: 'sent' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});
