import { Router, Request, Response } from 'express';
import { videoService } from '../services/videoService';

export const videoRouter = Router();

// Get or create session + token for a room
videoRouter.get('/session/:room', async (req: Request, res: Response) => {
  try {
    const { room } = req.params;
    const sessionId = await videoService.getOrCreateSession(room);
    const { token, apiKey } = videoService.generateToken(sessionId);
    res.json({ sessionId, token, apiKey });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Start recording
videoRouter.post('/session/:room/startArchive', async (req: Request, res: Response) => {
  try {
    const { room } = req.params;
    const sessionId = await videoService.getOrCreateSession(room);
    const archive = await videoService.startArchive(room, sessionId);
    res.json({ archiveId: archive.id, status: 200 });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Stop recording
videoRouter.post('/session/:room/:archiveId/stopArchive', async (req: Request, res: Response) => {
  try {
    const { archiveId } = req.params;
    const result = await videoService.stopArchive(archiveId);
    res.json({ message: result, status: 200 });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// List recordings
videoRouter.get('/session/:room/archives', async (req: Request, res: Response) => {
  try {
    const { room } = req.params;
    const sessionId = await videoService.getOrCreateSession(room);
    const archives = await videoService.listArchives(sessionId);
    res.json({ archives, status: 200 });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Enable captions
videoRouter.post('/session/:room/enableCaptions', async (req: Request, res: Response) => {
  try {
    const { room } = req.params;
    const sessionId = await videoService.getOrCreateSession(room);
    const captions = await videoService.enableCaptions(sessionId);
    res.json({ captionsId: captions, status: 200 });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Disable captions
videoRouter.post('/session/:room/:captionsId/disableCaptions', async (req: Request, res: Response) => {
  try {
    const { captionsId } = req.params;
    const result = await videoService.disableCaptions(captionsId);
    res.json({ message: result, status: 200 });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});
