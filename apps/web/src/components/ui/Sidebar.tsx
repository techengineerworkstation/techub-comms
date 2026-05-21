import { Link, useLocation } from 'react-router-dom';
import { Video, Phone, MessageSquare, FolderOpen, Home } from 'lucide-react';

const navItems = [
  { path: '/', label: 'Home', icon: Home },
  { path: '/voice', label: 'Voice Calls', icon: Phone },
  { path: '/messages', label: 'Messages', icon: MessageSquare },
  { path: '/recordings', label: 'Recordings', icon: FolderOpen },
];

export default function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-64 bg-white border-r border-beige-100 flex flex-col">
      <div className="p-6 border-b border-beige-100">
        <h1 className="text-2xl font-bold text-teal-600">Techub</h1>
        <p className="text-sm text-gray-500">Comms App</p>
      </div>
      <nav className="flex-1 p-4 space-y-1">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.path;
          return (
            <Link
              key={item.path}
              to={item.path}
              className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors duration-200 ${
                isActive
                  ? 'bg-teal-50 text-teal-700 font-medium'
                  : 'text-gray-600 hover:bg-beige-50 hover:text-gray-900'
              }`}
            >
              <Icon size={20} />
              <span>{item.label}</span>
            </Link>
          );
        })}
      </nav>
      <div className="p-4 border-t border-beige-100">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-teal-500 rounded-full flex items-center justify-center text-white text-sm font-medium">
            U
          </div>
          <div>
            <p className="text-sm font-medium text-gray-900">User</p>
            <p className="text-xs text-gray-500">Online</p>
          </div>
        </div>
      </div>
    </aside>
  );
}
