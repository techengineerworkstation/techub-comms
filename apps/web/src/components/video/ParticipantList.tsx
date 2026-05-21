import { Mic, MicOff, Video, VideoOff } from 'lucide-react';

interface ParticipantListProps {
  participants: any[];
}

export default function ParticipantList({ participants }: ParticipantListProps) {
  return (
    <div className="h-full flex flex-col">
      <div className="p-4 border-b border-beige-100">
        <h3 className="font-semibold text-gray-900">Participants ({participants.length + 1})</h3>
      </div>
      <div className="flex-1 overflow-auto p-4 space-y-3">
        {/* Local user */}
        <div className="flex items-center justify-between p-3 bg-beige-50 rounded-lg">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-teal-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
              Y
            </div>
            <span className="font-medium text-gray-900">You</span>
          </div>
          <div className="flex items-center gap-2">
            <Mic size={16} className="text-gray-500" />
            <Video size={16} className="text-gray-500" />
          </div>
        </div>

        {/* Remote participants */}
        {participants.map((p, i) => (
          <div key={p.id} className="flex items-center justify-between p-3 hover:bg-beige-50 rounded-lg">
            <div className="flex items-center gap-3">
              <div className="w-8 h-8 bg-beige-400 rounded-full flex items-center justify-center text-white text-sm font-medium">
                {String.fromCharCode(65 + i)}
              </div>
              <span className="text-gray-900">Participant {i + 1}</span>
            </div>
            <div className="flex items-center gap-2">
              <Mic size={16} className="text-gray-400" />
              <Video size={16} className="text-gray-400" />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
