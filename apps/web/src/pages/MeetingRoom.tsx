import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import VideoRoom from '../components/video/VideoRoom';

const API_URL = import.meta.env.VITE_API_URL || '';

export default function MeetingRoom() {
  const { room } = useParams<{ room: string }>();
  const navigate = useNavigate();
  const [sessionData, setSessionData] = useState<{ sessionId: string; token: string; apiKey: string } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!room) return;

    const fetchSession = async () => {
      try {
        const res = await fetch(`${API_URL}/api/video/session/${room}`);
        if (!res.ok) throw new Error('Failed to create session');
        const data = await res.json();
        setSessionData(data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchSession();
  }, [room]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-teal-500 mx-auto mb-4"></div>
          <p className="text-gray-600">Joining meeting room...</p>
        </div>
      </div>
    );
  }

  if (error || !sessionData) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="card p-8 text-center max-w-md">
          <p className="text-red-500 mb-4">{error || 'Failed to join meeting'}</p>
          <button onClick={() => navigate('/')} className="btn-primary">
            Go Back
          </button>
        </div>
      </div>
    );
  }

  return (
    <VideoRoom
      room={room!}
      sessionId={sessionData.sessionId}
      token={sessionData.token}
      apiKey={sessionData.apiKey}
      onLeave={() => navigate('/')}
    />
  );
}
