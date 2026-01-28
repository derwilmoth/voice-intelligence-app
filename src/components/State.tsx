"use client";

import React, { useEffect, useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useAppStore } from "@/lib/store";
import { Mic, Brain, CheckCircle2 } from "lucide-react";
import { listen } from "@tauri-apps/api/event";
import { toast } from "sonner";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

export function State() {
  const { status, fetchHistory, fetchStatus, triggerAction, stopPipeline } =
    useAppStore();
  const [statusMessage, setStatusMessage] = React.useState<string>("");
  const [showStopDialog, setShowStopDialog] = useState(false);

  const handleStopProcessing = async () => {
    await stopPipeline();
    // Immediately fetch status from JSON to reflect the stop
    await fetchStatus();
    setShowStopDialog(false);
  };

  useEffect(() => {
    // Fetch initial status from JSON
    fetchStatus();
    fetchHistory();

    let unlistenStatus: (() => void) | undefined;
    let unlistenError: (() => void) | undefined;
    let unlistenPipelineStatus: (() => void) | undefined;
    let unlistenPipelineComplete: (() => void) | undefined;
    let unlistenRecordingTimeout: (() => void) | undefined;

    async function setupListeners() {
      unlistenStatus = await listen("status-changed", (event) => {
        // Fetch status from JSON instead of using event payload directly
        fetchStatus();
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
        // Fetch status from JSON instead of setting directly
        fetchStatus();
      });
    }
    setupListeners();

    // Refetch status when window gains focus
    const handleFocus = () => {
      fetchStatus();
    };
    window.addEventListener("focus", handleFocus);

    return () => {
      if (unlistenStatus) unlistenStatus();
      if (unlistenError) unlistenError();
      if (unlistenPipelineComplete) unlistenPipelineComplete();
      if (unlistenRecordingTimeout) unlistenRecordingTimeout();
      window.removeEventListener("focus", handleFocus);
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
            <Button
              onClick={() => setShowStopDialog(true)}
              size="lg"
              variant="destructive"
            >
              Cancel Session
            </Button>
          )}
        </CardContent>
      </Card>

      <AlertDialog open={showStopDialog} onOpenChange={setShowStopDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This will cancel the current transcription and enrichment process.
              Any progress will be lost.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Continue</AlertDialogCancel>
            <AlertDialogAction onClick={handleStopProcessing}>
              Stop
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
