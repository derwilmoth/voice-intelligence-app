"use client";

import React, { useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { useAppStore } from "@/lib/store";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Activity, Mic, Brain, CheckCircle2, Loader2 } from "lucide-react";
import { listen } from "@tauri-apps/api/event";

export function Dashboard() {
  const {
    status,
    history,
    error,
    fetchHistory,
    setStatus,
    setError,
    triggerAction,
  } = useAppStore();
  const [statusMessage, setStatusMessage] = React.useState<string>("");

  useEffect(() => {
    fetchHistory();

    let unlistenStatus: () => void;
    let unlistenError: () => void;
    let unlistenPipelineStatus: () => void;
    let unlistenPipelineComplete: () => void;

    async function setupListeners() {
      unlistenStatus = await listen("status-changed", (event) => {
        setStatus(event.payload as any);
        // Clear error when status changes
        setError(null);
        setStatusMessage("");
      });

      unlistenError = await listen("pipeline-error", (event) => {
        console.error("Pipeline error:", event.payload);
        setError(event.payload as string);
        setStatusMessage("");
      });

      unlistenPipelineComplete = await listen("pipeline-complete", (event) => {
        console.log("Pipeline complete:", event.payload);
        // Don't manually set status here - backend will emit status-changed to idle
        fetchHistory(); // Refresh history to show the new item
        setStatusMessage("");
      });
    }
    setupListeners();

    return () => {
      if (unlistenStatus) unlistenStatus();
      if (unlistenError) unlistenError();
      if (unlistenPipelineComplete) unlistenPipelineComplete();
    };
  }, [fetchHistory]);

  const getStatusColor = (s: string) => {
    switch (s) {
      case "idle":
        return "bg-primary";
      case "instruction":
        return "bg-primary animate-pulse";
      case "content":
        return "bg-primary animate-pulse";
      case "processing":
        return "bg-primary animate-pulse";
      case "success":
        return "bg-primary";
      default:
        return "bg-primary";
    }
  };

  const getStatusIcon = (s: string) => {
    switch (s) {
      case "instruction":
        return <Mic className="w-8 h-8 text-primary-foreground" />;
      case "content":
        return <Mic className="w-8 h-8 text-primary-foreground" />;
      case "processing":
        return <Brain className="w-8 h-8 text-primary-foreground" />;
      case "success":
        return <CheckCircle2 className="w-8 h-8 text-primary-foreground" />;
      default:
        return <Mic className="w-8 h-8 text-primary-foreground" />;
    }
  };

  const getStatusText = (s: string) => {
    switch (s) {
      case "idle":
        return "Ready";
      case "instruction":
        return "Listening for Instruction...";
      case "content":
        return "Listening for Content...";
      case "processing":
        return "Processing...";
      case "success":
        return "Done!";
      default:
        return "Ready";
    }
  };

  const recentHistory = history.slice(-3).reverse();

  return (
    <div className="space-y-4 p-4">
      {/* Error Display */}
      {error && (
        <Card className="border-destructive bg-destructive/10">
          <CardContent className="py-4">
            <div className="flex items-start space-x-2">
              <div className="text-destructive font-semibold">Error:</div>
              <div className="text-destructive/90 text-sm flex-1">{error}</div>
            </div>
          </CardContent>
        </Card>
      )}

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
          <p className="text-sm text-muted-foreground text-center mb-2">
            {status === "idle" && "Press global hotkey or click below to start"}
          </p>
          {status === "idle" && (
            <Button
              onClick={triggerAction}
              size="lg"
              variant="default"
              className="w-full max-w-xs"
            >
              Start Recording
            </Button>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
