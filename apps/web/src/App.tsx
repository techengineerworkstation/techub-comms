import { Routes, Route } from 'react-router-dom';
import Layout from './components/ui/Layout';
import LandingPage from './pages/LandingPage';
import MeetingRoom from './pages/MeetingRoom';
import VoicePage from './pages/VoicePage';
import MessagesPage from './pages/MessagesPage';
import RecordingsPage from './pages/RecordingsPage';

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<LandingPage />} />
        <Route path="meeting/:room" element={<MeetingRoom />} />
        <Route path="voice" element={<VoicePage />} />
        <Route path="messages" element={<MessagesPage />} />
        <Route path="recordings" element={<RecordingsPage />} />
      </Route>
    </Routes>
  );
}
