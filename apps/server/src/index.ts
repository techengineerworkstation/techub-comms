import 'dotenv/config';
import express from 'express';
import cors from 'cors';
import { config } from './config';
import { videoRouter } from './routes/video';
import { voiceRouter } from './routes/voice';
import { messageRouter } from './routes/messages';
import { webhookRouter } from './routes/webhooks';

const app = express();
const PORT = config.server.port;

app.use(cors({ origin: [config.frontend.url, 'https://thbtechub.sbs'], credentials: true }));
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// API Routes
app.use('/api/video', videoRouter);
app.use('/api/voice', voiceRouter);
app.use('/api/message', messageRouter);

// Voice Webhooks (under /webhooks prefix)
app.use('/webhooks', webhookRouter);

// Vonage Video API Callbacks (mounted at root per Vonage dashboard config)
app.use('/', webhookRouter);

// Health check
app.get('/health', (_req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.listen(PORT, () => {
  console.log(`Techub Comms Server running on port ${PORT}`);
});

export default app;
