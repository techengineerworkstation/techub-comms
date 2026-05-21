import { create } from 'zustand';
import type { Message } from '../types';

interface MessageState {
  messages: Message[];
  conversations: Map<string, Message[]>;
  activeConversation: string | null;
  isComposing: boolean;

  addMessage: (message: Message) => void;
  setActiveConversation: (contact: string | null) => void;
  setComposing: (composing: boolean) => void;
  getConversation: (contact: string) => Message[];
}

export const useMessageStore = create<MessageState>((set, get) => ({
  messages: [],
  conversations: new Map(),
  activeConversation: null,
  isComposing: false,

  addMessage: (message) => {
    set((state) => {
      const contact = message.direction === 'outbound' ? message.to : message.from;
      const existing = state.conversations.get(contact) || [];
      const updated = new Map(state.conversations);
      updated.set(contact, [...existing, message]);
      return { messages: [...state.messages, message], conversations: updated };
    });
  },
  setActiveConversation: (contact) => set({ activeConversation: contact }),
  setComposing: (composing) => set({ isComposing: composing }),
  getConversation: (contact) => {
    return get().conversations.get(contact) || [];
  },
}));
