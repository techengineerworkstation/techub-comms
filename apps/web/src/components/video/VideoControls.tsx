import { useState } from 'react';
import {
  Mic,
  MicOff,
  Video,
  VideoOff,
  Monitor,
  PhoneOff,
  MessageSquare,
  Users,
  Circle,
  Captions,
} from 'lucide-react';
import { useSessionStore } from '@techub/shared';

interface VideoControlsProps {
  room: string;
  onLeave: () => void;
  onStartRecording: () => void;
  onStopRecording: () => void;
}

export default function VideoControls({ room, onLeave, onStartRecording, onStopRecording }: VideoControlsProps) {
  const {
    isRecording,
    isScreenSharing,
    isChatOpen,
    isParticipantsOpen,
    setScreenSharing,
    toggleChat,
    toggleParticipants,
  } = useSessionStore();

  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(false);

  return (
    <div className="h-20 bg-white border-t border-beige-100 flex items-center justify-center gap-3 px-6">
      <button
        onClick={() => setIsMuted(!isMuted)}
        className={`btn-icon ${isMuted ? 'bg-red-100 text-red-500' : 'text-gray-600 hover:bg-beige-100'}`}
        title={isMuted ? 'Unmute' : 'Mute'}
      >
        {isMuted ? <MicOff size={22} /> : <Mic size={22} />}
      </button>

      <button
        onClick={() => setIsVideoOff(!isVideoOff)}
        className={`btn-icon ${isVideoOff ? 'bg-red-100 text-red-500' : 'text-gray-600 hover:bg-beige-100'}`}
        title={isVideoOff ? 'Turn on camera' : 'Turn off camera'}
      >
        {isVideoOff ? <VideoOff size={22} /> : <Video size={22} />}
      </button>

      <button
        onClick={() => setScreenSharing(!isScreenSharing)}
        className={`btn-icon ${isScreenSharing ? 'bg-teal-100 text-teal-600' : 'text-gray-600 hover:bg-beige-100'}`}
        title="Screen share"
      >
        <Monitor size={22} />
      </button>

      <button
        onClick={isRecording ? onStopRecording : onStartRecording}
        className={`btn-icon ${isRecording ? 'bg-red-500 text-white animate-pulse' : 'text-gray-600 hover:bg-beige-100'}`}
        title={isRecording ? 'Stop recording' : 'Start recording'}
      >
        <Circle size={22} fill={isRecording ? 'currentColor' : 'none'} />
      </button>

      <button
        onClick={toggleParticipants}
        className={`btn-icon ${isParticipantsOpen ? 'bg-teal-100 text-teal-600' : 'text-gray-600 hover:bg-beige-100'}`}
        title="Participants"
      >
        <Users size={22} />
      </button>

      <button
        onClick={toggleChat}
        className={`btn-icon ${isChatOpen ? 'bg-teal-100 text-teal-600' : 'text-gray-600 hover:bg-beige-100'}`}
        title="Chat"
      >
        <MessageSquare size={22} />
      </button>

      <button
        className="btn-icon text-gray-600 hover:bg-beige-100"
        title="Live captions"
      >
        <Captions size={22} />
      </button>

      <div className="mx-4 h-8 w-px bg-beige-200" />

      <button
        onClick={onLeave}
        className="bg-red-500 hover:bg-red-600 text-white p-3 rounded-full transition-colors duration-200"
        title="Leave meeting"
      >
        <PhoneOff size={22} />
      </button>
    </div>
  );
}

