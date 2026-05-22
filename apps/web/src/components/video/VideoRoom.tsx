import { useEffect, useRef, useState, useCallback } from 'react';
import VideoControls from './VideoControls';
import ParticipantList from './ParticipantList';
import ChatPanel from './ChatPanel';
import { useSessionStore } from '@techub/shared';

const API_URL = import.meta.env.VITE_API_URL || '';

interface VideoRoomProps {
  room: string;
  sessionId: string;
  token: string;
  apiKey: string;
  onLeave: () => void;
}

export default function VideoRoom({ room, sessionId, token, apiKey, onLeave }: VideoRoomProps) {
  const sessionRef = useRef<any>(null);
  const publisherRef = useRef<any>(null);
  const subscriberRefs = useRef<Map<string, HTMLDivElement>>(new Map());
  const [isConnected, setIsConnected] = useState(false);
  const [participants, setParticipants] = useState<any[]>([]);
  const { isChatOpen, isParticipantsOpen, isRecording, isScreenSharing, setScreenSharing, setCaptionsId } = useSessionStore();
  const screenSharingPublisherRef = useRef<any>(null);

  const setSubscriberRef = useCallback((id: string, el: HTMLDivElement | null) => {
    if (el) {
      subscriberRefs.current.set(id, el);
    } else {
      subscriberRefs.current.delete(id);
    }
  }, []);

  const handleStreamCreated = useCallback((event: any) => {
    const stream = event.stream;
    const connectionId = stream.connection.connectionId;
    setParticipants((prev) => [...prev, { stream, id: connectionId }]);

    // Subscribe to the remote stream after DOM element is available
    setTimeout(() => {
      const container = subscriberRefs.current.get(connectionId);
      if (container && sessionRef.current) {
        const OT = (window as any).OT;
        sessionRef.current.subscribe(stream, container, {
          insertMode: 'replace',
          width: '100%',
          height: '100%',
          style: { buttonDisplayMode: 'off' },
        });
      }
    }, 100);
  }, []);

  const handleStreamDestroyed = useCallback((event: any) => {
    setParticipants((prev) => prev.filter((p) => p.id !== event.stream.connection.connectionId));
  }, []);

  useEffect(() => {
    if (!sessionId || !token) return;

    const initSession = async () => {
      try {
        const OT = (window as any).OT;
        if (!OT) {
          console.error('OpenTok SDK not loaded');
          return;
        }

        const session = OT.initSession(apiKey, sessionId);

        session.on('streamCreated', handleStreamCreated);
        session.on('streamDestroyed', handleStreamDestroyed);

        // Handle chat signals
        session.on('signal:chat', (event: any) => {
          if (event.from?.connectionId !== session.connection?.connectionId) {
            const data = JSON.parse(event.data || '{}');
            useSessionStore.getState().addChatMessage?.({
              id: Date.now().toString(),
              sender: data.sender || 'Participant',
              text: data.text,
              timestamp: new Date(),
              isOwn: false,
            });
          }
        });

        session.connect(token, (error: any) => {
          if (error) {
            console.error('Session connection error:', error);
            return;
          }
          setIsConnected(true);

          const publisher = OT.initPublisher('publisher', {
            insertMode: 'append',
            width: '100%',
            height: '100%',
            style: { buttonDisplayMode: 'off' },
            name: 'You',
          });

          session.publish(publisher);
          publisherRef.current = publisher;
        });

        sessionRef.current = session;
      } catch (err) {
        console.error('Failed to initialize session:', err);
      }
    };

    initSession();

    return () => {
      if (sessionRef.current) {
        sessionRef.current.disconnect();
      }
    };
  }, [sessionId, token, apiKey, handleStreamCreated, handleStreamDestroyed]);

  const handleStartRecording = async () => {
    try {
      await fetch(`${API_URL}/api/video/session/${room}/startArchive`, { method: 'POST' });
      useSessionStore.getState().setRecording(true);
    } catch (err) {
      console.error('Failed to start recording:', err);
    }
  };

  const handleStopRecording = async () => {
    try {
      const res = await fetch(`${API_URL}/api/video/session/${room}/archives`);
      const data = await res.json();
      if (data.archives?.length > 0) {
        const latest = data.archives[0];
        await fetch(`${API_URL}/api/video/session/${room}/${latest.id}/stopArchive`, { method: 'POST' });
      }
      useSessionStore.getState().setRecording(false);
    } catch (err) {
      console.error('Failed to stop recording:', err);
    }
  };

  const handleToggleScreenShare = async () => {
    const OT = (window as any).OT;
    if (!OT || !sessionRef.current || !publisherRef.current) return;

    if (isScreenSharing) {
      // Stop screen sharing — republish camera
      if (screenSharingPublisherRef.current) {
        sessionRef.current.unpublish(screenSharingPublisherRef.current);
        screenSharingPublisherRef.current.destroy();
        screenSharingPublisherRef.current = null;
      }
      const cameraPublisher = OT.initPublisher('publisher', {
        insertMode: 'replace',
        width: '100%',
        height: '100%',
        style: { buttonDisplayMode: 'off' },
        name: 'You',
      });
      sessionRef.current.publish(cameraPublisher);
      publisherRef.current = cameraPublisher;
      setScreenSharing(false);
    } else {
      // Start screen sharing
      try {
        const screenPublisher = OT.initPublisher('publisher', {
          insertMode: 'replace',
          width: '100%',
          height: '100%',
          videoSource: 'screen',
          publishAudio: true,
          style: { buttonDisplayMode: 'off' },
        });

        screenPublisher.on('accessDenied', () => {
          console.warn('Screen sharing access denied');
          setScreenSharing(false);
        });

        screenPublisher.on('destroyed', () => {
          // Screen share ended by user (e.g., browser stop button)
          if (screenSharingPublisherRef.current) {
            screenSharingPublisherRef.current = null;
            const cameraPublisher = OT.initPublisher('publisher', {
              insertMode: 'replace',
              width: '100%',
              height: '100%',
              style: { buttonDisplayMode: 'off' },
              name: 'You',
            });
            sessionRef.current?.publish(cameraPublisher);
            publisherRef.current = cameraPublisher;
            setScreenSharing(false);
          }
        });

        sessionRef.current.unpublish(publisherRef.current);
        sessionRef.current.publish(screenPublisher);
        screenSharingPublisherRef.current = screenPublisher;
        publisherRef.current = screenPublisher;
        setScreenSharing(true);
      } catch (err) {
        console.error('Failed to start screen sharing:', err);
        setScreenSharing(false);
      }
    }
  };

  const handleToggleCaptions = async () => {
    const { captionsId } = useSessionStore.getState();
    if (captionsId) {
      try {
        await fetch(`${API_URL}/api/video/session/${room}/${captionsId}/disableCaptions`, { method: 'POST' });
        setCaptionsId(null);
      } catch (err) {
        console.error('Failed to disable captions:', err);
      }
    } else {
      try {
        const res = await fetch(`${API_URL}/api/video/session/${room}/enableCaptions`, { method: 'POST' });
        const data = await res.json();
        if (data.captionsId) {
          setCaptionsId(data.captionsId);
        }
      } catch (err) {
        console.error('Failed to enable captions:', err);
      }
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 flex">
        {/* Video Grid */}
        <div className="flex-1 p-4">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 h-full">
            {/* Local Publisher */}
            <div className="relative bg-gray-900 rounded-xl overflow-hidden aspect-video">
              <div id="publisher" className="w-full h-full" />
              <div className="absolute bottom-3 left-3 bg-black/50 text-white text-sm px-2 py-1 rounded">
                You
              </div>
            </div>

            {/* Remote Subscribers */}
            {participants.map((p) => (
              <div key={p.id} className="relative bg-gray-900 rounded-xl overflow-hidden aspect-video">
                <div
                  ref={(el) => setSubscriberRef(p.id, el)}
                  id={`subscriber-${p.id}`}
                  className="w-full h-full"
                />
                <div className="absolute bottom-3 left-3 bg-black/50 text-white text-sm px-2 py-1 rounded">
                  {p.stream?.name || 'Participant'}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Side Panels */}
        {isParticipantsOpen && (
          <div className="w-80 border-l border-beige-100 bg-white">
            <ParticipantList participants={participants} />
          </div>
        )}
        {isChatOpen && (
          <div className="w-80 border-l border-beige-100 bg-white">
            <ChatPanel session={sessionRef.current} />
          </div>
        )}
      </div>

      {/* Controls */}
      <VideoControls
        room={room}
        onLeave={onLeave}
        onStartRecording={handleStartRecording}
        onStopRecording={handleStopRecording}
        publisherRef={publisherRef}
        onToggleScreenShare={handleToggleScreenShare}
        onToggleCaptions={handleToggleCaptions}
      />
    </div>
  );
}
