"use client";

import React, { useEffect, useState } from "react";
import { useAppStore } from "@/lib/store";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { RefreshCw, Save, FileText } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";

export function Settings() {
  const {
    settings,
    models,
    microphones,
    fetchSettings,
    saveSettings,
    fetchModels,
    fetchMicrophones,
  } = useAppStore();
  const [localSettings, setLocalSettings] = useState(settings);
  const [appInfo, setAppInfo] = useState<any>(null);

  useEffect(() => {
    fetchSettings();
    fetchModels();
    fetchMicrophones();

    // Fetch app info for diagnostics
    invoke("get_app_info")
      .then((info) => setAppInfo(info))
      .catch(console.error);
  }, []);

  useEffect(() => {
    setLocalSettings(settings);
  }, [settings]);

  const handleSave = async () => {
    await saveSettings(localSettings);
  };

  const hasChanges =
    localSettings.model !== settings.model ||
    localSettings.microphone !== settings.microphone ||
    localSettings.hotkey !== settings.hotkey ||
    localSettings.recording_timeout_minutes !==
      settings.recording_timeout_minutes;

  return (
    <div className="flex flex-col h-full">
      <div className="flex justify-between items-center px-4 pt-4 pb-3 shrink-0">
        <h2 className="text-xl font-bold h-8">Settings</h2>
        {hasChanges && (
          <Button size="sm" onClick={handleSave}>
            <Save className="mr-2 h-3 w-3" /> Save
          </Button>
        )}
      </div>

      <div className="flex-1 overflow-hidden">
        <ScrollArea className="h-full">
          <div className="px-4 pb-4 space-y-4">
            <Card>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="model">Ollama Enrichment Model</Label>
                  <div className="flex gap-2">
                    <Select
                      value={localSettings.model}
                      onValueChange={(val) =>
                        setLocalSettings({ ...localSettings, model: val })
                      }
                    >
                      <SelectTrigger id="model" className="w-69">
                        <SelectValue placeholder="Select model" />
                      </SelectTrigger>
                      <SelectContent className="w-69">
                        {models.length === 0 ? (
                          <SelectItem value="none" disabled>
                            No models found
                          </SelectItem>
                        ) : (
                          models.map((m) => (
                            <SelectItem key={m} value={m}>
                              {m}
                            </SelectItem>
                          ))
                        )}
                      </SelectContent>
                    </Select>
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={() => fetchModels()}
                      title="Refresh Models"
                    >
                      <RefreshCw className="h-4 w-4" />
                    </Button>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    Models detected in your .ollama directory
                  </p>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="mic">Microphone</Label>
                  <div className="flex gap-2">
                    <Select
                      value={localSettings.microphone}
                      onValueChange={(val) =>
                        setLocalSettings({ ...localSettings, microphone: val })
                      }
                    >
                      <SelectTrigger id="mic" className="w-69">
                        <SelectValue placeholder="Select microphone" />
                      </SelectTrigger>
                      <SelectContent className="w-69">
                        <SelectItem value="default">
                          Default System Device
                        </SelectItem>
                        {microphones.map((mic) => (
                          <SelectItem key={mic} value={mic}>
                            {mic}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={() => fetchMicrophones()}
                      title="Refresh Mics"
                    >
                      <RefreshCw className="h-4 w-4" />
                    </Button>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="hotkey">Global Hotkey</Label>
                  <Input
                    id="hotkey"
                    value={localSettings.hotkey}
                    onChange={(e) =>
                      setLocalSettings({
                        ...localSettings,
                        hotkey: e.target.value,
                      })
                    }
                  />
                  <p className="text-xs text-muted-foreground">
                    Format: Modifiers+Key (e.g., Ctrl+I, Alt+Space)
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="timeout">Recording Timeout (minutes)</Label>
                  <Input
                    id="timeout"
                    type="number"
                    min="1"
                    max="60"
                    value={localSettings.recording_timeout_minutes}
                    onChange={(e) =>
                      setLocalSettings({
                        ...localSettings,
                        recording_timeout_minutes:
                          parseInt(e.target.value) || 10,
                      })
                    }
                  />
                  <p className="text-xs text-muted-foreground">
                    Maximum duration before auto-timeout (1-60 minutes)
                  </p>
                </div>
              </CardContent>
            </Card>
          </div>
        </ScrollArea>
      </div>
    </div>
  );
}
