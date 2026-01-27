"use client";

import React, { useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { useAppStore } from "@/lib/store";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Activity, Mic, Brain, CheckCircle2 } from "lucide-react";
import { listen } from "@tauri-apps/api/event";

export function Dashboard() {
  const { status, history, fetchHistory, setStatus, triggerAction } =
    useAppStore();

  useEffect(() => {
    fetchHistory();

    let unlisten: () => void;

    async function setupListener() {
      unlisten = await listen("status-changed", (event) => {
        setStatus(event.payload as any);
      });
    }
    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const getStatusColor = (s: string) => {
    switch (s) {
      case "idle":
        return "bg-gray-500";
      case "instruction":
        return "bg-blue-500 animate-pulse";
      case "content":
        return "bg-red-500 animate-pulse";
      case "processing":
        return "bg-yellow-500 animate-pulse";
      case "success":
        return "bg-green-500";
      default:
        return "bg-gray-500";
    }
  };

  const getStatusIcon = (s: string) => {
    switch (s) {
      case "instruction":
        return <Mic className="w-8 h-8 text-white" />;
      case "content":
        return <Mic className="w-8 h-8 text-white" />;
      case "processing":
        return <Brain className="w-8 h-8 text-white" />;
      case "success":
        return <CheckCircle2 className="w-8 h-8 text-white" />;
      default:
        return <Activity className="w-8 h-8 text-white" />;
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
          <p className="text-sm text-muted-foreground text-center mb-2">
            {status === "idle"
              ? "Press global hotkey or click below to start"
              : "Processing your voice command"}
          </p>
          <Button
            onClick={triggerAction}
            size="lg"
            variant={status === "idle" ? "default" : "destructive"}
            className="w-full max-w-xs"
          >
            {status === "idle" ? "Start Recording" : "Next Step / Stop"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
