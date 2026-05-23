import { useState, useEffect, useRef } from 'react';
import { Send } from 'lucide-react';

interface ChatMessage {
  id: string;
  sender: string;
  text: string;
  timestamp: Date;
  isOwn: boolean;
}

interface ChatPanelProps {
  session: any;
}

export default function ChatPanel({ session }: ChatPanelProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  useEffect(() => {
    if (!session) return;

    const handleSignal = (event: any) => {
      if (event.from?.connectionId === session.connection?.connectionId) return;

      try {
        const data = JSON.parse(event.data || '{}');
        setMessages((prev) => [
          ...prev,
          {
            id: `${Date.now()}-${Math.random()}`,
            sender: data.sender || 'Participant',
            text: data.text,
            timestamp: new Date(),
            isOwn: false,
          },
        ]);
      } catch {
        // ignore malformed signals
      }
    };

    session.on('signal:chat', handleSignal);
    return () => {
      session.off('signal:chat', handleSignal);
    };
  }, [session]);

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || !session) return;

    const text = input.trim();
    const sender = 'You';

    session.signal(
      {
        type: 'chat',
        data: JSON.stringify({ text, sender }),
      },
      (error: any) => {
        if (error) {
          console.error('Signal error:', error);
        }
      }
    );

    setMessages((prev) => [
      ...prev,
      {
        id: `${Date.now()}-${Math.random()}`,
        sender,
        text,
        timestamp: new Date(),
        isOwn: true,
      },
    ]);

    setInput('');
  };

  return (
    <div className="h-full flex flex-col">
      <div className="p-4 border-b border-beige-100">
        <h3 className="font-semibold text-gray-900">Chat</h3>
      </div>
      <div className="flex-1 overflow-auto p-4 space-y-4">
        {messages.length === 0 ? (
          <p className="text-sm text-gray-400 text-center mt-8">No messages yet</p>
        ) : (
          messages.map((msg) => (
            <div key={msg.id} className={`flex flex-col ${msg.isOwn ? 'items-end' : 'items-start'}`}>
              <div
                className={`max-w-[80%] px-4 py-2 rounded-lg ${
                  msg.isOwn ? 'bg-teal-500 text-white' : 'bg-beige-100 text-gray-900'
                }`}
              >
                {!msg.isOwn && <p className="text-xs font-medium mb-1">{msg.sender}</p>}
                <p className="text-sm">{msg.text}</p>
              </div>
              <p className="text-xs text-gray-400 mt-1">
                {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              </p>
            </div>
          ))
        )}
        <div ref={messagesEndRef} />
      </div>
      <form onSubmit={handleSend} className="p-4 border-t border-beige-100">
        <div className="flex gap-2">
          <input
            type="text"
            placeholder="Type a message..."
            value={input}
            onChange={(e) => setInput(e.target.value)}
            className="input flex-1"
          />
          <button type="submit" className="btn-primary p-2">
            <Send size={20} />
          </button>
        </div>
      </form>
    </div>
  );
}
