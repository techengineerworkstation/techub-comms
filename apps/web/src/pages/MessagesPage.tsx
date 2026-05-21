import { useState } from 'react';
import { Send, MessageSquare, Image } from 'lucide-react';

export default function MessagesPage() {
  const [to, setTo] = useState('');
  const [from, setFrom] = useState('');
  const [text, setText] = useState('');
  const [channel, setChannel] = useState<'sms' | 'whatsapp' | 'mms'>('sms');
  const [status, setStatus] = useState<string | null>(null);
  const [sending, setSending] = useState(false);

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!to || !text) return;

    setSending(true);
    setStatus(null);

    try {
      const endpoint = channel === 'whatsapp' ? '/api/message/send-whatsapp' : '/api/message/send';
      const res = await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ to, from, text }),
      });
      const data = await res.json();
      setStatus(`Message sent! ID: ${data.messageId}`);
      setText('');
    } catch (err: any) {
      setStatus(`Error: ${err.message}`);
    } finally {
      setSending(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold text-gray-900 mb-8">Messages</h1>

      <div className="card p-8">
        <h2 className="text-xl font-semibold mb-6">Send a Message</h2>

        {/* Channel Selector */}
        <div className="flex gap-2 mb-6">
          {(['sms', 'whatsapp', 'mms'] as const).map((ch) => (
            <button
              key={ch}
              onClick={() => setChannel(ch)}
              className={`px-4 py-2 rounded-lg font-medium text-sm capitalize ${
                channel === ch
                  ? 'bg-teal-500 text-white'
                  : 'bg-beige-100 text-gray-700 hover:bg-beige-200'
              }`}
            >
              {ch === 'sms' ? 'SMS' : ch === 'whatsapp' ? 'WhatsApp' : 'MMS'}
            </button>
          ))}
        </div>

        <form onSubmit={handleSend} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">To</label>
            <input
              type="tel"
              placeholder="+1 234 567 8901"
              value={to}
              onChange={(e) => setTo(e.target.value)}
              className="input"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">From (optional)</label>
            <input
              type="tel"
              placeholder="Your Vonage number"
              value={from}
              onChange={(e) => setFrom(e.target.value)}
              className="input"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Message</label>
            <textarea
              placeholder="Type your message..."
              value={text}
              onChange={(e) => setText(e.target.value)}
              className="input min-h-[120px]"
              required
            />
          </div>
          <button type="submit" className="btn-primary w-full" disabled={sending}>
            <Send size={18} className="inline mr-2" />
            {sending ? 'Sending...' : `Send ${channel.toUpperCase()}`}
          </button>
        </form>

        {status && (
          <div className={`mt-4 p-4 rounded-lg text-sm ${status.startsWith('Error') ? 'bg-red-50 text-red-600' : 'bg-teal-50 text-teal-700'}`}>
            {status}
          </div>
        )}
      </div>
    </div>
  );
}
