import { create } from 'zustand';
import type { Participant, Archive } from '../types';

interface SessionState {
  sessionId: string | null;
  token: string | null;
  apiKey: string | null;
  roomName: string | null;
  isConnected: boolean;
  participants: Participant[];
  archives: Archive[];
  captionsId: string | null;
  isRecording: boolean;
  isScreenSharing: boolean;
  isChatOpen: boolean;
  isParticipantsOpen: boolean;

  setSession: (sessionId: string, token: string, apiKey: string, roomName: string) => void;
  clearSession: () => void;
  setConnected: (connected: boolean) => void;
  addParticipant: (participant: Participant) => void;
  removeParticipant: (id: string) => void;
  updateParticipant: (id: string, updates: Partial<Participant>) => void;
  setArchives: (archives: Archive[]) => void;
  setRecording: (recording: boolean) => void;
  setScreenSharing: (sharing: boolean) => void;
  setCaptionsId: (id: string | null) => void;
  toggleChat: () => void;
  toggleParticipants: () => void;
}

export const useSessionStore = create<SessionState>((set) => ({
  sessionId: null,
  token: null,
  apiKey: null,
  roomName: null,
  isConnected: false,
  participants: [],
  archives: [],
  captionsId: null,
  isRecording: false,
  isScreenSharing: false,
  isChatOpen: false,
  isParticipantsOpen: false,

  setSession: (sessionId, token, apiKey, roomName) =>
    set({ sessionId, token, apiKey, roomName }),
  clearSession: () =>
    set({ sessionId: null, token: null, apiKey: null, roomName: null, isConnected: false, participants: [] }),
  setConnected: (connected) => set({ isConnected: connected }),
  addParticipant: (participant) =>
    set((state) => ({ participants: [...state.participants, participant] })),
  removeParticipant: (id) =>
    set((state) => ({ participants: state.participants.filter((p) => p.id !== id) })),
  updateParticipant: (id, updates) =>
    set((state) => ({
      participants: state.participants.map((p) => (p.id === id ? { ...p, ...updates } : p)),
    })),
  setArchives: (archives) => set({ archives }),
  setRecording: (recording) => set({ isRecording: recording }),
  setScreenSharing: (sharing) => set({ isScreenSharing: sharing }),
  setCaptionsId: (id) => set({ captionsId: id }),
  toggleChat: () => set((state) => ({ isChatOpen: !state.isChatOpen, isParticipantsOpen: false })),
  toggleParticipants: () => set((state) => ({ isParticipantsOpen: !state.isParticipantsOpen, isChatOpen: false })),
}));
