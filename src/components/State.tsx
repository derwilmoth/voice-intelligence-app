"use client";

import React, { useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { useAppStore } from "@/lib/store";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Activity,
  Mic,
  Brain,
  CheckCircle2,
  Loader2,
  AudioLines,
} from "lucide-react";
import { listen } from "@tauri-apps/api/event";
import { toast } from "sonner";

export function State() {
  const {
    status,
    history,
    fetchHistory,
    setStatus,
    triggerAction,
    stopPipeline,
  } = useAppStore();
  const [statusMessage, setStatusMessage] = React.useState<string>("");

  useEffect(() => {
    fetchHistory();

    let unlistenStatus: (() => void) | undefined;
    let unlistenError: (() => void) | undefined;
    let unlistenPipelineStatus: (() => void) | undefined;
    let unlistenPipelineComplete: (() => void) | undefined;
    let unlistenRecordingTimeout: (() => void) | undefined;

    async function setupListeners() {
      unlistenStatus = await listen("status-changed", (event) => {
        setStatus(event.payload as any);
        setStatusMessage("");
      });

      unlistenError = await listen("pipeline-error", (event) => {
        console.error("Pipeline error:", event.payload);
        toast.error("Pipeline Error", {
          description: event.payload as string,
          duration: 5000,
        });
        setStatusMessage("");
      });

      unlistenPipelineComplete = await listen("pipeline-complete", (event) => {
        console.log("Pipeline complete:", event.payload);
        toast.success("Success!", {
          description: "Content has been enriched and copied to clipboard",
          duration: 3000,
        });
        fetchHistory();
        setStatusMessage("");
      });

      unlistenRecordingTimeout = await listen("recording-timeout", (event) => {
        console.warn("Recording timeout:", event.payload);
        toast.error("Recording Timeout", {
          description: event.payload as string,
          duration: 5000,
        });
        setStatus("idle");
      });
    }
    setupListeners();

    return () => {
      if (unlistenStatus) unlistenStatus();
      if (unlistenError) unlistenError();
      if (unlistenPipelineComplete) unlistenPipelineComplete();
      if (unlistenRecordingTimeout) unlistenRecordingTimeout();
    };
  }, []);

  const getStatusColor = (s: string) => {
    switch (s) {
      case "idle":
        return "bg-secondary";
      case "instruction":
        return "bg-secondary animate-pulse";
      case "content":
        return "bg-secondary animate-pulse";
      case "processing":
        return "bg-secondary animate-pulse";
      case "success":
        return "bg-secondary";
      default:
        return "bg-secondary";
    }
  };

  const getStatusIcon = (s: string) => {
    switch (s) {
      case "instruction":
        return <Mic className="w-8 h-8 text-secondary-foreground" />;
      case "content":
        return <Mic className="w-8 h-8 text-secondary-foreground" />;
      case "processing":
        return <Brain className="w-8 h-8 text-secondary-foreground" />;
      case "success":
        return <CheckCircle2 className="w-8 h-8 text-secondary-foreground" />;
      default:
        return <Mic className="w-8 h-8 text-secondary-foreground" />;
    }
  };

  const getStatusText = (s: string) => {
    switch (s) {
      case "idle":
        return "Ready";
      case "instruction":
        return "Listening for Instruction";
      case "content":
        return "Listening for Content";
      case "processing":
        return "Processing";
      case "success":
        return "Done!";
      default:
        return "Ready";
    }
  };

  const recentHistory = history.slice(-3).reverse();

  return (
    <div className="space-y-4 p-4">
      {/* Status Indicator */}
      <Card className="border-none shadow-md bg-secondary/20">
        <CardContent className="flex flex-col items-center justify-center py-6 space-y-3">
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-300 ${getStatusColor(status)}`}
          >
            {getStatusIcon(status)}
          </div>
          <h2 className="text-xl font-semibold tracking-tight">
            {getStatusText(status)}
          </h2>
          {statusMessage && (
            <p className="text-sm font-medium text-foreground">
              {statusMessage}
            </p>
          )}
          <p className="text-sm text-muted-foreground text-center mb-4">
            {status === "idle"
              ? "Press global hotkey or click below to start"
              : status === "instruction"
                ? "Talk about what I’m supposed to contribute"
                : status === "content"
                  ? "Tell me about what I’m supposed to enrich"
                  : status === "processing" &&
                    "The result will be copied to your clipboard"}
          </p>
          {(status === "idle" ||
            status == "instruction" ||
            status == "content") && (
            <Button onClick={triggerAction} size="lg" variant="default">
              {status === "idle"
                ? "Record Instruction"
                : status === "instruction"
                  ? "Record Content"
                  : "Continue Processing"}
            </Button>
          )}
          {status === "processing" && (
            <Button onClick={stopPipeline} size="lg" variant="destructive">
              Stop Processing
            </Button>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
