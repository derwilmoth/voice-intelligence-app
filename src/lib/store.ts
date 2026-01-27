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
    
    // Actions
    fetchSettings: () => Promise<void>;
    saveSettings: (settings: Settings) => Promise<void>;
    fetchHistory: () => Promise<void>;
    clearHistory: () => Promise<void>;
    fetchModels: () => Promise<void>;
    fetchMicrophones: () => Promise<void>;
    setStatus: (status: AppState['status']) => void;
}

export const useAppStore = create<AppState>((set, get) => ({
    settings: {
        model: 'gemma:4b',
        microphone: 'default',
        hotkey: 'CommandOrControl+Shift+Space'
    },
    history: [],
    models: [],
    microphones: [],
    status: 'idle',

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

    setStatus: (status) => set({ status })
}));
