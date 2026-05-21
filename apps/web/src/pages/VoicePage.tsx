import { useState } from 'react';
import { Phone, PhoneOff, Mic, MicOff, Hash } from 'lucide-react';

export default function VoicePage() {
  const [phoneNumber, setPhoneNumber] = useState('');
  const [callText, setCallText] = useState('');
  const [activeCall, setActiveCall] = useState<{ uuid: string } | null>(null);
  const [isMuted, setIsMuted] = useState(false);
  const [callStatus, setCallStatus] = useState<string | null>(null);
  const [dtmf, setDtmf] = useState('');

  const handleCall = async (type: 'simple' | 'ivr' | 'conference') => {
    try {
      setCallStatus('Initiating call...');
      const body: any = { to: phoneNumber };
      if (type === 'ivr') body.ivr = { prompt: 'Welcome. Press 1 for sales, 2 for support.' };
      if (type === 'conference') body.conference = 'techub-room';
      if (type === 'simple') body.text = callText || 'Hello from Techub Comms.';

      const res = await fetch('/api/voice/call', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      const data = await res.json();
      setActiveCall({ uuid: data.uuid });
      setCallStatus('Call connected');
    } catch (err: any) {
      setCallStatus(`Error: ${err.message}`);
    }
  };

  const handleHangup = async () => {
    if (!activeCall) return;
    try {
      await fetch(`/api/voice/call/${activeCall.uuid}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'hangup' }),
      });
      setActiveCall(null);
      setCallStatus('Call ended');
    } catch (err: any) {
      setCallStatus(`Error: ${err.message}`);
    }
  };

  const handleMute = async () => {
    if (!activeCall) return;
    const action = isMuted ? 'unmute' : 'mute';
    await fetch(`/api/voice/call/${activeCall.uuid}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ action }),
    });
    setIsMuted(!isMuted);
  };

  const handleSendDTMF = async () => {
    if (!activeCall || !dtmf) return;
    await fetch(`/api/voice/dtmf/${activeCall.uuid}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ digits: dtmf }),
    });
    setDtmf('');
  };

  const handlePlayTTS = async () => {
    if (!activeCall || !callText) return;
    await fetch(`/api/voice/talk/${activeCall.uuid}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text: callText, language: 'en-US', voiceName: 'Amy' }),
    });
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold text-gray-900 mb-8">Voice Calls</h1>

      {/* Dialer */}
      <div className="card p-8 mb-6">
        <h2 className="text-xl font-semibold mb-4">Make a Call</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Phone Number</label>
            <input
              type="tel"
              placeholder="+1 234 567 8901"
              value={phoneNumber}
              onChange={(e) => setPhoneNumber(e.target.value)}
              className="input"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">TTS Message</label>
            <textarea
              placeholder="Enter text to speak..."
              value={callText}
              onChange={(e) => setCallText(e.target.value)}
              className="input min-h-[100px]"
            />
          </div>
          <div className="flex gap-3">
            <button onClick={() => handleCall('simple')} className="btn-primary flex-1" disabled={!!activeCall}>
              <Phone size={18} className="inline mr-2" />
              Call
            </button>
            <button onClick={() => handleCall('ivr')} className="btn-secondary flex-1" disabled={!!activeCall}>
              IVR Call
            </button>
            <button onClick={() => handleCall('conference')} className="btn-secondary flex-1" disabled={!!activeCall}>
              Conference
            </button>
          </div>
        </div>
      </div>

      {/* Active Call Controls */}
      {activeCall && (
        <div className="card p-8 mb-6">
          <h2 className="text-xl font-semibold mb-4">Active Call</h2>
          <p className="text-sm text-gray-500 mb-4">Call ID: {activeCall.uuid}</p>
          {callStatus && <p className="text-sm text-teal-600 mb-4">{callStatus}</p>}

          <div className="flex gap-3 mb-4">
            <button
              onClick={handleMute}
              className={`btn-icon ${isMuted ? 'bg-red-100 text-red-500' : 'text-gray-600'}`}
            >
              {isMuted ? <MicOff size={22} /> : <Mic size={22} />}
            </button>
            <button onClick={handlePlayTTS} className="btn-secondary">
              Play TTS
            </button>
            <button onClick={handleHangup} className="btn-danger flex-1">
              <PhoneOff size={18} className="inline mr-2" />
              Hang Up
            </button>
          </div>

          {/* DTMF Keypad */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Send DTMF</label>
            <div className="grid grid-cols-4 gap-2">
              {['1', '2', '3', '4', '5', '6', '7', '8', '9', '*', '0', '#'].map((digit) => (
                <button
                  key={digit}
                  onClick={() => setDtmf((prev) => prev + digit)}
                  className="p-3 bg-beige-100 hover:bg-beige-200 rounded-lg font-mono text-lg"
                >
                  {digit}
                </button>
              ))}
            </div>
            {dtmf && (
              <div className="mt-2 flex gap-2">
                <input value={dtmf} readOnly className="input flex-1 font-mono" />
                <button onClick={handleSendDTMF} className="btn-primary">Send</button>
                <button onClick={() => setDtmf('')} className="btn-secondary">Clear</button>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Call Status */}
      {callStatus && !activeCall && (
        <div className="card p-4 text-center text-sm text-gray-600">
          {callStatus}
        </div>
      )}
    </div>
  );
}
