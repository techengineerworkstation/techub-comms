import { useEffect, useRef, useState, useCallback } from 'react';
import VideoControls from './VideoControls';
import ParticipantList from './ParticipantList';
import ChatPanel from './ChatPanel';
import { useSessionStore } from '@techub/shared';

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
  const [isConnected, setIsConnected] = useState(false);
  const [participants, setParticipants] = useState<any[]>([]);
  const { isChatOpen, isParticipantsOpen, isRecording, isScreenSharing } = useSessionStore();

  const handleStreamCreated = useCallback((event: any) => {
    setParticipants((prev) => [...prev, { stream: event.stream, id: event.stream.connection.connectionId }]);
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
      await fetch(`/api/video/session/${room}/startArchive`, { method: 'POST' });
      useSessionStore.getState().setRecording(true);
    } catch (err) {
      console.error('Failed to start recording:', err);
    }
  };

  const handleStopRecording = async () => {
    try {
      const res = await fetch(`/api/video/session/${room}/archives`);
      const data = await res.json();
      if (data.archives?.length > 0) {
        const latest = data.archives[0];
        await fetch(`/api/video/session/${room}/${latest.id}/stopArchive`, { method: 'POST' });
      }
      useSessionStore.getState().setRecording(false);
    } catch (err) {
      console.error('Failed to stop recording:', err);
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
                <div id={`subscriber-${p.id}`} className="w-full h-full" />
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
            <ChatPanel />
          </div>
        )}
      </div>

      {/* Controls */}
      <VideoControls
        room={room}
        onLeave={onLeave}
        onStartRecording={handleStartRecording}
        onStopRecording={handleStopRecording}
      />
    </div>
  );
}
