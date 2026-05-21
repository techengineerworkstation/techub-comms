import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Video, Phone, MessageSquare, Users } from 'lucide-react';

export default function LandingPage() {
  const [roomName, setRoomName] = useState('');
  const navigate = useNavigate();

  const handleJoinRoom = (e: React.FormEvent) => {
    e.preventDefault();
    if (roomName.trim()) {
      navigate(`/meeting/${roomName.trim()}`);
    }
  };

  const quickActions = [
    { label: 'Start Video Call', icon: Video, color: 'bg-teal-500', action: () => navigate(`/meeting/room-${Date.now()}`) },
    { label: 'Voice Call', icon: Phone, color: 'bg-beige-500', action: () => navigate('/voice') },
    { label: 'Send Message', icon: MessageSquare, color: 'bg-teal-600', action: () => navigate('/messages') },
    { label: 'Group Meeting', icon: Users, color: 'bg-beige-600', action: () => navigate('/meeting/team-standup') },
  ];

  return (
    <div className="max-w-4xl mx-auto">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold text-gray-900 mb-4">
          Welcome to <span className="text-teal-500">Techub Comms</span>
        </h1>
        <p className="text-lg text-gray-600">
          Connect with your team through video, voice, and messaging
        </p>
      </div>

      {/* Join Room */}
      <div className="card p-8 mb-8">
        <h2 className="text-xl font-semibold mb-4">Join a Meeting</h2>
        <form onSubmit={handleJoinRoom} className="flex gap-4">
          <input
            type="text"
            placeholder="Enter room name..."
            value={roomName}
            onChange={(e) => setRoomName(e.target.value)}
            className="input flex-1"
          />
          <button type="submit" className="btn-primary">
            Join Room
          </button>
        </form>
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {quickActions.map((action) => {
          const Icon = action.icon;
          return (
            <button
              key={action.label}
              onClick={action.action}
              className="card p-6 hover:shadow-md transition-shadow duration-200 text-left group"
            >
              <div className={`${action.color} w-12 h-12 rounded-lg flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-200`}>
                <Icon size={24} className="text-white" />
              </div>
              <h3 className="font-semibold text-gray-900">{action.label}</h3>
              <p className="text-sm text-gray-500 mt-1">Click to start</p>
            </button>
          );
        })}
      </div>
    </div>
  );
}
