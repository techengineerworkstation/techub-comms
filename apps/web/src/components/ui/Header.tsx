import { Bell, Settings } from 'lucide-react';

export default function Header() {
  return (
    <header className="h-16 bg-white border-b border-beige-100 flex items-center justify-between px-6">
      <div>
        <h2 className="text-lg font-semibold text-gray-900">Welcome to Techub Comms</h2>
      </div>
      <div className="flex items-center gap-4">
        <button className="btn-icon text-gray-500 hover:text-gray-700">
          <Bell size={20} />
        </button>
        <button className="btn-icon text-gray-500 hover:text-gray-700">
          <Settings size={20} />
        </button>
      </div>
    </header>
  );
}
