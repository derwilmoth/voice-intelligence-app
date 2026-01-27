import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface Settings {
    model: string;
    microphone: string;
    hotkey: string;
}

export interface HistoryItem {
    id: string;
    timestamp: string;
    instruction: string;
    original_content: string;
    enriched_content: string;
}

interface AppState {
    settings: Settings;
    history: HistoryItem[];
    models: string[];
    microphones: string[];
    status: 'idle' | 'instruction' | 'content' | 'processing' | 'success';
    error: string | null;
    
    // Actions
    fetchSettings: () => Promise<void>;
    saveSettings: (settings: Settings) => Promise<void>;
    fetchHistory: () => Promise<void>;
    clearHistory: () => Promise<void>;
    deleteHistoryItem: (id: string) => Promise<void>;
    fetchModels: () => Promise<void>;
    fetchMicrophones: () => Promise<void>;
    setStatus: (status: AppState['status']) => void;
    setError: (error: string | null) => void;
    triggerAction: () => Promise<void>;
}

export const useAppStore = create<AppState>((set, get) => ({
    settings: {
        model: 'gemma:4b',
        microphone: 'default',
        hotkey: 'Ctrl+I'
    },
    history: [],
    models: [],
    microphones: [],
    status: 'idle',
    error: null,

    fetchSettings: async () => {
        try {
            const settings = await invoke<Settings>('get_settings');
            set({ settings });
        } catch (error) {
            console.error('Failed to fetch settings:', error);
        }
    },

    saveSettings: async (newSettings) => {
        try {
            await invoke('save_settings', { settings: newSettings });
            set({ settings: newSettings });
        } catch (error) {
            console.error('Failed to save settings:', error);
        }
    },

    fetchHistory: async () => {
        try {
            const history = await invoke<HistoryItem[]>('get_history');
            set({ history });
        } catch (error) {
            console.error('Failed to fetch history:', error);
        }
    },

    clearHistory: async () => {
        try {
            await invoke('clear_history');
            set({ history: [] });
        } catch (error) {
            console.error('Failed to clear history:', error);
        }
    },

    deleteHistoryItem: async (id: string) => {
        try {
            await invoke('delete_history_item', { id });
            const history = get().history.filter(item => item.id !== id);
            set({ history });
        } catch (error) {
            console.error('Failed to delete history item:', error);
        }
    },

    fetchModels: async () => {
        try {
            const models = await invoke<string[]>('get_models');
            set({ models });
        } catch (error) {
            console.error('Failed to fetch models:', error);
        }
    },

    fetchMicrophones: async () => {
        try {
            const microphones = await invoke<string[]>('get_input_devices');
            set({ microphones });
        } catch (error) {
            console.error('Failed to fetch microphones:', error);
        }
    },

    setStatus: (status) => set({ status }),

    setError: (error) => set({ error }),

    triggerAction: async () => {
        try {
            await invoke('manual_trigger');
        } catch (error) {
            console.error('Failed to trigger action:', error);
        }
    },
}));
