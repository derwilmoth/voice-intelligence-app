# Voice Intelligence App

A desktop application that uses voice input to enrich content with AI. Built with Tauri v2, Next.js, and Rust.

## Features

- **Voice Recording**: Capture audio instructions and content using your microphone
- **AI Enrichment**: Uses local Whisper for transcription and Ollama for content enhancement
- **Global Hotkey**: Control recording with customizable keyboard shortcut
- **History Management**: View and manage past enrichment sessions
- **Configurable Settings**: Customize AI models, microphone input, and recording timeout
- **Toast Notifications**: Get real-time feedback on operations/errors
- **Stop Processing**: Cancel ongoing transcription/enrichment at any time
- **Recording Timeout**: Automatic timeout protection (configurable, default 10 minutes)

## How It Works

1. **Start Recording** - Press the global hotkey or click "Record Instruction"
2. **Speak Your Instruction** - Describe what you want to do with the content
3. **Record Content** - Press hotkey or button again to record the content to be enriched
4. **Speak Your Content** - Provide the text you want to enhance
5. **Processing** - Whisper transcribes audio, Ollama enriches based on your instruction
6. **Done** - Enriched content is automatically copied to your clipboard

## Prerequisite for Use

- **Ollama** - Running locally with at least one model installed
  - Install from [ollama.com](https://ollama.com)
  - Pull a model: `ollama pull gemma3:4b` (or your preferred model)

### Installation

- Download the latest portable release for your architectur and operating system
- You should be able to execute the portable out of the box
- Not available/tested for many architectures/operating systems

## German Demo
[![Watch the video](https://img.youtube.com/vi/1UWuSE-b7DE/maxresdefault.jpg)](https://youtu.be/1UWuSE-b7DE)

## Prerequisites for Build

- **Node.js** (v18 or later)
- **Rust** (latest stable)

### Getting Started

```bash
# Clone the repository
git clone <your-repo-url>
cd voice-intelligence-app

# Install frontend dependencies
npm install

# Build the frontend
npm run build

# Run in development mode
npx tauri dev

# Build for production
npx tauri build
```

## Configuration

### Settings Panel

Access settings via the Settings tab to configure:

- **Ollama Model**: Select from locally installed models
- **Microphone**: Choose your preferred input device
- **Global Hotkey**: Customize the keyboard shortcut (default: Ctrl+I)
- **Recording Timeout**: Set maximum recording duration (1-60 minutes, default: 10)

### Default Settings

```json
{
  "model": "gemma3:4b",
  "microphone": "default",
  "hotkey": "Ctrl+I",
  "recording_timeout_minutes": 10
}
```

## Technical Details

### Backend (Rust)

- **Tauri v2.9.5**: Cross-platform desktop runtime
- **whisper-rs v0.15.1**: Local speech-to-text with automatic language detection
- **cpal**: Cross-platform audio I/O
- **reqwest**: HTTP client for Ollama API
- **Custom build profile**: `opt-level=0` to prevent Whisper FFI issues

### Frontend (TypeScript/React)

- **Next.js 14+**: React framework with static export
- **shadcn/ui**: Component library (Tailwind CSS)
- **Zustand**: State management
- **Sonner**: Toast notifications
- **Event-driven**: Backend communicates via Tauri events

### Key Technologies

- **Whisper Large Turbo**: Auto-downloads on first run (~1GB)
- **Ollama**: Local LLM for content enrichment
- **16kHz Audio**: Optimized for Whisper compatibility
- **Event Architecture**: `status-changed`, `pipeline-complete`, `pipeline-error`, `recording-timeout`

## Troubleshooting

### Whisper Model Download

On first run, the app downloads the Whisper large-v3-turbo-q8_0 model (~1GB). This may take several minutes depending on your internet connection. The model is cached in your app data directory.

### Ollama Connection

Ensure Ollama is running on `http://localhost:11434`. If you see connection errors:

- Start Ollama: `ollama serve`
- Verify models are installed: `ollama list`
- Pull a model if needed: `ollama pull gemma3:4b`

### Audio Issues

If microphone isn't working:

- Check Settings > Microphone to select the correct device
- Grant microphone permissions to the app
- Test with the "Play Test Sound" button in Settings

### Recording Timeout

If recordings are being cut off:

- Increase the timeout in Settings > Recording Timeout
- Default is 10 minutes, max is 60 minutes
