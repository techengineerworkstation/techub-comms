import { Router, Request, Response, NextFunction } from 'express';
import { talkAction, inputAction, connectAction, recordAction } from '../services/nccoBuilder';
import { config } from '../config';

export const webhookRouter = Router();

// Middleware to validate Vonage Video API callback verification tokens
function validateVerificationToken(tokenKey: keyof typeof config.verificationTokens) {
  return (req: Request, res: Response, next: NextFunction) => {
    const expectedToken = config.verificationTokens[tokenKey];
    if (!expectedToken) {
      // No token configured — skip validation (dev mode)
      return next();
    }
    const receivedToken = req.body?.verificationToken || req.headers['x-vonage-verification-token'];
    if (receivedToken && receivedToken !== expectedToken) {
      console.warn(`[Webhook Auth] Invalid verification token for ${tokenKey}`);
      return res.status(403).json({ error: 'Invalid verification token' });
    }
    next();
  };
}

// Answer webhook — called when outbound call is answered
webhookRouter.all('/answer', (req: Request, res: Response) => {
  const { conversation_uuid, to, from } = req.body;

  // Default IVR menu
  const ncco = [
    talkAction('Welcome to Techub Comms. Press 1 for sales, 2 for support, or 3 for billing.', {
      bargeIn: false,
    }),
    inputAction({
      dtmf: { maxDigits: 1, timeOut: 5 },
      eventUrl: [`${process.env.BASE_URL}/webhooks/input`],
    }),
  ];

  res.json(ncco);
});

// Input webhook — handles DTMF and speech input
webhookRouter.post('/input', (req: Request, res: Response) => {
  const { dtmf, speech } = req.body;

  if (dtmf?.digits) {
    const digit = dtmf.digits;
    let ncco;

    switch (digit) {
      case '1':
        ncco = [
          talkAction('Connecting you to sales. Please hold.'),
          connectAction({ type: 'phone', number: '15551111111' }),
        ];
        break;
      case '2':
        ncco = [
          talkAction('Connecting you to support. Please hold.'),
          connectAction({ type: 'phone', number: '15552222222' }),
        ];
        break;
      case '3':
        ncco = [
          talkAction('Please describe your billing issue after the tone.'),
          recordAction({
            timeOut: 30,
            endOnSilence: 3,
            eventUrl: [`${process.env.BASE_URL}/webhooks/recording`],
          }),
        ];
        break;
      default:
        ncco = [talkAction('Invalid option. Goodbye.')];
    }

    res.json(ncco);
  } else if (speech?.results?.[0]?.text) {
    const speechText = speech.results[0].text;
    res.json([
      talkAction(`You said: ${speechText}. Let me help you with that.`),
    ]);
  } else {
    res.json([talkAction('We did not receive your input. Goodbye.')]);
  }
});

// Event webhook — receives call status events
webhookRouter.post('/event', (req: Request, res: Response) => {
  const { status, uuid, conversation_uuid, direction, start_time, end_time, duration, price } = req.body;
  console.log(`[Voice Event] Call ${uuid}: ${status} (${direction})`);
  res.status(200).send('OK');
});

// Recording webhook — receives recording completion events
webhookRouter.post('/recording', (req: Request, res: Response) => {
  const { recording_uuid, recording_url, start_time, end_time, size, format } = req.body;
  console.log(`[Recording] ${recording_uuid}: ${recording_url} (${format}, ${size} bytes)`);
  res.status(200).send('OK');
});

// ─── Vonage Video API Callbacks ─────────────────────────────────────────────

// Session monitoring callback
webhookRouter.post('/monitoring-event', validateVerificationToken('monitoring'), (req: Request, res: Response) => {
  console.log('[Monitoring Event]', JSON.stringify(req.body, null, 2));
  res.status(200).send('OK');
});

// Archive recording callback
webhookRouter.post('/recording-event', validateVerificationToken('recording'), (req: Request, res: Response) => {
  const { status, id, name, sessionId, createdAt, duration, url, reason } = req.body;
  console.log(`[Recording Event] Archive ${id}: ${status} - ${name}`);
  res.status(200).send('OK');
});

// Broadcast status callback
webhookRouter.post('/broadcast-event', validateVerificationToken('broadcast'), (req: Request, res: Response) => {
  const { status, id, sessionId, broadcastUrls } = req.body;
  console.log(`[Broadcast Event] ${id}: ${status}`);
  res.status(200).send('OK');
});

// Experience Composer callback
webhookRouter.post('/composer-event', validateVerificationToken('composer'), (req: Request, res: Response) => {
  const { status, id, sessionId, name, url, reason } = req.body;
  console.log(`[Composer Event] ${id}: ${status} - ${name}`);
  res.status(200).send('OK');
});

// Captions callback
webhookRouter.post('/captions-event', validateVerificationToken('captions'), (req: Request, res: Response) => {
  const { captionsId, status, sessionId, reason } = req.body;
  console.log(`[Captions Event] ${captionsId}: ${status}`);
  res.status(200).send('OK');
});

// SIP monitoring callback
webhookRouter.post('/sip-monitoring-event', validateVerificationToken('sipMonitoring'), (req: Request, res: Response) => {
  console.log('[SIP Monitoring Event]', JSON.stringify(req.body, null, 2));
  res.status(200).send('OK');
});
