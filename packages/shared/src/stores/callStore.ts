import { create } from 'zustand';
import type { VoiceCall } from '../types';

interface CallState {
  activeCall: VoiceCall | null;
  callHistory: VoiceCall[];
  isInCall: boolean;
  isMuted: boolean;
  isOnHold: boolean;

  setActiveCall: (call: VoiceCall | null) => void;
  addToHistory: (call: VoiceCall) => void;
  setInCall: (inCall: boolean) => void;
  setMuted: (muted: boolean) => void;
  setOnHold: (onHold: boolean) => void;
  endCall: () => void;
}

export const useCallStore = create<CallState>((set) => ({
  activeCall: null,
  callHistory: [],
  isInCall: false,
  isMuted: false,
  isOnHold: false,

  setActiveCall: (call) => set({ activeCall: call, isInCall: !!call }),
  addToHistory: (call) => set((state) => ({ callHistory: [call, ...state.callHistory] })),
  setInCall: (inCall) => set({ isInCall: inCall }),
  setMuted: (muted) => set({ isMuted: muted }),
  setOnHold: (onHold) => set({ isOnHold: onHold }),
  endCall: () => set((state) => ({
    activeCall: null,
    isInCall: false,
    isMuted: false,
    isOnHold: false,
    callHistory: state.activeCall ? [state.activeCall, ...state.callHistory] : state.callHistory,
  })),
}));
