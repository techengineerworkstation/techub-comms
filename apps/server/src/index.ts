import 'dotenv/config';
import express from 'express';
import cors from 'cors';
import { videoRouter } from './routes/video';
import { voiceRouter } from './routes/voice';
import { messageRouter } from './routes/messages';
import { webhookRouter } from './routes/webhooks';

const app = express();
const PORT = process.env.PORT || 3039;

app.use(cors({ origin: true, credentials: true }));
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// API Routes
app.use('/api/video', videoRouter);
app.use('/api/voice', voiceRouter);
app.use('/api/message', messageRouter);

// Webhooks
app.use('/webhooks', webhookRouter);

// Health check
app.get('/health', (_req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.listen(PORT, () => {
  console.log(`Techub Comms Server running on port ${PORT}`);
});

export default app;
