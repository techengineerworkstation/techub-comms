import { Auth } from '@vonage/auth';
import {
  Video,
  MediaMode,
  LayoutType,
  Resolution,
  type SingleArchiveResponse,
  type EnableCaptionResponse,
} from '@vonage/video';
import { config } from '../config';

class VideoService {
  private credentials: Auth;
  private video: Video;
  private sessions: Map<string, string> = new Map();

  constructor() {
    this.credentials = new Auth({
      applicationId: config.vonage.applicationId,
      privateKey: config.vonage.privateKey,
    });
    this.video = new Video(this.credentials);
  }

  async getOrCreateSession(roomName: string): Promise<string> {
    const existing = this.sessions.get(roomName);
    if (existing) return existing;

    const { sessionId } = await this.video.createSession({
      mediaMode: MediaMode.ROUTED,
    });
    this.sessions.set(roomName, sessionId);
    return sessionId;
  }

  generateToken(sessionId: string): { token: string; apiKey: string } {
    const token = this.video.generateClientToken(sessionId, {
      role: 'moderator',
    });
    return { token, apiKey: process.env.VONAGE_APPLICATION_ID! };
  }

  async startArchive(roomName: string, sessionId: string): Promise<SingleArchiveResponse> {
    return this.video.startArchive(sessionId, {
      name: roomName,
      resolution: Resolution.FHD_LANDSCAPE,
      layout: {
        type: LayoutType.BEST_FIT,
        screenshareType: 'horizontalPresentation',
      },
    });
  }

  async stopArchive(archiveId: string): Promise<string> {
    await this.video.stopArchive(archiveId);
    return 'Archive stopped successfully';
  }

  async listArchives(sessionId: string): Promise<SingleArchiveResponse[]> {
    const archives = await this.video.searchArchives({ sessionId });
    return archives.items;
  }

  async enableCaptions(sessionId: string): Promise<EnableCaptionResponse> {
    const { token } = this.generateToken(sessionId);
    return this.video.enableCaptions(sessionId, token, {
      languageCode: 'en-US',
      maxDuration: 1800,
      partialCaptions: 'true',
    });
  }

  async disableCaptions(captionsId: string): Promise<string> {
    await this.video.disableCaptions(captionsId);
    return 'Captions stopped successfully';
  }
}

export const videoService = new VideoService();
