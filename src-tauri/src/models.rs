use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub model: String,
    pub microphone: String,
    pub hotkey: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model: "gemma3:4b".to_string(), // Default as per requirements
            microphone: "default".to_string(),
            hotkey: "CommandOrControl+Shift+Space".to_string(), // Example default
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub id: String,
    pub timestamp: String,
    pub instruction: String,
    pub original_content: String,
    pub enriched_content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppStateData {
    pub settings: Settings,
    pub history: Vec<HistoryItem>,
}
