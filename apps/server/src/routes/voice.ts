import { Router, Request, Response } from 'express';
import { voiceService } from '../services/voiceService';
import { talkAction, ivrMenu, connectAction, conversationAction, recordAction } from '../services/nccoBuilder';

export const voiceRouter = Router();

// Initiate outbound call
voiceRouter.post('/call', async (req: Request, res: Response) => {
  try {
    const { to, from, text, ivr, connectTo, conference } = req.body;
    const fromNumber = from || process.env.VONAGE_NUMBER;

    let ncco;
    if (conference) {
      ncco = [conversationAction(conference)];
    } else if (connectTo) {
      ncco = [connectAction({ type: 'phone', number: connectTo })];
    } else if (ivr) {
      ncco = ivrMenu(ivr.prompt || 'Press 1 for sales, 2 for support.', {
        eventUrl: [`${process.env.BASE_URL}/webhooks/input`],
      });
    } else {
      ncco = [talkAction(text || 'Hello from Techub Comms.')];
    }

    const result = await voiceService.createOutboundCall(
      to,
      fromNumber,
      ncco,
      `${process.env.BASE_URL}/webhooks/answer`,
      `${process.env.BASE_URL}/webhooks/event`
    );
    res.json({ uuid: result.uuid, status: 'call-initiated' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Modify active call (hangup, mute, unmute, transfer)
voiceRouter.put('/call/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    const { action } = req.body;
    await voiceService.modifyCall(uuid, action);
    res.json({ message: `Call ${action} successful` });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Play TTS into active call
voiceRouter.post('/talk/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    const { text, language, voiceName } = req.body;
    await voiceService.playTTS(uuid, text, language, voiceName);
    res.json({ message: 'TTS started' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Stop TTS
voiceRouter.delete('/talk/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    await voiceService.stopTTS(uuid);
    res.json({ message: 'TTS stopped' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Stream audio into call
voiceRouter.post('/stream/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    const { streamUrl } = req.body;
    await voiceService.streamAudio(uuid, streamUrl);
    res.json({ message: 'Audio streaming started' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Stop audio stream
voiceRouter.delete('/stream/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    await voiceService.stopStream(uuid);
    res.json({ message: 'Audio streaming stopped' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Send DTMF
voiceRouter.post('/dtmf/:uuid', async (req: Request, res: Response) => {
  try {
    const { uuid } = req.params;
    const { digits } = req.body;
    await voiceService.sendDTMF(uuid, digits);
    res.json({ message: 'DTMF sent' });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});
