import { useState, useEffect } from 'react';
import { Play, Download, Trash2, RefreshCw } from 'lucide-react';

const API_URL = import.meta.env.VITE_API_URL || '';

interface Recording {
  id: string;
  name: string;
  status: string;
  createdAt: string;
  duration?: number;
  url?: string;
}

export default function RecordingsPage() {
  const [recordings, setRecordings] = useState<Recording[]>([]);
  const [loading, setLoading] = useState(false);
  const [roomFilter, setRoomFilter] = useState('');

  const fetchRecordings = async () => {
    if (!roomFilter) return;
    setLoading(true);
    try {
      const res = await fetch(`${API_URL}/api/video/session/${roomFilter}/archives`);
      const data = await res.json();
      setRecordings(data.archives || []);
    } catch (err) {
      console.error('Failed to fetch recordings:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold text-gray-900 mb-8">Recordings</h1>

      <div className="card p-6 mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            placeholder="Enter room name..."
            value={roomFilter}
            onChange={(e) => setRoomFilter(e.target.value)}
            className="input flex-1"
          />
          <button onClick={fetchRecordings} className="btn-primary" disabled={loading}>
            <RefreshCw size={18} className={`inline mr-2 ${loading ? 'animate-spin' : ''}`} />
            Load
          </button>
        </div>
      </div>

      {recordings.length === 0 ? (
        <div className="card p-12 text-center">
          <p className="text-gray-500">
            {roomFilter ? 'No recordings found for this room' : 'Enter a room name to view recordings'}
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {recordings.map((rec) => (
            <div key={rec.id} className="card p-4 flex items-center justify-between">
              <div>
                <h3 className="font-medium text-gray-900">{rec.name || 'Untitled Recording'}</h3>
                <p className="text-sm text-gray-500">
                  {new Date(rec.createdAt).toLocaleString()} &middot; Status: {rec.status}
                  {rec.duration ? ` &middot; ${Math.round(rec.duration)}s` : ''}
                </p>
              </div>
              <div className="flex items-center gap-2">
                {rec.url && (
                  <>
                    <button className="btn-icon text-teal-500 hover:bg-teal-50" title="Play">
                      <Play size={20} />
                    </button>
                    <a href={rec.url} download className="btn-icon text-gray-500 hover:bg-gray-50" title="Download">
                      <Download size={20} />
                    </a>
                  </>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
