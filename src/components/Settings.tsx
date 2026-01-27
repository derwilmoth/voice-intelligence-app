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
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { RefreshCw, Save, FolderOpen, FileText } from "lucide-react";
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

  return (
    <div className="p-4 space-y-4">
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">AI Configuration</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="model">Ollama Model</Label>
            <div className="flex gap-2">
              <Select
                value={localSettings.model}
                onValueChange={(val) =>
                  setLocalSettings({ ...localSettings, model: val })
                }
              >
                <SelectTrigger id="model">
                  <SelectValue placeholder="Select model" />
                </SelectTrigger>
                <SelectContent>
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
              Models detected in your .ollama directory.
            </p>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Input & Control</CardTitle>
        </CardHeader>
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
                <SelectTrigger id="mic">
                  <SelectValue placeholder="Select microphone" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="default">Default System Device</SelectItem>
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
                setLocalSettings({ ...localSettings, hotkey: e.target.value })
              }
            />
            <p className="text-xs text-muted-foreground">
              Format: Modifiers+Key (e.g., Ctrl+I, Alt+Space)
            </p>
          </div>
        </CardContent>
      </Card>

      <Button className="w-full" onClick={handleSave}>
        <Save className="mr-2 h-4 w-4" /> Save Settings
      </Button>

      {/* Diagnostic Info */}
      {appInfo && (
        <Card className="border-muted">
          <CardHeader>
            <CardTitle className="text-lg">Diagnostic Information</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3 text-sm">
            <div>
              <Label className="text-xs text-muted-foreground">
                Build Mode
              </Label>
              <p className="font-mono">{appInfo.build_mode}</p>
            </div>
            {appInfo.app_data_dir && (
              <div>
                <Label className="text-xs text-muted-foreground">
                  App Data Directory
                </Label>
                <div className="flex gap-2 items-center">
                  <p className="font-mono text-xs flex-1 break-all">
                    {appInfo.app_data_dir}
                  </p>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => open(appInfo.app_data_dir)}
                    title="Open folder"
                  >
                    <FolderOpen className="h-3 w-3" />
                  </Button>
                </div>
                <p className="text-xs text-muted-foreground mt-1">
                  Contains settings.json, history.json, and audio files
                </p>
              </div>
            )}
            {appInfo.log_dir && (
              <div>
                <Label className="text-xs text-muted-foreground">
                  Log Directory
                </Label>
                <div className="flex gap-2 items-center">
                  <p className="font-mono text-xs flex-1 break-all">
                    {appInfo.log_dir}
                  </p>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => open(appInfo.log_dir)}
                    title="Open log folder"
                  >
                    <FileText className="h-3 w-3" />
                  </Button>
                </div>
                <p className="text-xs text-muted-foreground mt-1">
                  Check app.log for error messages (release mode only)
                </p>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
