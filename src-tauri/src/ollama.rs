use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;
use dirs;

#[cfg(target_os = "windows")]
fn get_ollama_models_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".ollama").join("models"))
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_ollama_models_dir() -> Option<PathBuf> {
    // Check standard install location first for Linux
    #[cfg(target_os = "linux")]
    {
        let usr_share = PathBuf::from("/usr/share/ollama/.ollama/models");
        if usr_share.exists() {
            return Some(usr_share);
        }
    }
    
    // Fallback to user home for Mac/Linux user installs
    dirs::home_dir().map(|home| home.join(".ollama").join("models"))
}

pub fn scan_models() -> Vec<String> {
    let mut models = Vec::new();
    
    let base_path = match get_ollama_models_dir() {
        Some(p) => p,
        None => return models,
    };

    // The structure is usually .ollama/models/manifests/registry.ollama.ai/library/<model_name>/<tag>
    // Or sometimes just .ollama/models/manifests/registry.ollama.ai/<namespace>/<model_name>/<tag>
    // We will walk the 'manifests' directory.
    let manifests_path = base_path.join("manifests");

    if !manifests_path.exists() {
        return models;
    }

    // Walks recursively. We want to find files that act as tags.
    // Typical path: manifests/registry.ollama.ai/library/llama2/latest
    // We want to extract "llama2:latest"
    
    for entry in WalkDir::new(&manifests_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            
            // Try to extract relative path from manifests dir
            if let Ok(relative) = path.strip_prefix(&manifests_path) {
                // components look like ["registry.ollama.ai", "library", "llama2", "latest"]
                let components: Vec<_> = relative.components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect();

                // We expect at least 3 parts: registry, namespace, model, tag (sometimes registry is implicit or different)
                // Standard: registry.ollama.ai / library / model / tag
                if components.len() >= 3 {
                    let tag = components.last().unwrap();
                    let model = components.get(components.len() - 2).unwrap();
                    
                    // Basic heuristic: combine model and tag
                    // If namespace is not 'library', preprend it? 
                    // Let's look at the instruction: "Parse directory structure"
                    // E.g. .../library/gemma/4b -> gemma:4b
                    
                    let model_string = format!("{}:{}", model, tag);
                    models.push(model_string);
                }
            }
        }
    }

    models
}
