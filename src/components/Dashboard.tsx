"use client";

import React, { useEffect } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useAppStore } from "@/lib/store";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Activity, Mic, Brain, CheckCircle2 } from "lucide-react";

export function Dashboard() {
  const { status, history, fetchHistory } = useAppStore();

  useEffect(() => {
    fetchHistory();
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
          <p className="text-sm text-muted-foreground text-center">
            {status === "idle"
              ? "Press global hotkey to start"
              : "Processing your voice command"}
          </p>
        </CardContent>
      </Card>

      {/* Recent Activity */}
      <div className="space-y-2">
        <h3 className="text-sm font-medium text-muted-foreground px-1">
          Recent Activity
        </h3>
        <ScrollArea className="h-[200px] rounded-md border p-2 bg-background">
          {recentHistory.length === 0 ? (
            <div className="flex items-center justify-center h-full text-sm text-muted-foreground">
              No history yet.
            </div>
          ) : (
            <div className="space-y-3">
              {recentHistory.map((item) => (
                <Card key={item.id} className="p-3">
                  <div className="flex justify-between items-start mb-2">
                    <Badge
                      variant="outline"
                      className="text-xs bg-slate-100 dark:bg-slate-800"
                    >
                      {item.instruction}
                    </Badge>
                    <span className="text-[10px] text-muted-foreground">
                      {new Date(item.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                  <p className="text-sm text-foreground line-clamp-2">
                    {item.enriched_content}
                  </p>
                </Card>
              ))}
            </div>
          )}
        </ScrollArea>
      </div>
    </div>
  );
}
