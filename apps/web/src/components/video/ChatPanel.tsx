import { useState } from 'react';
import { Send } from 'lucide-react';

interface ChatMessage {
  id: string;
  sender: string;
  text: string;
  timestamp: Date;
}

export default function ChatPanel() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;

    const newMessage: ChatMessage = {
      id: Date.now().toString(),
      sender: 'You',
      text: input.trim(),
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, newMessage]);
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
            <div key={msg.id} className={`flex flex-col ${msg.sender === 'You' ? 'items-end' : 'items-start'}`}>
              <div
                className={`max-w-[80%] px-4 py-2 rounded-lg ${
                  msg.sender === 'You' ? 'bg-teal-500 text-white' : 'bg-beige-100 text-gray-900'
                }`}
              >
                {msg.sender !== 'You' && <p className="text-xs font-medium mb-1">{msg.sender}</p>}
                <p className="text-sm">{msg.text}</p>
              </div>
              <p className="text-xs text-gray-400 mt-1">
                {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              </p>
            </div>
          ))
        )}
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
